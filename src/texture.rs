use super::color;
use super::ray;
use super::utils::RayTracingFloat;

pub trait Texture {
    fn value(&self, u: &RayTracingFloat, v: &RayTracingFloat, p: &ray::Point) -> &color::Color;
}

//
// Solid Color
//
pub struct SolidColor {
    color_value: color::Color,
}

impl SolidColor {
    pub fn new(color: color::Color) -> Self {
        return Self { color_value: color };
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: &RayTracingFloat, _v: &RayTracingFloat, _p: &ray::Point) -> &color::Color {
        return &self.color_value;
    }
}

//
// Checker Texture
//
pub struct CheckerTexture {
    odd: std::rc::Rc<dyn Texture>,
    even: std::rc::Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(c1: color::Color, c2: color::Color) -> Self {
        return Self {
            even: std::rc::Rc::new(SolidColor::new(c1)),
            odd: std::rc::Rc::new(SolidColor::new(c2)),
        };
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: &RayTracingFloat, v: &RayTracingFloat, p: &ray::Point) -> &color::Color {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            return self.odd.value(u, v, p);
        } else {
            return self.even.value(u, v, p);
        }
    }
}
