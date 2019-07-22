use crate::{v3color::*, shapes::*};

use std::f32::consts::PI;
use rand::{prelude as random, Rng};

pub struct Camera {
    origin: V3,
    lower_left_corner: V3,
    horizontal: V3,
    vertical: V3,
    u: V3,
    v: V3,
    lens_radius: f32,
    time1: f32,
    time2: f32
}

fn random_in_unit_disk() -> V3 {
    let mut rng = random::thread_rng();
    let mut p;
    let unit = V3 { x: 1.0, y: 1.0, z: 0.0};
    loop {
        p = 2.0 * V3 { 
            x: rng.gen::<f32>(),
            y: rng.gen::<f32>(),
            z: 0.0
        } - unit;
        if p.squared_length() < 1.0 { break p; }
    }
}

pub struct CameraParams<'a> {
    pub look_from: &'a V3,
    pub look_at: &'a V3,
    pub vup: &'a V3,
    pub vert_fov_deg: f32,
    pub aspect: f32,
    pub aperture: f32,
    pub focus_dist: f32,
    pub time1: f32, // shutter open time
    pub time2: f32  // shutter close time
}

impl Camera {
    pub fn new(params: CameraParams) -> Camera {
        let theta = params.vert_fov_deg*PI/180.0;
        let half_height = f32::tan(theta/2.0)*params.focus_dist;
        let half_width = params.aspect * half_height;
        let w = (params.look_from - params.look_at).unit();
        let u = V3::cross(&params.vup, &w).unit();
        let v = V3::cross(&w, &u);
        Camera {
            origin: *params.look_from,
            lower_left_corner: params.look_from - half_width*u - half_height*v - params.focus_dist*w,
            u, v,
            horizontal: 2.0*half_width*u,
            vertical: 2.0*half_height*v,
            lens_radius: params.aperture/2.0,
            time1: params.time1,
            time2: params.time2
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius*random_in_unit_disk();
        let offset = self.u*rd.x + self.v*rd.y;
        let time: f32 = self.time1 + rand::thread_rng().gen::<f32>()*(self.time2-self.time1);
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner 
                + s*self.horizontal + t*self.vertical - self.origin - offset,
            time
        }
    }
}