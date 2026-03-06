use crate::lib::random::{rand, rand_int};
use crate::lib::vec3::Vec3;

pub struct Perlin {
    rand_vec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    const point_count: i32 = 256;

    pub fn new() -> Self {
        Self {
            rand_vec: (0..Self::point_count)
                .map(|i| Vec3::rand_range(-1.0, 1.0).unit())
                .collect(),
            perm_x: Self::generate_perm(),
            perm_y: Self::generate_perm(),
            perm_z: Self::generate_perm(),
        }
    }

    pub fn noise(&self, point: &Vec3) -> f64 {
        let mut u = point.x() - point.x().floor();
        let mut v = point.y() - point.y().floor();
        let mut w = point.z() - point.z().floor();

        let i = point.x().floor() as i32;
        let j = point.y().floor() as i32;
        let k = point.z().floor() as i32;

        let mut c = vec![vec![vec![Vec3::zeroes(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let x_idx = ((i + di) & 255) as usize;
                    let y_idx = ((j + dj) & 255) as usize;
                    let z_idx = ((k + dk) & 255) as usize;
                    let rand_idx =
                        (self.perm_x[x_idx] ^ self.perm_y[y_idx] ^ self.perm_z[z_idx]) as usize;
                    c[di as usize][dj as usize][dk as usize] = self.rand_vec[rand_idx];
                }
            }
        }

        Self::perlin_interpolate(c, u, v, w)
    }

    pub fn turb(&self, point: &Vec3, depth: i32) -> f64 {
        let mut accum = 0.0f64;
        let mut temp_p = *point;
        let mut weight = 1.0f64;

        for i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p.scale(2.0);
        }

        accum.abs()
    }

    fn generate_perm() -> Vec<i32> {
        let p: Vec<i32> = (0..Self::point_count).collect();
        Self::permute(p, Self::point_count)
    }

    fn permute(p: Vec<i32>, n: i32) -> Vec<i32> {
        let mut new_p = p;
        for i in (1..n).rev() {
            let target = rand_int(0, i) as usize;
            let tmp = new_p[i as usize];
            new_p[i as usize] = new_p[target];
            new_p[target] = tmp;
        }
        new_p
    }

    fn perlin_interpolate(c: Vec<Vec<Vec<Vec3>>>, u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0f64;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let fi = f64::from(i as i32);
                    let fj = f64::from(j as i32);
                    let fk = f64::from(k as i32);

                    let weight_v = Vec3::new(u - fi, v - fj, w - fk);

                    let i_acc = (fi * uu) + (1.0 - fi) * (1.0 - uu);
                    let j_acc = (fj * vv) + (1.0 - fj) * (1.0 - vv);
                    let k_acc = (fk * ww) + (1.0 - fk) * (1.0 - ww);
                    let c_dot = c[i as usize][j as usize][k as usize].dot(&weight_v);
                    accum += i_acc * j_acc * k_acc * c_dot;
                }
            }
        }
        accum
    }
}
