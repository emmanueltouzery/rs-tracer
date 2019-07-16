use crate::{v3color::*, shapes::*};
use std::f32::consts::PI;

pub struct Camera {
    pub lower_left_corner: V3,
    pub horizontal: V3,
    pub vertical: V3,
    pub origin: V3
}

impl Camera {
    pub fn new(vert_fov_deg: f32, aspect: f32) -> Camera {
        let theta = vert_fov_deg*PI/180.0;
        let half_height = f32::tan(theta/2.0);
        let half_width = aspect * half_height;
        Camera {
            lower_left_corner: V3 { x: -half_width, y: -half_height, z: -1.0 },
            horizontal: V3 { x: 2.0*half_width, y: 0.0, z: 0.0 },
            vertical: V3 { x: 0.0, y: 2.0*half_height, z: 0.0 },
            origin: V3 { x: 0.0, y: 0.0, z: 0.0 }
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner 
                + u*self.horizontal + v*self.vertical - self.origin
        }
    }
}