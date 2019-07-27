use crate::v3color::*;

pub trait Texture: Sync {
    fn value(&self, p: &V3) -> Color;
}

pub struct ConstantTexture {
    pub color: Color
}

impl Texture for ConstantTexture {
    fn value(&self, _p: &V3) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    pub odd: Box<Texture>,
    pub even: Box<Texture>
}

impl Texture for CheckerTexture {
    fn value(&self, p: &V3) -> Color {
        let sines = f32::sin(10.0*p.x)
            * f32::sin(10.0*p.y)
            * f32::sin(10.0*p.z);
        if sines < 0.0 {
            self.odd.value(p)
        } else {
            self.even.value(p)
        }
    }
}