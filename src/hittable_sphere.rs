use super::aabb;
use super::material;
use super::ray;
use super::hittable;
use super::utils::RayTracingFloat;
use super::vec3;

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

impl hittable::Hittable for Sphere {
    fn hit(
        &self,
        r: &ray::Ray,
        t_min: &RayTracingFloat,
        t_max: &RayTracingFloat,
        rec: &mut hittable::HitRecord,
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

    fn bounding_box(
        &self,
        _time0: &RayTracingFloat,
        _time1: &RayTracingFloat,
        output_box: &mut aabb::AxisAlignedBoundingBoxes,
    ) -> bool {
        *output_box = aabb::AxisAlignedBoundingBoxes::new(
            self.center() - ray::Vector::new(*self.radius(), *self.radius(), *self.radius()),
            self.center() + ray::Vector::new(*self.radius(), *self.radius(), *self.radius()),
        );
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

impl hittable::Hittable for MovingSphere {
    fn hit(
        &self,
        r: &ray::Ray,
        t_min: &RayTracingFloat,
        t_max: &RayTracingFloat,
        rec: &mut hittable::HitRecord,
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

    fn bounding_box(
        &self,
        time0: &RayTracingFloat,
        time1: &RayTracingFloat,
        output_box: &mut aabb::AxisAlignedBoundingBoxes,
    ) -> bool {
        let box0 = aabb::AxisAlignedBoundingBoxes::new(
            self.center(time0) - ray::Vector::new(*self.radius(), *self.radius(), *self.radius()),
            self.center(time0) + ray::Vector::new(*self.radius(), *self.radius(), *self.radius()),
        );
        let box1 = aabb::AxisAlignedBoundingBoxes::new(
            self.center(time1) - ray::Vector::new(*self.radius(), *self.radius(), *self.radius()),
            self.center(time1) + ray::Vector::new(*self.radius(), *self.radius(), *self.radius()),
        );
        *output_box = aabb::surrounding_box(&box0, &box1);
        return true;
    }
}
