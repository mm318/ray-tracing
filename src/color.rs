use super::vec3;

pub type Color = vec3::Vec3<f32>;

impl Color {
    fn r(&self) -> f32 {
        return self.x();
    }

    fn g(&self) -> f32 {
        return self.y();
    }

    fn b(&self) -> f32 {
        return self.z();
    }
}

pub fn write_color(color: &Color) -> rgb::RGBA8 {
    let ir = (255.999 * color.r()) as u8;
    let ig = (255.999 * color.g()) as u8;
    let ib = (255.999 * color.b()) as u8;
    return rgb::RGBA8::new(ir, ig, ib, std::u8::MAX);
}
