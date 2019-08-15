use arr_macro::arr;

use rand::{prelude as random, Rng};
use rand::seq::SliceRandom;

use crate::{v3color::*, texture::*};

fn perlin_generate() -> [f32; 256] {
    let mut rng = random::thread_rng();
    arr!(rng.gen::<f32>(); 256)
}

fn perlin_generate_perm() -> [i32; 256] {
    let mut rng = random::thread_rng();
    let mut i = 0;
    let mut p = arr!(({i+=1; i-1}); 256);
    p.shuffle(&mut rng);
    p
}

pub struct NoiseTexture {
    pub ranfloat: [f32; 256],
    pub perm_x: [i32; 256],
    pub perm_y: [i32; 256],
    pub perm_z: [i32; 256]
}

impl NoiseTexture {
    pub fn new() -> NoiseTexture {
        let ranfloat = perlin_generate();
        let perm_x = perlin_generate_perm();
        let perm_y = perlin_generate_perm();
        let perm_z = perlin_generate_perm();
        NoiseTexture {
            ranfloat, perm_x, perm_y, perm_z
        }
    }

    fn noise(&self, p: &V3) -> f32 {
        // let u = p.x - f32::floor(p.x);
        // let v = p.y - f32::floor(p.y);
        // let w = p.z - f32::floor(p.z);
        let i = ((4.0*p.x) as usize) & 255;
        let j = ((4.0*p.y) as usize) & 255;
        let k = ((4.0*p.z) as usize) & 255;
        self.ranfloat[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize]
    }
}

impl Texture for NoiseTexture {
    fn value(&self, p: &V3) -> Color {
        (V3 { x: 1.0, y: 1.0, z: 1.0} * self.noise(&p)).to_color()
    }
}