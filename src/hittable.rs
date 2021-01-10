use super::material;
use super::ray;
use super::utils::RayTracingFloat;
use super::vec3;

pub struct HitRecord {
    p: ray::Point,
    normal: ray::Vector,
    mat: std::rc::Weak<dyn material::Material>,
    t: RayTracingFloat,
    front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        return Self {
            p: ray::Point::new(0.0, 0.0, 0.0),
            normal: ray::Vector::new(0.0, 0.0, 0.0),
            mat: std::rc::Weak::<material::Metal>::new(),
            t: 0.0,
            front_face: false,
        };
    }

    pub fn point(&self) -> &ray::Vector {
        return &self.p;
    }

    pub fn normal(&self) -> &ray::Vector {
        return &self.normal;
    }

    pub fn front_face(&self) -> &bool {
        return &self.front_face;
    }

    pub fn material(&self) -> std::rc::Rc<dyn material::Material> {
        return self.mat.upgrade().unwrap();
    }

    pub fn set_face_normal(&mut self, r: &ray::Ray, outward_normal: &ray::Vector) {
        self.front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(
        &self,
        r: &ray::Ray,
        t_min: &RayTracingFloat,
        t_max: &RayTracingFloat,
        rec: &mut HitRecord,
    ) -> bool;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new_empty() -> Self {
        return Self {
            objects: Vec::new(),
        };
    }

    pub fn new(object: Box<dyn Hittable>) -> Self {
        return Self {
            objects: vec![object],
        };
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        r: &ray::Ray,
        t_min: &RayTracingFloat,
        t_max: &RayTracingFloat,
        rec: &mut HitRecord,
    ) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max.clone();
        for object in &self.objects {
            let mut temp_rec = HitRecord::new();
            if object.hit(r, t_min, &closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        return hit_anything;
    }
}

//
// Sphere
//
pub struct Sphere {
    center: ray::Point,
    radius: RayTracingFloat,
    mat: std::rc::Rc<dyn material::Material>,
}

impl Sphere {
    pub fn new(
        cen: ray::Point,
        r: RayTracingFloat,
        m: std::rc::Rc<dyn material::Material>,
    ) -> Self {
        return Self {
            center: cen,
            radius: r,
            mat: m,
        };
    }

    pub fn center(&self) -> &ray::Point {
        return &self.center;
    }

    pub fn radius(&self) -> &RayTracingFloat {
        return &self.radius;
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: &ray::Ray,
        t_min: &RayTracingFloat,
        t_max: &RayTracingFloat,
        rec: &mut HitRecord,
    ) -> bool {
        let oc = r.origin() - self.center();
        let a = r.direction().length_squared();
        let half_b = vec3::dot(&oc, r.direction());
        let c = oc.length_squared() - self.radius() * self.radius();

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < *t_min || *t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < *t_min || *t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(&rec.t);
        let outward_normal = (&rec.p - self.center()) / self.radius();
        rec.set_face_normal(r, &outward_normal);
        rec.mat = std::rc::Rc::downgrade(&self.mat);

        return true;
    }
}

//
// Moving Sphere
//
pub struct MovingSphere {
    center0: ray::Point,
    center1: ray::Point,
    time0: RayTracingFloat,
    time1: RayTracingFloat,
    radius: RayTracingFloat,
    mat: std::rc::Rc<dyn material::Material>,
}

impl MovingSphere {
    pub fn new(
        cen0: ray::Point,
        cen1: ray::Point,
        _time0: RayTracingFloat,
        _time1: RayTracingFloat,
        r: RayTracingFloat,
        m: std::rc::Rc<dyn material::Material>,
    ) -> Self {
        return Self {
            center0: cen0,
            center1: cen1,
            time0: _time0,
            time1: _time1,
            radius: r,
            mat: m,
        };
    }

    pub fn center(&self, time: &RayTracingFloat) -> ray::Point {
        return &self.center0
            + (&self.center1 - &self.center0)
                * ((time - &self.time0) / (&self.time1 - &self.time0));
    }

    pub fn radius(&self) -> &RayTracingFloat {
        return &self.radius;
    }
}

impl Hittable for MovingSphere {
    fn hit(
        &self,
        r: &ray::Ray,
        t_min: &RayTracingFloat,
        t_max: &RayTracingFloat,
        rec: &mut HitRecord,
    ) -> bool {
        let oc = r.origin() - self.center(r.time());
        let a = r.direction().length_squared();
        let half_b = vec3::dot(&oc, r.direction());
        let c = oc.length_squared() - self.radius() * self.radius();

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < *t_min || *t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < *t_min || *t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(&rec.t);
        let outward_normal = (&rec.p - self.center(r.time())) / self.radius();
        rec.set_face_normal(r, &outward_normal);
        rec.mat = std::rc::Rc::downgrade(&self.mat);

        return true;
    }
}
