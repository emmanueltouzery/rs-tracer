use crate::{v3color::*, shapes::*};

use rand::{prelude as random, Rng};

pub struct MaterialScatterInfo {
    pub attenuation: V3,
    pub scattered: Ray
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<MaterialScatterInfo>;
}

fn random_in_unit_sphere() -> V3 {
    let mut rng = random::thread_rng();
    let mut p;
    let unit = V3 { x: 1.0, y: 1.0, z: 1.0};
    loop {
        p = 2.0 * V3 { 
            x: rng.gen::<f32>(),
            y: rng.gen::<f32>(),
            z: rng.gen::<f32>()
        } - unit;
        if p.squared_length() < 1.0 { break p; }
    }
}

pub struct Lambertian {
    pub albedo: V3
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<MaterialScatterInfo> {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere();
        Some(MaterialScatterInfo {
            scattered: Ray {
                origin: hit_record.p, 
                direction: target - hit_record.p
            },
            attenuation: self.albedo
        })
    }
}

pub struct Metal {
    pub albedo: V3
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<MaterialScatterInfo> {
        let reflected = V3::reflect(
            &ray_in.direction.unit(), 
            &hit_record.normal);
        Some(MaterialScatterInfo {
            scattered: Ray {
                origin: hit_record.p,
                direction: reflected
            },
            attenuation: self.albedo
        }).filter(|v| V3::dot(&v.scattered.direction, &hit_record.normal) > 0.0)
    }
}