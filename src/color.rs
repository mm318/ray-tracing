use super::utils;
use super::utils::RayTracingFloat;
use super::vec3;

pub type Color = vec3::Vec3<RayTracingFloat>;

impl Color {
    fn r(&self) -> RayTracingFloat {
        return self.x();
    }

    fn g(&self) -> RayTracingFloat {
        return self.y();
    }

    fn b(&self) -> RayTracingFloat {
        return self.z();
    }
}

pub fn write_color(pixel_color: &Color, samples_per_pixel: &usize) -> rgb::RGBA8 {
    let samples = *samples_per_pixel as RayTracingFloat;

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let r = (pixel_color.r() / samples).sqrt();
    let g = (pixel_color.g() / samples).sqrt();
    let b = (pixel_color.b() / samples).sqrt();

    // Write the translated [0,255] value of each color component.
    let ir = (256.0 * utils::clamp(r, 0.0, 0.999)) as u8;
    let ig = (256.0 * utils::clamp(g, 0.0, 0.999)) as u8;
    let ib = (256.0 * utils::clamp(b, 0.0, 0.999)) as u8;

    return rgb::RGBA8::new(ir, ig, ib, std::u8::MAX);
}
