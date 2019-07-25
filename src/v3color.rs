use std::ops;
use std::cmp;

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

#[derive(Copy, Clone, Debug)]
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

impl_op_ex!(- |a: &V3| -> V3 {
    V3 {
        x: -a.x,
        y: -a.y,
        z: -a.z
    }
});

impl_op_ex!(- |a: &V3, b: &V3| -> V3 {
    V3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z
    }
});

impl_op_ex!(* |a: &V3, b: &V3| -> V3 {
    V3 {
        x: a.x * b.x,
        y: a.y * b.y,
        z: a.z * b.z
    }
});

impl_op_ex_commutative!(* |a: &f32, b: &V3| -> V3 {
    V3 {
        x: a*b.x,
        y: a*b.y,
        z: a*b.z
    }
});

impl_op_ex!(/ |a: &V3, b: &f32| -> V3 {
    V3 {
        x: a.x/b,
        y: a.y/b,
        z: a.z/b
    }
});

// https://www.reddit.com/r/rust/comments/29kia3/no_ord_for_f32/cilrzik/
pub fn f32_cmp(a: f32, b: f32) -> cmp::Ordering {
    a.partial_cmp(&b).unwrap_or(cmp::Ordering::Equal)
}

impl V3 {
    pub fn to_color(&self) -> Color {
        Color {
            r: self.x,
            g: self.y,
            b: self.z
        }
    }

    pub fn unit(&self) -> V3 {
        self / self.length()
    }

    pub fn squared_length(&self) -> f32 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.squared_length())
    }

    pub fn dot(v1: &V3, v2: &V3) -> f32 {
        v1.x*v2.x + v1.y*v2.y + v1.z*v2.z
    }

    pub fn cross(v1: &V3, v2: &V3) -> V3 {
        return V3 {
            x: v1.y*v2.z - v1.z*v2.y,
            y: v1.z*v2.x - v1.x*v2.z,
            z: v1.x*v2.y - v1.y*v2.x
        }
    }

    pub fn reflect(v: &V3, n: &V3) -> V3 {
        v - 2.0 * V3::dot(v, n) * n
    }

    pub fn get_x(v: &V3) -> f32 {
        v.x
    }

    pub fn get_y(v: &V3) -> f32 {
        v.y
    }

    pub fn get_z(v: &V3) -> f32 {
        v.z
    }
}
