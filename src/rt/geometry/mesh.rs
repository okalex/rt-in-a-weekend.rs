use std::sync::Arc;

use parry3d_f64::{
    bounding_volume::Aabb,
    math::Vec3,
    query::RayCast,
    shape::{FeatureId, TriMesh},
};

use crate::rt::{
    interval::Interval,
    geometry::{hit_record::HitRecord, triangle::Triangle},
    ray::Ray,
    types::{Float, Point, Uint, from_parry_vec, to_parry_vec},
};

pub struct Mesh {
    underlying: TriMesh,
    tex_coords: Arc<Vec<[Float; 2]>>,
    tex_coord_indices: Arc<Vec<Uint>>,
    bbox: Aabb,
    mat_idx: usize,
}

impl Mesh {
    pub fn from_tobj(obj: &tobj::Mesh, mat_idx: usize) -> Self {
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
        let tex_coords: Arc<Vec<[Float; 2]>> = Arc::new(
            obj.texcoords
                .chunks_exact(2)
                .map(|vt| [vt[0] as Float, vt[1] as Float])
                .collect(),
        );
        let tex_coord_indices = Arc::new(
            obj.texcoord_indices
                .clone()
                .into_iter()
                .map(|i| i as Uint)
                .collect(),
        );
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
            mat_idx,
        }
    }

    pub fn get_face_uvs(&self, face_id: usize, hit_point: Point) -> [Float; 2] {
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
        let hit = to_parry_vec(hit_point);
        let (w0, w1, w2) = Triangle::barycentric_coords(&verts[0], &verts[1], &verts[2], &hit); // returns (u, v, w)

        // Interpolate UVs
        let u = w0 * uv0[0] + w1 * uv1[0] + w2 * uv2[0];
        let v = w0 * uv0[1] + w1 * uv1[1] + w2 * uv2[1];
        [u, v]
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let r = ray.to_parry3d();
        match self
            .underlying
            .cast_local_ray_and_get_normal(&r, ray_t.max as f64, true)
        {
            Some(intersection) if intersection.time_of_impact >= (ray_t.min as f64) => {
                let point = ray.at(intersection.time_of_impact as Float);
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
                    intersection.time_of_impact as Float,
                    u,
                    v,
                    self.mat_idx,
                ))
            }

            _ => None,
        }
    }

    pub fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
