use super::ray;

#[derive(Clone)]
pub struct HitRecord {
    p: ray::Point,
    normal: ray::Vector,
    t: f32
}

pub trait Hittable {
    fn hit(&self, r: &ray::Ray, t_min: &f32, t_max: &f32, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    center: ray::Point,
    radius: f32
}

impl Sphere {
    pub fn new(cen: ray::Point, r: f32) -> Self {
        return Self { center: cen, radius: r };
    }

    pub fn center(&self) -> &ray::Point {
        return &self.center;
    }

    pub fn radius(&self) -> &f32 {
        return &self.radius;
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &ray::Ray, t_min: &f32, t_max: &f32, rec: &mut HitRecord) -> bool
    {
        let oc = r.origin() - self.center();
        return false;
    }
}
