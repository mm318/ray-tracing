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

//
// XZ Rect
//
pub struct XZ_Rect {
    mp: std::rc::Rc<dyn material::Material>,
    x0: RayTracingFloat,
    x1: RayTracingFloat,
    z0: RayTracingFloat,
    z1: RayTracingFloat,
    k: RayTracingFloat,
}

impl XZ_Rect {
    pub fn new(
        _x0: RayTracingFloat,
        _x1: RayTracingFloat,
        _z0: RayTracingFloat,
        _z1: RayTracingFloat,
        _k: RayTracingFloat,
        mat: std::rc::Rc<dyn material::Material>,
    ) -> Self {
        return Self {
            mp: mat,
            x0: _x0,
            x1: _x1,
            z0: _z0,
            z1: _z1,
            k: _k,
        };
    }
}

impl hittable::Hittable for XZ_Rect {
    fn hit(
        &self,
        r: &ray::Ray,
        t_min: &RayTracingFloat,
        t_max: &RayTracingFloat,
        rec: &mut hittable::HitRecord,
    ) -> bool {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < *t_min || t > *t_max {
            return false;
        }

        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }

        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        rec.set_face_normal(r, &ray::Vector::new(0.0, 1.0, 0.0));
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
        // The bounding box must have non-zero width in each dimension, so pad the Y
        // dimension a small amount.
        *output_box = aabb::AxisAlignedBoundingBoxes::new(
            ray::Point::new(self.x0, self.k - 0.0001, self.z0),
            ray::Point::new(self.x1, self.k + 0.0001, self.z1),
        );
        return true;
    }
}

//
// YZ Rect
//
pub struct YZ_Rect {
    mp: std::rc::Rc<dyn material::Material>,
    y0: RayTracingFloat,
    y1: RayTracingFloat,
    z0: RayTracingFloat,
    z1: RayTracingFloat,
    k: RayTracingFloat,
}

impl YZ_Rect {
    pub fn new(
        _y0: RayTracingFloat,
        _y1: RayTracingFloat,
        _z0: RayTracingFloat,
        _z1: RayTracingFloat,
        _k: RayTracingFloat,
        mat: std::rc::Rc<dyn material::Material>,
    ) -> Self {
        return Self {
            mp: mat,
            y0: _y0,
            y1: _y1,
            z0: _z0,
            z1: _z1,
            k: _k,
        };
    }
}

impl hittable::Hittable for YZ_Rect {
    fn hit(
        &self,
        r: &ray::Ray,
        t_min: &RayTracingFloat,
        t_max: &RayTracingFloat,
        rec: &mut hittable::HitRecord,
    ) -> bool {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < *t_min || t > *t_max {
            return false;
        }

        let y = r.origin().y() + t * r.direction().y();
        let z = r.origin().z() + t * r.direction().z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }

        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        rec.set_face_normal(r, &ray::Vector::new(1.0, 0.0, 0.0));
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
        // The bounding box must have non-zero width in each dimension, so pad the Y
        // dimension a small amount.
        *output_box = aabb::AxisAlignedBoundingBoxes::new(
            ray::Point::new(self.k - 0.0001, self.y0, self.z0),
            ray::Point::new(self.k + 0.0001, self.y1, self.z1),
        );
        return true;
    }
}

//
// Box
//
pub struct Box {
    box_min: ray::Point,
    box_max: ray::Point,
    sides: hittable::HittableList,
}

impl Box {
    pub fn new(p0: ray::Point, p1: ray::Point, ptr: std::rc::Rc<dyn material::Material>) -> Self {
        let mut sides = hittable::HittableList::new_empty();

        sides.add(std::rc::Rc::new(XY_Rect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            ptr.clone(),
        )));
        sides.add(std::rc::Rc::new(XY_Rect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            ptr.clone(),
        )));

        sides.add(std::rc::Rc::new(XZ_Rect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            ptr.clone(),
        )));
        sides.add(std::rc::Rc::new(XZ_Rect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            ptr.clone(),
        )));

        sides.add(std::rc::Rc::new(YZ_Rect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            ptr.clone(),
        )));
        sides.add(std::rc::Rc::new(YZ_Rect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            ptr,
        )));

        return Self {
            box_min: p0,
            box_max: p1,
            sides: sides,
        };
    }
}

impl hittable::Hittable for Box {
    fn hit(
        &self,
        r: &ray::Ray,
        t_min: &RayTracingFloat,
        t_max: &RayTracingFloat,
        rec: &mut hittable::HitRecord,
    ) -> bool {
        return self.sides.hit(r, t_min, t_max, rec);
    }

    fn bounding_box(
        &self,
        _time0: &RayTracingFloat,
        _time1: &RayTracingFloat,
        output_box: &mut aabb::AxisAlignedBoundingBoxes,
    ) -> bool {
        *output_box =
            aabb::AxisAlignedBoundingBoxes::new(self.box_min.clone(), self.box_max.clone());
        return true;
    }
}
