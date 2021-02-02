use super::color;
use super::hittable;
use super::ray;
use super::texture;
use super::utils;
use super::utils::RayTracingFloat;
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
    albedo: std::rc::Rc<dyn texture::Texture>,
}

impl Lambertian {
    pub fn new(color: color::Color) -> Self {
        return Self {
            albedo: std::rc::Rc::new(texture::SolidColor::new(color)),
        };
    }

    pub fn new_with_texture(a: std::rc::Rc<dyn texture::Texture>) -> Self {
        return Self { albedo: a };
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
        //// let mut scatter_direction = rec.point() + rec.normal() + ray::Vector::random_unit_vector();
        let mut scatter_direction = rec.normal() + ray::Vector::random_unit_vector();
        // let mut scatter_direction = rec.normal() + ray::Vector::random_in_unit_sphere();
        // let mut scatter_direction = ray::Vector::random_in_hemisphere(rec.normal());

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal().clone();
        }
        *scattered = ray::Ray::new(rec.point().clone(), scatter_direction, r_in.time().clone());
        *attenuation = self.albedo.value(&rec.u, &rec.v, &rec.p).clone();
        return true;
    }
}

//
// Metal
//
pub struct Metal {
    albedo: color::Color,
    fuzz: RayTracingFloat,
}

impl Metal {
    pub fn new(color: color::Color, f: RayTracingFloat) -> Self {
        return Self {
            albedo: color,
            fuzz: utils::clamp(f, 0.0, 1.0),
        };
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
        let mut reflected = ray::reflect(&r_in.direction().unit_vector(), rec.normal());
        if self.fuzz > 0.0 {
            reflected += ray::Vector::random_in_unit_sphere() * self.fuzz;
        }
        *scattered = ray::Ray::new(rec.point().clone(), reflected, r_in.time().clone());
        *attenuation = self.albedo.clone();
        return vec3::dot(scattered.direction(), rec.normal()) > 0.0;
    }
}

//
// Dielectric
//
pub struct Dielectric {
    ir: RayTracingFloat, // index of refraction
}

impl Dielectric {
    pub fn new(index_of_refraction: RayTracingFloat) -> Self {
        return Self {
            ir: index_of_refraction,
        };
    }

    fn reflectance(cosine: RayTracingFloat, ref_idx: RayTracingFloat) -> RayTracingFloat {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &ray::Ray,
        rec: &hittable::HitRecord,
        attenuation: &mut color::Color,
        scattered: &mut ray::Ray,
    ) -> bool {
        *attenuation = color::Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if *rec.front_face() {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = vec3::dot(&-(&unit_direction), rec.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0
            || Self::reflectance(cos_theta, refraction_ratio) > utils::random_double(&0.0, &1.0);
        let direction = if cannot_refract {
            ray::reflect(&unit_direction, rec.normal())
        } else {
            ray::refract(&unit_direction, rec.normal(), &refraction_ratio)
        };
        *scattered = ray::Ray::new(rec.point().clone(), direction, r_in.time().clone());

        return true;
    }
}
