use crate::{v3color::*, shapes::*};

pub struct Camera {
    pub lower_left_corner: V3,
    pub horizontal: V3,
    pub vertical: V3,
    pub origin: V3
}

impl Camera {
    pub fn default() -> Camera {
        Camera {
            lower_left_corner: V3 { x: -2.0, y: -1.0, z: -1.0 },
            origin: V3 { x: 0.0, y: 0.0, z: 0.0 },
            horizontal: V3 { x: 4.0, y: 0.0, z: 0.0 },
            vertical: V3 { x: 0.0, y: 2.0, z: 0.0 }
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner 
                + u*self.horizontal + v*self.vertical
        }
    }
}