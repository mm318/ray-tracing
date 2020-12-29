use super::ray;

#[derive(Clone)]
pub struct Camera {
    origin: ray::Point,
    lower_left_corner: ray::Point,
    horizontal: ray::Vector,
    vertical: ray::Vector,
}

impl Camera {
    pub fn new(aspect_ratio: &f32) -> Self {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = ray::Point::new(0.0, 0.0, 0.0);
        let horizontal = ray::Vector::new(viewport_width, 0.0, 0.0);
        let vertical = ray::Vector::new(0.0, viewport_height, 0.0);
        let lower_left_corner = &origin
            - &horizontal / 2.0
            - &vertical / 2.0
            - ray::Vector::new(0.0, 0.0, focal_length);

        return Self {
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
        };
    }

    pub fn get_ray(&self, u: &f32, v: &f32) -> ray::Ray {
        return ray::Ray::new(
            self.origin.clone(),
            &self.lower_left_corner + &self.horizontal * u + &self.vertical * v - &self.origin,
        );
    }
}
