use std::{
    marker::PhantomData,
    sync::{Arc, mpsc::channel},
};

use bytemuck::{AnyBitPattern, NoUninit};

use crate::rt::gpu::gpu::Gpu;

pub struct GpuCompute<I: NoUninit + AnyBitPattern, O: NoUninit + AnyBitPattern> {
    gpu: Arc<Gpu>,
    pipeline: wgpu::ComputePipeline,
    bind_group: wgpu::BindGroup,
    dim_buf: wgpu::Buffer,
    input_buf: wgpu::Buffer,
    output_buf: wgpu::Buffer,
    temp_buf: wgpu::Buffer,
    _i: PhantomData<I>,
    _o: PhantomData<O>,
}

impl<I: NoUninit + AnyBitPattern, O: NoUninit + AnyBitPattern> GpuCompute<I, O> {
    pub fn new(
        gpu: Arc<Gpu>,
        shader: &wgpu::ShaderModule,
        input_size: u64,
        output_size: u64,
    ) -> Self {
        let input_usages = wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE;
        let output_usages = wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE;
        let temp_usages = wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ;

        let dim_buf = Self::create_buffer(
            Arc::clone(&gpu),
            2 * std::mem::size_of::<u32>() as u64,
            input_usages,
        );
        let input_buf = Self::create_buffer(Arc::clone(&gpu), input_size, input_usages);
        let output_buf = Self::create_buffer(Arc::clone(&gpu), output_size, output_usages);
        let temp_buf = Self::create_buffer(Arc::clone(&gpu), output_size, temp_usages);

        let pipeline = gpu.create_compute_pipeline(shader);
        let bind_group_layout = pipeline.get_bind_group_layout(0);
        let bind_group_entries = [
            wgpu::BindGroupEntry {
                binding: 0,
                resource: dim_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: input_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: output_buf.as_entire_binding(),
            },
        ];
        let bind_group = gpu.create_bind_group(&bind_group_layout, &bind_group_entries);

        Self {
            gpu,
            pipeline,
            bind_group,
            dim_buf,
            input_buf,
            output_buf,
            temp_buf,
            _i: PhantomData,
            _o: PhantomData,
        }
    }

    fn create_buffer(gpu: Arc<Gpu>, size: u64, usages: wgpu::BufferUsages) -> wgpu::Buffer {
        gpu.device().create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: size,
            usage: usages,
            mapped_at_creation: false,
        })
    }

    pub fn set_dims(&self, dims: [u32; 2]) {
        self.gpu
            .queue()
            .write_buffer(&self.dim_buf, 0, bytemuck::cast_slice(&dims));
    }

    pub async fn dispatch(
        &self,
        input: &Vec<I>,
        workgroup_dims: [u32; 3],
    ) -> anyhow::Result<Vec<O>> {
        // Copy input data to GPU
        self.gpu
            .queue()
            .write_buffer(&self.input_buf, 0, bytemuck::cast_slice(&input));

        let mut encoder = self
            .gpu
            .device()
            .create_command_encoder(&Default::default());

        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.dispatch_workgroups(workgroup_dims[0], workgroup_dims[1], workgroup_dims[2]);
        }

        encoder.copy_buffer_to_buffer(
            &self.output_buf,
            0,
            &self.temp_buf,
            0,
            self.output_buf.size(),
        );
        self.gpu.queue().submit([encoder.finish()]);

        let output_data: Vec<O> = {
            let (tx, rx) = channel();

            self.temp_buf
                .map_async(wgpu::MapMode::Read, .., move |result| {
                    tx.send(result).unwrap()
                });
            self.gpu
                .device()
                .poll(wgpu::PollType::wait_indefinitely())?;
            rx.recv()??;

            eprintln!("Data processed");
            let output_view = self.temp_buf.get_mapped_range(..);
            bytemuck::cast_slice(&output_view).to_vec()
        };

        // We need to unmap the buffer to be able to use it again
        self.temp_buf.unmap();

        Ok(output_data)
    }
}
