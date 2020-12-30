use super::ray;
use super::vec3;

#[derive(Clone)]
pub struct Camera {
    origin: ray::Point,
    lower_left_corner: ray::Point,
    horizontal: ray::Vector,
    vertical: ray::Vector,
    u: ray::Vector,
    v: ray::Vector,
    w: ray::Vector,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: &ray::Point,
        lookat: &ray::Point,
        vup: &ray::Vector,
        vfov: &f32, // vertical field-of-view in degrees
        aspect_ratio: &f32,
        aperture: &f32,
        focus_dist: &f32,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vec3::cross(vup, &w).unit_vector();
        let v = vec3::cross(&w, &u);

        let origin = lookfrom.clone();
        let horizontal = &u * focus_dist * viewport_width;
        let vertical = &v * focus_dist * viewport_height;
        let lower_left_corner = &origin - &horizontal / 2.0 - &vertical / 2.0 - &w * focus_dist;

        return Self {
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
            u: u,
            v: v,
            w: w,
            lens_radius: aperture / 2.0,
        };
    }

    pub fn get_ray(&self, s: &f32, t: &f32) -> ray::Ray {
        let rd = ray::Vector::random_in_unit_disk() * self.lens_radius;
        let offset = &self.u * rd.x() + &self.v * rd.y();

        return ray::Ray::new(
            &self.origin + &offset,
            &self.lower_left_corner + &self.horizontal * s + &self.vertical * t
                - &self.origin
                - &offset,
        );
    }
}
