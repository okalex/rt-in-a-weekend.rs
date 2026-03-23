use std::time::Duration;

use glam::{
    Mat3,
    Mat4,
    Vec3,
};
use obvhs::{
    bvh2::{
        builder::build_bvh2,
        Bvh2,
    },
    ray::RayHit,
    BvhBuildParams,
};

use crate::{
    rt::{
        geometry::{
            aabb::Aabb,
            hit_record::HitRecord,
            mesh::Mesh,
            primitive::Primitive,
        },
        materials::material::Material,
        ray::Ray,
    },
    util::{
        interval::Interval,
        types::{
            Float,
            INFINITY,
        },
    },
};

#[derive(Clone, Copy)]
pub struct PrimitiveId {
    pub id: usize,
}

#[derive(Clone, Copy)]
pub struct InstanceId {
    pub id: usize,
}

#[derive(Clone, Copy)]
pub struct LightId {
    #[allow(unused)]
    pub id: usize,
}

#[derive(Clone, Copy)]
pub struct MeshId {
    pub id: usize,
}

#[derive(Clone)]
pub struct MeshDescriptor {
    pub id: MeshId,
}

#[derive(Clone, Copy)]
pub struct MaterialId {
    pub id: usize,
}

pub struct Instance {
    pub primitive_id: PrimitiveId,
    pub material_id: MaterialId,
    pub transform: Mat4,
    pub inv_transform: Mat4,
}

impl Instance {
    pub fn new(primitive_id: PrimitiveId, material_id: MaterialId) -> Self {
        Self {
            primitive_id,
            material_id,
            transform: Mat4::IDENTITY,
            inv_transform: Mat4::IDENTITY,
        }
    }

    pub fn transform(&self, transform: Mat4) -> Self {
        let new_transform = transform * self.transform;
        Self {
            primitive_id: self.primitive_id,
            material_id: self.material_id,
            transform: new_transform,
            inv_transform: new_transform.inverse(),
        }
    }

    #[allow(unused)]
    pub fn scale(&self, scale: [Float; 3]) -> Self {
        let transform = Mat4::from_scale(Vec3::from(scale));
        self.transform(transform)
    }

    #[allow(unused)]
    pub fn scale_uniform(&self, scale: Float) -> Self {
        let transform = Mat4::from_scale(Vec3::splat(scale));
        self.transform(transform)
    }

    #[allow(unused)]
    pub fn translate(&self, translation: [Float; 3]) -> Self {
        let transform = Mat4::from_translation(Vec3::from(translation));
        self.transform(transform)
    }

    #[allow(unused)]
    pub fn rotate_x(&self, radians: Float) -> Self {
        let transform = Mat4::from_rotation_x(radians);
        self.transform(transform)
    }

    #[allow(unused)]
    pub fn rotate_y(&self, radians: Float) -> Self {
        let transform = Mat4::from_rotation_y(radians);
        self.transform(transform)
    }

    #[allow(unused)]
    pub fn rotate_z(&self, radians: Float) -> Self {
        let transform = Mat4::from_rotation_z(radians);
        self.transform(transform)
    }
}

pub struct SceneBuilder {
    primitives: Vec<Primitive>,
    instances: Vec<Instance>,
    meshes: Vec<Mesh>,
    materials: Vec<Material>,
    lights: Vec<InstanceId>,
}

impl SceneBuilder {
    pub fn new() -> Self {
        Self {
            primitives: vec![],
            instances: vec![],
            meshes: vec![],
            materials: vec![],
            lights: vec![],
        }
    }

    pub fn add_primitive(&mut self, primitive: Primitive) -> PrimitiveId {
        let id = self.primitives.len();
        self.primitives.push(primitive);
        PrimitiveId { id }
    }

    pub fn add_material(&mut self, material: Material) -> MaterialId {
        let id = self.materials.len();
        self.materials.push(material);
        MaterialId { id }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> MeshId {
        let id = self.meshes.len();
        self.meshes.push(mesh);
        MeshId { id }
    }

    pub fn add_instance(&mut self, instance: Instance) -> InstanceId {
        if !self.contains_primitive(&instance.primitive_id) || !self.contains_material(&instance.material_id) {
            panic!();
        }

        let id = self.instances.len();
        self.instances.push(instance);
        InstanceId { id }
    }

    pub fn create_instance(&mut self, primitive_id: PrimitiveId, material_id: MaterialId) -> InstanceId {
        let instance = Instance::new(primitive_id, material_id);
        self.add_instance(instance)
    }

    pub fn add_light(&mut self, instance_id: InstanceId) -> LightId {
        if !self.contains_instance(&instance_id) {
            panic!();
        }

        let id = self.lights.len();
        self.lights.push(instance_id);
        LightId { id }
    }

    pub fn build(self) -> Scene {
        let mut scene = Scene::new(self.primitives, self.instances, self.meshes, self.materials, self.lights);
        scene.build_bvh();
        scene
    }

    fn contains_primitive(&self, primitive_id: &PrimitiveId) -> bool {
        primitive_id.id < self.primitives.len()
    }

    fn contains_material(&self, material_id: &MaterialId) -> bool {
        material_id.id < self.materials.len()
    }

    fn contains_instance(&self, instance_id: &InstanceId) -> bool {
        instance_id.id < self.instances.len()
    }
}

pub struct Scene {
    pub primitives: Vec<Primitive>,
    pub instances: Vec<Instance>,
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
    pub lights: Vec<InstanceId>,
    pub bvh: Bvh2,
}

impl Scene {
    pub fn new(
        primitives: Vec<Primitive>,
        instances: Vec<Instance>,
        meshes: Vec<Mesh>,
        materials: Vec<Material>,
        lights: Vec<InstanceId>,
    ) -> Self {
        let instance_count = instances.len();
        Self {
            primitives,
            instances,
            meshes,
            materials,
            lights,
            bvh: Bvh2::zeroed(instance_count),
        }
    }

