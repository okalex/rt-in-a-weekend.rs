use std::{
    marker::PhantomData,
    sync::{Arc, mpsc::channel},
};

use bytemuck::{AnyBitPattern, NoUninit};
use encase::{ShaderType, internal::WriteInto};

use crate::{
    gpu::gpu::Gpu,
    rt::renderer::gpu::{gpu_meta::GpuMeta, gpu_types::GpuScene},
};

pub struct RenderPipeline<O: NoUninit + AnyBitPattern> {
    gpu: Arc<Gpu>,
    pipeline: wgpu::ComputePipeline,
    bind_group: wgpu::BindGroup,
    pub meta_buf: wgpu::Buffer,
    pub primitives_buf: wgpu::Buffer,
    pub instances_buf: wgpu::Buffer,
    pub instance_bvh_buf: wgpu::Buffer,
    pub meshes_buf: wgpu::Buffer,
    pub mesh_bvhs_buf: wgpu::Buffer,
    pub materials_buf: wgpu::Buffer,
    pub lights_buf: wgpu::Buffer,
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

impl<O: NoUninit + AnyBitPattern> RenderPipeline<O> {
    pub fn new(gpu: Arc<Gpu>, shader: &wgpu::ShaderModule, output_size: u64, meta: Arc<GpuMeta>, scene: Arc<GpuScene>) -> Self {
        let meta_size = meta.size().get();
        let meta_buf = gpu.create_buffer(meta_size, buffer_usages::UNIFORM);

        let primitives_size = scene.primitives.size().get();
        let primitives_buf = gpu.create_buffer(primitives_size, buffer_usages::INPUT);

        let instances_size = scene.instances.size().get();
        let instances_buf = gpu.create_buffer(instances_size, buffer_usages::INPUT);

        let instance_bvh_size = scene.instance_bvh.size().get();
        let instance_bvh_buf = gpu.create_buffer(instance_bvh_size, buffer_usages::INPUT);

        let meshes_size = scene.meshes.size().get();
        let meshes_buf = gpu.create_buffer(meshes_size, buffer_usages::INPUT);

        let mesh_bvhs_size = scene.mesh_bvhs.size().get();
        let mesh_bvhs_buf = gpu.create_buffer(mesh_bvhs_size, buffer_usages::INPUT);

        let materials_size = scene.materials.size().get();
        let materials_buf = gpu.create_buffer(materials_size, buffer_usages::INPUT);

        let lights_size = scene.lights.size().get();
        let lights_buf = gpu.create_buffer(lights_size, buffer_usages::INPUT);

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
                    resource: instance_bvh_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: meshes_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 6,
                    resource: mesh_bvhs_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 7,
                    resource: materials_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 8,
                    resource: lights_buf.as_entire_binding(),
                },
            ],
        );

        let render_pipeline = Self {
            gpu,
            pipeline,
            bind_group,
            meta_buf,
            primitives_buf,
            instances_buf,
            instance_bvh_buf,
            meshes_buf,
            mesh_bvhs_buf,
            materials_buf,
            lights_buf,
            output_buf,
            temp_buf,
            _o: PhantomData,
        };
        render_pipeline.load_scene(scene);
        render_pipeline
    }

    pub fn load_scene(&self, scene: Arc<GpuScene>) {
        self.init_buf(&scene.primitives, &self.primitives_buf);
        self.init_buf(&scene.instances, &self.instances_buf);
        self.init_buf(&scene.instance_bvh, &self.instance_bvh_buf);
        self.init_buf(&scene.meshes, &self.meshes_buf);
        self.init_buf(&scene.mesh_bvhs, &self.mesh_bvhs_buf);
        self.init_buf(&scene.materials, &self.materials_buf);
        self.init_buf(&scene.lights, &self.lights_buf);
    }

    pub fn init_buf<T: ShaderType + WriteInto>(&self, data: &T, buf: &wgpu::Buffer) {
        let mut buffer = encase::StorageBuffer::new(Vec::new());
        buffer.write(data).unwrap();
        self.gpu.queue().write_buffer(buf, 0, buffer.as_ref());
    }

    // Warm up the pipeline - this was meant to prevent dropped dispatches, but it
    // didn't work. Need to investigate
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

    // Run the renderer
    pub fn dispatch(&self, workgroup_dims: [u32; 2]) {
        let mut encoder = self.gpu.device().create_command_encoder(&Default::default());

        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.dispatch_workgroups(workgroup_dims[0], workgroup_dims[1], 1);
        }

        encoder.copy_buffer_to_buffer(&self.output_buf, 0, &self.temp_buf, 0, self.output_buf.size());
        self.gpu.queue().submit([encoder.finish()]);
    }

    // Read the output buffer
    pub async fn get_result(&self) -> anyhow::Result<Vec<O>> {
        let output_data: Vec<O> = {
            let (tx, rx) = channel();

            self.temp_buf
                .map_async(wgpu::MapMode::Read, .., move |result| tx.send(result).unwrap());
            self.gpu.device().poll(wgpu::PollType::wait_indefinitely())?;
            rx.recv()??;

            let output_view = self.temp_buf.get_mapped_range(..);
            bytemuck::cast_slice(&output_view).to_vec()
        };

        // Unmap the buffer to be able to use it again
        self.temp_buf.unmap();

        Ok(output_data)
    }
}
