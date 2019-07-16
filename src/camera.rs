use crate::{v3color::*, shapes::*};
use std::f32::consts::PI;

pub struct Camera {
    pub origin: V3,
    pub lower_left_corner: V3,
    pub horizontal: V3,
    pub vertical: V3
}

impl Camera {
    pub fn new(look_from: &V3, look_at: &V3,
               vup: &V3, vert_fov_deg: f32, aspect: f32) -> Camera {
        let theta = vert_fov_deg*PI/180.0;
        let half_height = f32::tan(theta/2.0);
        let half_width = aspect * half_height;
        let w = (look_from - look_at).unit();
        let u = V3::cross(&vup, &w).unit();
        let v = V3::cross(&w, &u);
        Camera {
            origin: *look_from,
            lower_left_corner: look_from - half_width*u - half_height*v - w,
            horizontal: 2.0*half_width*u,
            vertical: 2.0*half_height*v
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner 
                + s*self.horizontal + t*self.vertical - self.origin
        }
    }
}