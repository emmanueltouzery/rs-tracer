use crate::{v3color::*, material::*, bvh::*};

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

#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub p: V3,
    pub normal: V3,
    pub material: &'a Material
}

pub trait Shape: Sync {
    fn hit<'a>(&'a self, ray: &Ray, t_range: &std::ops::Range<f32>) -> Option<HitRecord<'a>>;

    /// at some point we should return an option because not
    /// all primitives have bounding boxes (eg infinite planes)
    /// but for now we don't implement or handle these so...
    fn bounding_box(&self, t_range: &std::ops::Range<f32>) -> Aabb;
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

fn sphere_bounding_box(center: &V3, radius: f32) -> Aabb {
    Aabb {
        min: center - V3 { x: radius, y: radius, z: radius },
        max: center + V3 { x: radius, y: radius, z: radius }
    }
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

    fn bounding_box(&self, _t_range: &std::ops::Range<f32>) -> Aabb {
        sphere_bounding_box(&self.center, self.radius)
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

fn moving_sphere_center_at_time(ms: &MovingSphere, time: f32) -> V3 {
    ms.center0 + ((time - ms.time0) / (ms.time1 - ms.time0)) * (ms.center1 - ms.center0)
}

impl Shape for MovingSphere {
    fn hit<'a>(&'a self, ray: &Ray, t_range: &std::ops::Range<f32>) -> Option<HitRecord<'a>> {
        let center = moving_sphere_center_at_time(&self, ray.time);
        sphere_hit(ray, &center, self.radius, &*self.material, t_range)
    }

    fn bounding_box(&self, t_range: &std::ops::Range<f32>) -> Aabb {
        sphere_bounding_box(&moving_sphere_center_at_time(&self, t_range.start), self.radius)
            .union(&sphere_bounding_box(&moving_sphere_center_at_time(&self, t_range.end), self.radius))
    }
}