    fn build_bvh(&mut self) {
        let mut build_time = Duration::default();
        let aabbs: Vec<_> = self
            .instances
            .iter()
            .map(|instance| {
                let primitive = self.get_primitive(&instance.primitive_id).unwrap(); // This should be safe
                let aabb = self.aabb_for(primitive).transform(instance.transform);
                aabb.to_obvhs()
            })
            .collect();
        self.bvh = build_bvh2(&aabbs, BvhBuildParams::fastest_build(), &mut build_time);
    }

    pub fn get_instance(&self, instance_id: &InstanceId) -> Option<&Instance> {
        self.instances.get(instance_id.id)
    }

    pub fn get_primitive(&self, primitive_id: &PrimitiveId) -> Option<&Primitive> {
        self.primitives.get(primitive_id.id)
    }

    pub fn get_mesh(&self, mesh_id: &MeshId) -> Option<&Mesh> {
        self.meshes.get(mesh_id.id)
    }

    pub fn get_material(&self, material_id: &MaterialId) -> Option<&Material> {
        self.materials.get(material_id.id)
    }

    pub fn get_primitive_for(&self, instance_id: &InstanceId) -> Option<&Primitive> {
        self.get_instance(instance_id)
            .and_then(|instance| self.get_primitive(&instance.primitive_id))
    }

    pub fn get_material_for(&self, instance_id: &InstanceId) -> Option<&Material> {
        self.get_instance(instance_id)
            .and_then(|instance| self.get_material(&instance.material_id))
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<(InstanceId, HitRecord)> {
        let obvhs_ray = ray.to_obvhs(ray_t);
        let mut rec = RayHit::none();
        let mut hit_record: Option<(InstanceId, HitRecord)> = None;
        let mut closest_t = ray_t.max;

        self.bvh.ray_traverse(obvhs_ray, &mut rec, |_r, prim_idx| {
            let instance_id = InstanceId {
                id: self.bvh.primitive_indices[prim_idx] as usize,
            };

            let instance = match self.get_instance(&instance_id) {
                Some(instance) => instance,
                None => return INFINITY,
            };
            let primitive = match self.get_primitive_for(&instance_id) {
                Some(primitive) => primitive,
                None => return INFINITY,
            };

            // Convert to primitive local space
            let world_to_local = instance.inv_transform;
            let local_to_world = instance.transform;
            let transformed_ray = ray.transform(world_to_local);

            match self.hit_primitive(primitive, &transformed_ray, ray_t.update_max(closest_t)) {
                None => INFINITY,

                Some(hit) => {
                    // Convert to world space
                    let normal_transform = Mat3::from_mat4(local_to_world.inverse().transpose());
                    let new_hit = HitRecord::new(
                        local_to_world.transform_point3(hit.point),
                        (normal_transform * hit.normal).normalize(),
                        hit.front_face,
                        hit.t,
                        hit.u,
                        hit.v,
                    );
                    hit_record = Some((instance_id, new_hit));

                    closest_t = hit.t;
                    closest_t
                }
            }
        });

        hit_record
    }

    fn hit_primitive(&self, primitive: &Primitive, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        match primitive {
            Primitive::Sphere(sphere) => sphere.hit(ray, ray_t),
            Primitive::Quad(quad) => quad.hit(ray, ray_t),
            Primitive::Triangle(triangle) => triangle.hit(ray, ray_t),
            Primitive::Mesh(mesh) => self.get_mesh(&mesh.id).and_then(|m| m.hit(ray, ray_t)),
        }
    }

    pub fn aabb_for(&self, primitive: &Primitive) -> Aabb {
        match primitive {
            Primitive::Sphere(sphere) => sphere.aabb,
            Primitive::Quad(quad) => quad.aabb,
            Primitive::Triangle(triangle) => triangle.aabb,
            Primitive::Mesh(mesh) => self.get_mesh(&mesh.id).map(|m| m.aabb).unwrap(),
        }
    }
}
