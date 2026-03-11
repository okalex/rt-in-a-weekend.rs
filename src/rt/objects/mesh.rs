use std::sync::Arc;

use parry3d_f64::{bounding_volume::Aabb, math::Vec3, query::RayCullingMode, shape::TriMesh};

use crate::rt::{
    interval::Interval,
    materials::material::Material,
    objects::hittable::{HitRecord, Hittable},
    ray::Ray, util::from_parry_vec,
};

pub struct Mesh {
    underlying: TriMesh,
    bbox: Aabb,
    mat: Arc<dyn Material>,
}

impl Mesh {
    pub fn from_tobj(obj: &tobj::Mesh, mat: Arc<dyn Material>) -> Self {
        let vertices = obj
            .positions
            .chunks_exact(3)
            .map(|m| Vec3::new(m[0].into(), m[1].into(), m[2].into()))
            .collect();
        let indices = obj
            .indices
            .chunks_exact(3)
            .map(|i| [i[0], i[1], i[2]])
            .collect();
        let underlying = match TriMesh::new(vertices, indices) {
            Ok(mesh) => mesh,
            _ => panic!(),
        };
        let bbox = underlying.local_aabb();

        Self {
            underlying,
            bbox,
            mat,
        }
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let r = ray.to_parry3d();
        match self
            .underlying
            .cast_local_ray_with_culling(&r, ray_t.max, RayCullingMode::IgnoreBackfaces)
        {
            Some(intersection) if intersection.time_of_impact >= ray_t.min => {
                let normal = intersection.normal;
                let front_face = r.dir.dot(normal) >= 0.0;
                Some(HitRecord::new(
                    ray.at(intersection.time_of_impact),
                    from_parry_vec(normal),
                    front_face,
                    intersection.time_of_impact,
                    0.0, // TODO
                    0.0, // TODO
                    Arc::clone(&self.mat),
                ))
            }

            _ => None,
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
