use crate::{v3color::*, material::*};

pub struct Ray {
    pub origin: V3,
    pub direction: V3,
    pub time: f32
}

impl Ray {
    pub fn point_at_parameter(&self, t: f32) -> V3 {
        self.origin + self.direction*t
    }
}

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: V3,
    pub normal: V3,
    pub material: &'a Material
}

pub trait Shape: Sync {
    fn hit<'a>(&'a self, ray: &Ray, t_range: &std::ops::Range<f32>) -> Option<HitRecord<'a>>;
}

fn sphere_hit<'a>(ray: &Ray, sphere_center: &V3, sphere_radius: f32,
        sphere_material: &'a Material, t_range: &std::ops::Range<f32>) -> Option<HitRecord<'a>> {
    let oc = ray.origin - sphere_center;
    let a = V3::dot(&ray.direction, &ray.direction);
    let b = V3::dot(&oc, &ray.direction);
    let c = V3::dot(&oc, &oc) - sphere_radius*sphere_radius;
    let discriminant = b*b - a*c;

    let get_hit_record = |solution| {
        let point = ray.point_at_parameter(solution);
        Some(HitRecord {
            t: solution,
            p: point,
            normal: (point - sphere_center) / sphere_radius,
            material: &*sphere_material
        })
    };

    if discriminant > 0.0 {
        let discriminant_root = f32::sqrt(discriminant);
        let solution1 = (-b - discriminant_root) / a;
        if t_range.contains(&solution1) {
            return get_hit_record(solution1);
        }
        let solution2 = (-b + discriminant_root)/a;
        if t_range.contains(&solution2) {
            return get_hit_record(solution2);
        }
    }
    None
}

pub struct Sphere {
    pub center: V3,
    pub radius: f32,
    pub material: Box<Material>
}

impl Shape for Sphere {
    fn hit<'a>(&'a self, ray: &Ray, t_range: &std::ops::Range<f32>) -> Option<HitRecord<'a>> {
        sphere_hit(ray, &self.center, self.radius, &*self.material, t_range)
    }
}

pub struct MovingSphere {
    pub center0: V3,
    pub center1: V3,
    pub time0: f32,
    pub time1: f32,
    pub radius: f32,
    pub material: Box<Material>
}

impl Shape for MovingSphere {
    fn hit<'a>(&'a self, ray: &Ray, t_range: &std::ops::Range<f32>) -> Option<HitRecord<'a>> {
        let center = self.center0 + ((ray.time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0);
        sphere_hit(ray, &center, self.radius, &*self.material, t_range)
    }
}