use std::{
    marker::PhantomData,
    sync::{Arc, mpsc::channel},
};

use bytemuck::{AnyBitPattern, NoUninit};
use encase::ShaderType;

use crate::rt::{gpu::gpu::Gpu, renderer::gpu::gpu_types::GpuMeta};

pub struct GpuCompute<I: NoUninit + AnyBitPattern, O: NoUninit + AnyBitPattern> {
    gpu: Arc<Gpu>,
    pipeline: wgpu::ComputePipeline,
    bind_group: wgpu::BindGroup,
    meta_buf: wgpu::Buffer,
    output_buf: wgpu::Buffer,
    temp_buf: wgpu::Buffer,
    _i: PhantomData<I>,
    _o: PhantomData<O>,
}

impl<I: NoUninit + AnyBitPattern, O: NoUninit + AnyBitPattern> GpuCompute<I, O> {
    pub fn new(gpu: Arc<Gpu>, shader: &wgpu::ShaderModule, output_size: u64) -> Self {
        let input_usages = wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE;
        let output_usages = wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE;
        let temp_usages = wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ;

        let meta_buf = gpu.create_buffer(
            <GpuMeta as ShaderType>::METADATA.min_size().get(),
            input_usages,
        );

        let output_buf = gpu.create_buffer(output_size, output_usages);
        let temp_buf = gpu.create_buffer(output_size, temp_usages);

        let pipeline = gpu.create_compute_pipeline(shader);
        let bind_group_layout = pipeline.get_bind_group_layout(0);
        let bind_group = gpu.create_bind_group(
            &bind_group_layout,
            &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: output_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: meta_buf.as_entire_binding(),
                },
            ],
        );

        Self {
            gpu,
            pipeline,
            bind_group,
            meta_buf,
            output_buf,
            temp_buf,
            _i: PhantomData,
            _o: PhantomData,
        }
    }

    pub fn set_meta(&self, meta: GpuMeta) {
        let mut buffer = encase::StorageBuffer::new(Vec::new());
        buffer.write(&meta).unwrap();
        self.gpu
            .queue()
            .write_buffer(&self.meta_buf, 0, buffer.as_ref());
    }

    pub async fn dispatch(&self, workgroup_dims: [u32; 2]) -> anyhow::Result<Vec<O>> {
        let mut encoder = self
            .gpu
            .device()
            .create_command_encoder(&Default::default());

        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.dispatch_workgroups(workgroup_dims[0], workgroup_dims[1], 1);
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
