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

use crate::rt::{
    geometry::{
        aabb::Aabb,
        hit_record::HitRecord,
        primitive::Primitive,
    },
    interval::Interval,
    materials::material::Material,
    ray::Ray,
    types::{
        Float,
        INFINITY,
    },
};

#[derive(Clone, Copy)]
pub struct PrimitiveId {
    pub id: usize,
}

#[derive(Clone, Copy)]
pub struct MaterialId {
    pub id: usize,
}

#[derive(Clone, Copy)]
pub struct InstanceId {
    pub id: usize,
}

pub struct Instance {
    pub primitive_id: PrimitiveId,
    pub material_id: MaterialId,
    pub transform: Mat4,
    pub inv_transform: Mat4,
    aabb: Aabb,
}

impl Instance {
    pub fn new(primitive_id: PrimitiveId, material_id: MaterialId, aabb: Aabb) -> Self {
        Self {
            primitive_id,
            material_id,
            transform: Mat4::IDENTITY,
            inv_transform: Mat4::IDENTITY,
            aabb,
        }
    }

    pub fn transform(&self, transform: Mat4) -> Self {
        let new_transform = transform * self.transform;
        Self {
            primitive_id: self.primitive_id,
            material_id: self.material_id,
            transform: new_transform,
            inv_transform: new_transform.inverse(),
            aabb: self.aabb.transform(transform),
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
    materials: Vec<Material>,
}

impl SceneBuilder {
    pub fn new() -> Self {
        Self {
            primitives: vec![],
            instances: vec![],
            materials: vec![],
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

    pub fn add_instance(&mut self, instance: Instance) -> InstanceId {
        if !self.contains_primitive(&instance.primitive_id) || !self.contains_material(&instance.material_id) {
            panic!();
        }

        let id = self.instances.len();
        self.instances.push(instance);
        InstanceId { id }
    }

    pub fn create_instance(&mut self, primitive_id: PrimitiveId, material_id: MaterialId) -> InstanceId {
        let primitive = &self.primitives[primitive_id.id];
        let instance = Instance::new(primitive_id, material_id, primitive.aabb());

        self.add_instance(instance)
    }

    pub fn build(self) -> Scene {
        Scene::new(self.primitives, self.instances, self.materials)
    }

    fn contains_primitive(&self, primitive_id: &PrimitiveId) -> bool {
        primitive_id.id < self.primitives.len()
    }

    fn contains_material(&self, material_id: &MaterialId) -> bool {
        material_id.id < self.materials.len()
    }
}

pub struct Scene {
    pub primitives: Vec<Primitive>,
    pub instances: Vec<Instance>,
    pub materials: Vec<Material>,
    pub bvh: Bvh2,
}

impl Scene {
    pub fn new(primitives: Vec<Primitive>, instances: Vec<Instance>, materials: Vec<Material>) -> Self {
        let bvh = Self::build_bvh(&instances);

        Self {
            primitives,
            instances,
            materials,
            bvh,
        }
    }

    fn build_bvh(instances: &Vec<Instance>) -> Bvh2 {
        let mut build_time = Duration::default();
        let aabbs: Vec<_> = instances.iter().map(|i| i.aabb.to_obvhs()).collect();
        build_bvh2(&aabbs, BvhBuildParams::fastest_build(), &mut build_time)
    }

    pub fn get_instance(&self, instance_id: &InstanceId) -> Option<&Instance> {
        self.instances.get(instance_id.id)
    }

    pub fn get_primitive(&self, primitive_id: &PrimitiveId) -> Option<&Primitive> {
        self.primitives.get(primitive_id.id)
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

            match primitive.hit(&transformed_ray, ray_t.update_max(closest_t)) {
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
}
