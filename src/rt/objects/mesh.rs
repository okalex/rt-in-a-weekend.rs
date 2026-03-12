use std::sync::Arc;

use nalgebra::Point3;
use parry3d_f64::{
    bounding_volume::Aabb,
    math::Vec3,
    query::RayCast,
    shape::{FeatureId, TriMesh},
};

use crate::rt::{
    interval::Interval,
    materials::material::Material,
    objects::{hittable::{HitRecord, Hittable}, triangle::Triangle},
    ray::Ray,
    util::from_parry_vec,
};

pub struct Mesh {
    underlying: TriMesh,
    tex_coords: Arc<Vec<[f64; 2]>>,
    tex_coord_indices: Arc<Vec<u32>>,
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
        let tex_coords: Arc<Vec<[f64; 2]>> = Arc::new(
            obj.texcoords
                .chunks_exact(2)
                .map(|vt| [vt[0] as f64, vt[1] as f64])
                .collect(),
        );
        let tex_coord_indices = Arc::new(obj.texcoord_indices.clone());
        let underlying = match TriMesh::new(vertices, indices) {
            Ok(mesh) => mesh,
            _ => panic!(),
        };
        let bbox = underlying.local_aabb();

        Self {
            underlying,
            tex_coords,
            tex_coord_indices,
            bbox,
            mat,
        }
    }

    pub fn get_face_uvs(&self, face_id: usize, hit_point: Point3<f64>) -> [f64; 2] {
        if face_id >= self.tex_coord_indices.len() {
            return [0.0, 0.0];
        }

        let tex_coord_idx = self.tex_coord_indices[face_id] as usize;
        if tex_coord_idx >= self.tex_coords.len() {
            return [0.0, 0.0];
        }

        // For face_id, get the 3 tex coord indices
        let base = face_id * 3;
        let uv0 = self.tex_coords[self.tex_coord_indices[base] as usize];
        let uv1 = self.tex_coords[self.tex_coord_indices[base + 1] as usize];
        let uv2 = self.tex_coords[self.tex_coord_indices[base + 2] as usize];

        // Get triangle vertices from the TriMesh
        let tri = self.underlying.triangle(face_id as u32);
        // Compute barycentric coords of hit point on the triangle
        let verts = tri.vertices();
        let hit = Vec3::new(hit_point.x, hit_point.y, hit_point.z);
        let (w0, w1, w2) = Triangle::barycentric_coords(&verts[0], &verts[1], &verts[2], &hit); // returns (u, v, w)

        // Interpolate UVs
        let u = w0 * uv0[0] + w1 * uv1[0] + w2 * uv2[0];
        let v = w0 * uv0[1] + w1 * uv1[1] + w2 * uv2[1];
        [u, v]
    }

    
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let r = ray.to_parry3d();
        match self
            .underlying
            .cast_local_ray_and_get_normal(&r, ray_t.max, true)
        {
            Some(intersection) if intersection.time_of_impact >= ray_t.min => {
                let point = ray.at(intersection.time_of_impact);
                let normal = intersection.normal;
                let front_face = r.dir.dot(normal) >= 0.0;

                let [u, v] = match intersection.feature {
                    FeatureId::Face(id) => {
                        let num_tris = self.underlying.indices().len() as u32;
                        let real_id = if id >= num_tris { id - num_tris } else { id };
                        self.get_face_uvs(real_id as usize, point)
                        // self.get_tex_coords(id as usize, point)
                    }
                    _ => [0.0, 0.0],
                };

                Some(HitRecord::new(
                    point,
                    from_parry_vec(normal),
                    front_face,
                    intersection.time_of_impact,
                    u, // TODO
                    v, // TODO
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
