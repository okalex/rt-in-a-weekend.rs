import package::{
    color::Rgba,
    types,
};

@group(0) @binding(0) var<storage, read_write> output: array<Rgba>;
@group(0) @binding(1) var<uniform> gpu_meta: types::GpuMeta;
@group(0) @binding(2) var<storage, read> objects: types::GpuObjects;
@group(0) @binding(3) var<storage, read> materials: types::GpuMaterials;
@group(0) @binding(4) var<storage, read> bvh: types::GpuBvh;
