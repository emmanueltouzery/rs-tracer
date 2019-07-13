use std::ops;

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

#[derive(Copy, Clone)]
pub struct V3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl_op_ex!(+ |a: &V3, b: &V3| -> V3 {
    V3 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z
    }
});

impl_op_ex!(- |a: &V3, b: &V3| -> V3 {
    V3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z
    }
});

impl_op_ex_commutative!(* |a: &f32, b: &V3| -> V3 {
    V3 {
        x: a*b.x,
        y: a*b.y,
        z: a*b.z
    }
});

pub fn v3_to_color(vec: &V3) -> Color {
    Color {
        r: vec.x,
        g: vec.y,
        b: vec.z
    }
}

pub fn unit_vector(vec: &V3) -> V3 {
    let norm =  f32::sqrt(vec.x * vec.x
        + vec.y * vec.y
        + vec.z * vec.z);
    (1.0/norm) * vec
}

pub fn dot(v1: &V3, v2: &V3) -> f32 {
    v1.x*v2.x + v1.y*v2.y + v1.z*v2.z
}