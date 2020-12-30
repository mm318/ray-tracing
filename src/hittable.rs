use super::material;
use super::ray;
use super::vec3;

pub struct HitRecord {
    p: ray::Point,
    normal: ray::Vector,
    mat: std::rc::Weak<dyn material::Material>,
    t: f64,
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
    fn hit(&self, r: &ray::Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool;
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
    fn hit(&self, r: &ray::Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool {
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

pub struct Sphere {
    center: ray::Point,
    radius: f64,
    mat: std::rc::Rc<dyn material::Material>,
}

impl Sphere {
    pub fn new(cen: ray::Point, r: f64, m: std::rc::Rc<dyn material::Material>) -> Self {
        return Self {
            center: cen,
            radius: r,
            mat: m,
        };
    }

    pub fn center(&self) -> &ray::Point {
        return &self.center;
    }

    pub fn radius(&self) -> &f64 {
        return &self.radius;
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &ray::Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool {
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
