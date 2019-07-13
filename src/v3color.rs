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

impl ops::Add<V3> for V3 {
    type Output = V3;

    fn add(self, rhs: V3) -> V3 {
        V3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::Mul<V3> for f32 {
    type Output = V3;

    fn mul(self, rhs: V3) -> V3 {
        V3 {
            x: self*rhs.x,
            y: self*rhs.y,
            z: self*rhs.z
        }
    }
}

pub fn as_color(vec: V3) -> Color {
    Color {
        r: vec.x,
        g: vec.y,
        b: vec.z
    }
}

pub fn unit_vector(vec: V3) -> V3 {
    let norm =  f32::sqrt(vec.x * vec.x
        + vec.y * vec.y
        + vec.z * vec.z);
    (1.0/norm) * vec
}
