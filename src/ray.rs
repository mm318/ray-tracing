use super::utils;
use super::vec3;

pub type Point = vec3::Vec3<f64>;
pub type Vector = vec3::Vec3<f64>;

impl vec3::Vec3<f64> {
    pub fn zero() -> Self {
        return Self::new(0.0, 0.0, 0.0);
    }

    pub fn random(min: &f64, max: &f64) -> Self {
        return Self::new(
            utils::random_double(min, max),
            utils::random_double(min, max),
            utils::random_double(min, max),
        );
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Self::new(
                utils::random_double(&-1.0, &1.0),
                utils::random_double(&-1.0, &1.0),
                0.0,
            );
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random(&-1.0, &1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Self {
        return Self::random_in_unit_sphere().unit_vector();
    }

    pub fn random_in_hemisphere(normal: &Self) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if vec3::dot(&in_unit_sphere, normal) > 0.0 {
            return in_unit_sphere;
        } else {
            return -in_unit_sphere;
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        return self[0] < s && self[1] < s && self[2] < s;
    }
}

pub fn reflect(v: &Vector, n: &Vector) -> Vector {
    return v - n * 2.0 * vec3::dot(v, n);
}

pub fn refract(uv: &Vector, n: &Vector, etai_over_etat: &f64) -> Vector {
    let cos_theta = vec3::dot(&-uv, n).min(1.0);
    let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
    let r_out_parallel = n * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
    return r_out_perp + r_out_parallel;
}

pub struct Ray {
    orig: Point,
    dir: Vector,
}

impl Ray {
    pub fn zero() -> Self {
        return Self::new(Point::zero(), Vector::zero());
    }

    pub fn new(origin: Point, direction: Vector) -> Self {
        return Ray {
            orig: origin,
            dir: direction,
        };
    }

    pub fn origin(&self) -> &Point {
        return &self.orig;
    }

    pub fn direction(&self) -> &Vector {
        return &self.dir;
    }

    pub fn at(&self, t: &f64) -> Point {
        return self.origin() + self.direction() * t.clone();
    }
}
