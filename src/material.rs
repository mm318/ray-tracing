// use std::rc::Rc;
use super::color;
use super::hittable;
use super::ray;
use super::vec3;

pub trait Material {
    fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &hittable::HitRecord,
        attenuation: &mut color::Color,
        scattered: &mut ray::Ray,
    ) -> bool;
}

//
// Lambertian
//
pub struct Lambertian {
    albedo: color::Color,
}

impl Lambertian {
    pub fn new(color: color::Color) -> Self {
        return Self { albedo: color };
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &hittable::HitRecord,
        attenuation: &mut color::Color,
        scattered: &mut ray::Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal() + ray::Vector::random_unit_vector();
        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal().clone();
        }
        *scattered = ray::Ray::new(rec.point().clone(), scatter_direction);
        *attenuation = self.albedo.clone();
        return true;
    }
}

//
// Metal
//
pub struct Metal {
    albedo: color::Color,
}

impl Metal {
    pub fn new(color: color::Color) -> Self {
        return Self { albedo: color };
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &hittable::HitRecord,
        attenuation: &mut color::Color,
        scattered: &mut ray::Ray,
    ) -> bool {
        let reflected = vec3::reflect(&r_in.direction().unit_vector(), rec.normal());
        *scattered = ray::Ray::new(rec.point().clone(), reflected);
        *attenuation = self.albedo.clone();
        return vec3::dot(scattered.direction(), rec.normal()) > 0.0;
    }
}
