use std::{
    marker::PhantomData,
    sync::{
        mpsc::channel,
        Arc,
    },
};

use bytemuck::{
    AnyBitPattern,
    NoUninit,
};
use encase::{
    internal::WriteInto,
    ShaderType,
};

use crate::rt::{
    gpu::gpu::Gpu,
    renderer::gpu::gpu_types::{
        GpuBvh,
        GpuInstances,
        GpuMaterials,
        GpuMeta,
        GpuPrimitives,
    },
};

pub struct GpuCompute<O: NoUninit + AnyBitPattern> {
    gpu: Arc<Gpu>,
    pipeline: wgpu::ComputePipeline,
    bind_group: wgpu::BindGroup,
    pub meta_buf: wgpu::Buffer,
    pub primitives_buf: wgpu::Buffer,
    pub instances_buf: wgpu::Buffer,
    pub materials_buf: wgpu::Buffer,
    pub bvh_buf: wgpu::Buffer,
    output_buf: wgpu::Buffer,
    temp_buf: wgpu::Buffer,
    _o: PhantomData<O>,
}

mod buffer_usages {
    pub const INPUT: wgpu::BufferUsages =
        wgpu::BufferUsages::from_bits_retain(wgpu::BufferUsages::COPY_DST.bits() | wgpu::BufferUsages::STORAGE.bits());
    pub const OUTPUT: wgpu::BufferUsages =
        wgpu::BufferUsages::from_bits_retain(wgpu::BufferUsages::COPY_SRC.bits() | wgpu::BufferUsages::STORAGE.bits());
    pub const TEMP: wgpu::BufferUsages =
        wgpu::BufferUsages::from_bits_retain(wgpu::BufferUsages::COPY_DST.bits() | wgpu::BufferUsages::MAP_READ.bits());
    pub const UNIFORM: wgpu::BufferUsages =
        wgpu::BufferUsages::from_bits_retain(wgpu::BufferUsages::UNIFORM.bits() | wgpu::BufferUsages::COPY_DST.bits());
}

impl<O: NoUninit + AnyBitPattern> GpuCompute<O> {
    pub fn new(
        gpu: Arc<Gpu>,
        shader: &wgpu::ShaderModule,
        output_size: u64,
        meta: Arc<GpuMeta>,
        primitives: Arc<GpuPrimitives>,
        instances: Arc<GpuInstances>,
        materials: Arc<GpuMaterials>,
        bvh: Arc<GpuBvh>,
    ) -> Self {
        let meta_size = meta.size().get();
        let meta_buf = gpu.create_buffer(meta_size, buffer_usages::UNIFORM);

        let primitives_size = primitives.size().get();
        let primitives_buf = gpu.create_buffer(primitives_size, buffer_usages::INPUT);

        let instances_size = instances.size().get();
        let instances_buf = gpu.create_buffer(instances_size, buffer_usages::INPUT);

        let materials_size = materials.size().get();
        let materials_buf = gpu.create_buffer(materials_size, buffer_usages::INPUT);

        let bvh_size = bvh.size().get();
        let bvh_buf = gpu.create_buffer(bvh_size, buffer_usages::INPUT);

        let output_buf = gpu.create_buffer(output_size, buffer_usages::OUTPUT);
        let temp_buf = gpu.create_buffer(output_size, buffer_usages::TEMP);

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
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: primitives_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: instances_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: materials_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: bvh_buf.as_entire_binding(),
                },
            ],
        );

        Self {
            gpu,
            pipeline,
            bind_group,
            meta_buf,
            primitives_buf,
            instances_buf,
            materials_buf,
            bvh_buf,
            output_buf,
            temp_buf,
            _o: PhantomData,
        }
    }

    pub fn init_bufs(
        &self,
        meta: Arc<GpuMeta>,
        primitives: Arc<GpuPrimitives>,
        instances: Arc<GpuInstances>,
        materials: Arc<GpuMaterials>,
        bvh: Arc<GpuBvh>,
    ) {
        self.init_buf(&meta, &self.meta_buf);
        self.init_buf(&primitives, &self.primitives_buf);
        self.init_buf(&instances, &self.instances_buf);
        self.init_buf(&materials, &self.materials_buf);
        self.init_buf(&bvh, &self.bvh_buf);
    }

    pub fn init_buf<T: ShaderType + WriteInto>(&self, data: &Arc<T>, buf: &wgpu::Buffer) {
        let mut buffer = encase::StorageBuffer::new(Vec::new());
        buffer.write(data).unwrap();
        self.gpu.queue().write_buffer(buf, 0, buffer.as_ref());
    }

    pub async fn warmup(&self) -> anyhow::Result<()> {
        let mut encoder = self.gpu.device().create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        self.gpu.queue().submit([encoder.finish()]);
        let _ = self.gpu.device().poll(wgpu::PollType::wait_indefinitely()); // block until done

        Ok(())
    }

    pub async fn dispatch(&self, workgroup_dims: [u32; 2]) -> anyhow::Result<Vec<O>> {
        let mut encoder = self.gpu.device().create_command_encoder(&Default::default());

        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.dispatch_workgroups(workgroup_dims[0], workgroup_dims[1], 1);
        }

        encoder.copy_buffer_to_buffer(&self.output_buf, 0, &self.temp_buf, 0, self.output_buf.size());
        self.gpu.queue().submit([encoder.finish()]);

        let output_data: Vec<O> = {
            let (tx, rx) = channel();

            self.temp_buf
                .map_async(wgpu::MapMode::Read, .., move |result| tx.send(result).unwrap());
            self.gpu.device().poll(wgpu::PollType::wait_indefinitely())?;
            rx.recv()??;

            let output_view = self.temp_buf.get_mapped_range(..);
            bytemuck::cast_slice(&output_view).to_vec()
        };

        // We need to unmap the buffer to be able to use it again
        self.temp_buf.unmap();

        Ok(output_data)
    }
}
