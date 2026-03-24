import package::{
    types,
};

@group(0) @binding(0) var<storage, read_write> output: array<vec4<f32>>;
@group(0) @binding(1) var<uniform> gpu_meta: types::GpuMeta;
@group(0) @binding(2) var<storage, read> primitives: types::GpuPrimitives;
@group(0) @binding(3) var<storage, read> instances: types::GpuInstances;
@group(0) @binding(4) var<storage, read> instance_bvh: types::GpuBvh;
@group(0) @binding(5) var<storage, read> meshes: types::GpuMeshes;
@group(0) @binding(6) var<storage, read> mesh_bvhs: types::GpuBvh;
@group(0) @binding(7) var<storage, read> materials: types::GpuMaterials;
@group(0) @binding(8) var<storage, read> lights: types::GpuLights;
