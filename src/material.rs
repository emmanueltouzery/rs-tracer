use crate::{v3color::*, shapes::*, texture::*};

use rand::{prelude as random, Rng};

pub struct MaterialScatterInfo {
    pub attenuation: Color,
    pub scattered: Ray
}

pub trait Material: Sync {
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
    pub albedo: Box<Texture>
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<MaterialScatterInfo> {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere();
        Some(MaterialScatterInfo {
            scattered: Ray {
                origin: hit_record.p, 
                direction: target - hit_record.p,
                time: _ray_in.time
            },
            attenuation: self.albedo.value(&hit_record.p)
        })
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32 // todo new ctor to clamp to 1.0 max?
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<MaterialScatterInfo> {
        let reflected = V3::reflect(
            &ray_in.direction.unit(), 
            &hit_record.normal);
        Some(MaterialScatterInfo {
            scattered: Ray {
                origin: hit_record.p,
                direction: reflected + self.fuzz*random_in_unit_sphere(),
                time: ray_in.time
            },
            attenuation: self.albedo
        }).filter(|v| V3::dot(&v.scattered.direction, &hit_record.normal) > 0.0)
    }
}

pub struct Dielectric {
    pub ref_idx: f32
}

fn refract(in_direction: &V3, normal: &V3, ni_over_nt: f32) -> Option<V3> {
    let u_indir = in_direction.unit();
    let dt = V3::dot(&u_indir, &normal);
    let discriminant = 1.0 - ni_over_nt*ni_over_nt*(1.0-dt*dt);
    if discriminant > 0.0 {
        Some(ni_over_nt*(u_indir - normal*dt) - normal*f32::sqrt(discriminant))
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ((1.0-ref_idx) / (1.0+ref_idx)).powi(2);
    r0 + (1.0-r0)*(1.0-cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<MaterialScatterInfo> {
        let mut rng = random::thread_rng();
        let reflected = || V3::reflect(
            &ray_in.direction.unit(), 
            &hit_record.normal);
        let (outward_normal, ni_over_nt, cosine_factor) = 
            if V3::dot(&ray_in.direction, &hit_record.normal) > 0.0 {
                (-hit_record.normal, self.ref_idx, self.ref_idx)
            } else {
                (hit_record.normal, 1.0 / self.ref_idx, -1.0)
            };

        let refract_direction_fn = |refracted| {
            let cosine = cosine_factor 
                * V3::dot(&ray_in.direction, &hit_record.normal)
                / ray_in.direction.length();
            let reflect_prob = schlick(cosine, self.ref_idx);
            if rng.gen::<f32>() < reflect_prob {
                reflected()
            } else {
                refracted
            }
        };

        Some(MaterialScatterInfo {
            scattered: Ray {
                origin: hit_record.p,
                direction:
                    refract(&ray_in.direction, &outward_normal, ni_over_nt)
                        .map_or_else(reflected, refract_direction_fn),
                time: ray_in.time
            },
            attenuation: Color { r: 1.0, g: 1.0, b: 1.0 }
        })
    }
}