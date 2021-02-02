use super::aabb;
use super::hittable;
use super::material;
use super::ray;
use super::utils::RayTracingFloat;

//
// XY Rect
//
pub struct XY_Rect {
    mp: std::rc::Rc<dyn material::Material>,
    x0: RayTracingFloat,
    x1: RayTracingFloat,
    y0: RayTracingFloat,
    y1: RayTracingFloat,
    k: RayTracingFloat,
}

impl XY_Rect {
    pub fn new(
        _x0: RayTracingFloat,
        _x1: RayTracingFloat,
        _y0: RayTracingFloat,
        _y1: RayTracingFloat,
        _k: RayTracingFloat,
        mat: std::rc::Rc<dyn material::Material>,
    ) -> Self {
        return Self {
            mp: mat,
            x0: _x0,
            x1: _x1,
            y0: _y0,
            y1: _y1,
            k: _k,
        };
    }
}

impl hittable::Hittable for XY_Rect {
    fn hit(
        &self,
        r: &ray::Ray,
        t_min: &RayTracingFloat,
        t_max: &RayTracingFloat,
        rec: &mut hittable::HitRecord,
    ) -> bool {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < *t_min || t > *t_max {
            return false;
        }

        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }

        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        rec.set_face_normal(r, &ray::Vector::new(0.0, 0.0, 1.0));
        rec.mat = std::rc::Rc::downgrade(&self.mp);
        rec.p = r.at(&t);

        return true;
    }

    fn bounding_box(
        &self,
        _time0: &RayTracingFloat,
        _time1: &RayTracingFloat,
        output_box: &mut aabb::AxisAlignedBoundingBoxes,
    ) -> bool {
        // The bounding box must have non-zero width in each dimension, so pad the Z
        // dimension a small amount.
        *output_box = aabb::AxisAlignedBoundingBoxes::new(
            ray::Point::new(self.x0, self.y0, self.k - 0.0001),
            ray::Point::new(self.x1, self.y1, self.k + 0.0001),
        );
        return true;
    }
}
