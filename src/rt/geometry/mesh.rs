use std::time::Duration;

use glam::Vec2;
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
            triangle::Triangle,
        },
        ray::Ray,
    },
    util::{
        interval::Interval,
        types::{
            Float,
            Vector,
            INFINITY,
        },
    },
};

pub struct TriangleId {
    pub id: usize,
}

pub struct Mesh {
    pub triangles: Vec<Triangle>,
    pub aabb: Aabb,
    pub bvh: Bvh2,
}

impl Mesh {
    pub fn from_tobj(obj: &tobj::Mesh) -> Self {
        let vertices: Vec<Vector> = obj
            .positions
            .chunks_exact(3)
            .map(|m| Vector::new(m[0].into(), m[1].into(), m[2].into()))
            .collect();

        let uvs: Vec<Vec2> = obj
            .texcoords
            .chunks_exact(2)
            .map(|vt| Vec2::new(vt[0] as Float, vt[1] as Float))
            .collect();

        let indices: Vec<[usize; 3]> = obj
            .indices
            .chunks_exact(3)
            .map(|i| [i[0] as usize, i[1] as usize, i[2] as usize])
            .collect();

        let triangles: Vec<Triangle> = indices
            .iter()
            .map(|i| {
                let v0 = vertices[i[0]];
                let v1 = vertices[i[1]];
                let v2 = vertices[i[2]];
                if uvs.len() > 0 {
                    let uv0 = uvs[i[0]];
                    let uv1 = uvs[i[1]];
                    let uv2 = uvs[i[2]];
                    Triangle::new_with_uvs(v0, v1, v2, uv0, uv1, uv2)
                } else {
                    Triangle::new(v0, v1, v2) // TODO: Use planar uv-mapping instead
                }
            })
            .collect();

        let bvh = Self::build_bvh(&triangles);
        let obvhs_aabb = bvh.nodes[0].aabb;
        let aabb = Aabb::new(Vector::from(obvhs_aabb.min), Vector::from(obvhs_aabb.max));

        Self { triangles, aabb, bvh }
    }

    fn build_bvh(triangles: &Vec<Triangle>) -> Bvh2 {
        let mut build_time = Duration::default();
        let aabbs: Vec<_> = triangles.iter().map(|i| i.aabb.to_obvhs()).collect();
        build_bvh2(&aabbs, BvhBuildParams::fastest_build(), &mut build_time)
    }

    pub fn get_triangle(&self, triangle_id: &TriangleId) -> Option<&Triangle> {
        self.triangles.get(triangle_id.id)
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let obvhs_ray = ray.to_obvhs(ray_t);
        let mut rec = RayHit::none();
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_t = ray_t.max;

        self.bvh.ray_traverse(obvhs_ray, &mut rec, |_r, prim_idx| {
            let triangle_id = TriangleId {
                id: self.bvh.primitive_indices[prim_idx] as usize,
            };

            let triangle = match self.get_triangle(&triangle_id) {
                Some(triangle) => triangle,
                None => return INFINITY,
            };

            match triangle.hit(ray, ray_t.update_max(closest_t)) {
                None => INFINITY,

                Some(hit) => {
                    closest_t = hit.t;
                    hit_record = Some(hit);
                    closest_t
                }
            }
        });

        hit_record
    }
}
