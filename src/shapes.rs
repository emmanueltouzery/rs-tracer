use crate::v3color::*;

pub struct Ray {
    pub origin: V3,
    pub direction: V3
}

impl Ray {
    pub fn point_at_parameter(&self, t: f32) -> V3 {
        self.origin + self.direction*t
    }
}

pub struct HitRecord {
    pub t: f32,
    pub p: V3,
    pub normal: V3
}

pub trait Shape {
    fn hit(&self, ray: &Ray, t_range: &std::ops::Range<f32>) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: V3,
    pub radius: f32
}

impl Shape for Sphere {
    fn hit(&self, ray: &Ray, t_range: &std::ops::Range<f32>) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = V3::dot(&ray.direction, &ray.direction);
        let b = V3::dot(&oc, &ray.direction);
        let c = V3::dot(&oc, &oc) - self.radius*self.radius;
        let discriminant = b*b - a*c;

        let get_hit_record = |solution| {
            let point = ray.point_at_parameter(solution);
            Some(HitRecord {
                t: solution,
                p: point,
                normal: (point - self.center) / self.radius
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
}