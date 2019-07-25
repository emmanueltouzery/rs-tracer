// Bounding Volume Hierarchy

use crate::{v3color::*, shapes::*};

use rand::{prelude as random, Rng};

// aabb == Axis-Aligned Bounding Box
pub struct Aabb {
    pub min: V3,
    pub max: V3
}

impl Aabb {
    pub fn union(&self, other: &Aabb) -> Aabb {
        Aabb {
            min: V3 {
                x: f32::min(self.min.x, other.min.x),
                y: f32::min(self.min.y, other.min.y),
                z: f32::min(self.min.z, other.min.z)
            },
            max: V3 {
                x: f32::max(self.max.x, other.max.x),
                y: f32::max(self.max.y, other.max.y),
                z: f32::max(self.max.z, other.max.z)
            }
        }
    }

    fn check_dimension(&self, ray: &Ray, tmin: &mut f32, tmax: &mut f32,
                       getter:&Fn(&V3)->f32) -> bool {
        let inv_d = 1.0 / getter(&ray.direction);
        let mut t0 = (getter(&self.min) - getter(&ray.origin)) * inv_d;
        let mut t1 = (getter(&self.max) - getter(&ray.origin)) * inv_d;
        if inv_d < 0.0 {
            std::mem::swap(&mut t0, &mut t1);
        };
        if t0 > *tmin {
            *tmin = t0;
        }
        if t1 < *tmax {
            *tmax = t1;
        }
        tmax <= tmin
    }

    pub fn hit(&self, ray: &Ray, t_range: &std::ops::Range<f32>) -> bool {
        let mut tmin = t_range.start;
        let mut tmax = t_range.end;

        if !self.check_dimension(ray, &mut tmin, &mut tmax, &V3::get_x) {
            return false;
        }
        if !self.check_dimension(ray, &mut tmin, &mut tmax, &V3::get_y) {
            return false;
        }
        if !self.check_dimension(ray, &mut tmin, &mut tmax, &V3::get_z) {
            return false;
        }
        true
    }
}

pub struct BvhNode {
    pub left: Box<Shape>,
    pub right: Box<Shape>,
    pub bbox: Aabb
}

impl Shape for BvhNode {
    fn hit<'a>(&'a self, ray: &Ray, t_range: &std::ops::Range<f32>) -> Option<HitRecord<'a>> {
        if !self.bbox.hit(&ray, t_range) {
            return None;
        }
        let hit_left = self.left.hit(&ray, t_range);
        let hit_right = self.right.hit(&ray, t_range);
        match (hit_left, hit_right) {
            (Some(l), Some(r)) if l.t < r.t => Some(l),
            (Some(_), Some(r)) => Some(r),
            (Some(l), _) => Some(l),
            (_, Some(r)) => Some(r),
            _ => None
        }
    }

    fn bounding_box(&self, _t_range: &std::ops::Range<f32>) -> Aabb {
        Aabb { min: self.bbox.min, max: self.bbox.max }
    }
}

impl BvhNode {
    pub fn compute_shapes_bvh(mut shapes: Vec<Box<Shape>>, t_range: &std::ops::Range<f32>) -> Box<Shape> {
        let getter = match random::thread_rng().gen_range(0,3) {
            0 => V3::get_x,
            1 => V3::get_y,
            _ => V3::get_z
        };
        shapes.sort_by(|a, b|
            f32_cmp(
                getter(&a.bounding_box(t_range).min),
                getter(&b.bounding_box(t_range).min)));
        let build_bhv = |left: Box<Shape>, right: Box<Shape>| {
            let bbox = left.bounding_box(t_range)
                .union(&right.bounding_box(t_range));
            Box::new(BvhNode { left, right, bbox })
        };
        match shapes.len() {
            1 => shapes.pop().unwrap(),
            2 => {
                let snd = shapes.pop().unwrap();
                let fst = shapes.pop().unwrap();
                build_bhv(fst, snd)
            },
            _ => {
                let r = shapes.split_off(shapes.len()/2); // modifies 'shapes'!!!
                build_bhv(BvhNode::compute_shapes_bvh(shapes, t_range),
                    BvhNode::compute_shapes_bvh(r, t_range))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::material::*;

    #[test]
    fn test_bvh_node_compute_shapes() {
        let range = 0.001..std::f32::MAX;
        let shape = BvhNode::compute_shapes_bvh(vec![
            Box::new(Sphere {
                center: V3 { x: 0.0, y: -1000.0, z: 0.0},
                radius: 1000.0,
                material: Box::new(Lambertian { albedo: Color { r: 0.5, g: 0.5, b: 0.5}})
            })
        ], &range);
        let bb = shape.bounding_box(&range);
        assert_eq!(V3 { x: 0.0, y: 0.0, z: 0.0}, bb.min);
        assert_eq!(V3 { x: 0.0, y: 0.0, z: 0.0}, bb.max);
    }
}