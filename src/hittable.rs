use super::aabb;
use super::material;
use super::ray;
use super::utils;
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

    fn bounding_box(
        &self,
        time0: &RayTracingFloat,
        time1: &RayTracingFloat,
        output_box: &mut aabb::AxisAlignedBoundingBoxes,
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

    fn bounding_box(
        &self,
        time0: &RayTracingFloat,
        time1: &RayTracingFloat,
        output_box: &mut aabb::AxisAlignedBoundingBoxes,
    ) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        let mut temp_box =
            aabb::AxisAlignedBoundingBoxes::new(ray::Point::zero(), ray::Point::zero());
        let mut first_box = false;
        for object in &self.objects {
            if !object.bounding_box(time0, time1, &mut temp_box) {
                return false;
            }

            *output_box = if first_box {
                temp_box.clone()
            } else {
                aabb::surrounding_box(output_box, &temp_box)
            };

            first_box = false;
        }

        return true;
    }
}

//
// Bounding Volume Hierarchies Node
//
pub struct BVH_Node {
    left: std::rc::Rc<dyn Hittable>,
    right: std::rc::Rc<dyn Hittable>,
    bounding_box: aabb::AxisAlignedBoundingBoxes,
}

impl BVH_Node {
    pub fn new(
        src_objects: &Vec<std::rc::Rc<dyn Hittable>>,
        start: &usize,
        end: &usize,
        time0: &RayTracingFloat,
        time1: &RayTracingFloat,
    ) -> BVH_Node {
        let axis = utils::random_int(&0, &2);
        let comparator = match axis {
            1 => aabb::box_x_compare,
            2 => aabb::box_y_compare,
            _ => aabb::box_z_compare,
        };

        let object_span = end - start;
        let (left, right) = if object_span == 1 {
            (src_objects[*start].clone(), src_objects[*start].clone())
        } else if object_span == 2 {
            if comparator(&*src_objects[*start], &*src_objects[start + 1])
                == std::cmp::Ordering::Less
            {
                (src_objects[*start].clone(), src_objects[start + 1].clone())
            } else {
                (src_objects[start + 1].clone(), src_objects[*start].clone())
            }
        } else {
            let mut objects = src_objects[*start..*end].to_vec();
            objects.sort_by(|a, b| comparator(&**a, &**b));

            let mid = start + object_span / 2;
            (
                std::rc::Rc::new(BVH_Node::new(&objects, &start, &mid, time0, time1))
                    as std::rc::Rc<dyn Hittable>,
                std::rc::Rc::new(BVH_Node::new(&objects, &mid, &end, time0, time1))
                    as std::rc::Rc<dyn Hittable>,
            )
        };

        let mut box_left =
            aabb::AxisAlignedBoundingBoxes::new(ray::Point::zero(), ray::Point::zero());
        let mut box_right =
            aabb::AxisAlignedBoundingBoxes::new(ray::Point::zero(), ray::Point::zero());
        if !left.bounding_box(&0.0, &0.0, &mut box_left)
            || !right.bounding_box(&0.0, &0.0, &mut box_right)
        {
            eprintln!("No bounding box in bvh_node constructor.");
        }

        return BVH_Node {
            left: left,
            right: right,
            bounding_box: aabb::surrounding_box(&box_left, &box_right),
        };
    }
}

impl Hittable for BVH_Node {
    fn hit(
        &self,
        r: &ray::Ray,
        t_min: &RayTracingFloat,
        t_max: &RayTracingFloat,
        rec: &mut HitRecord,
    ) -> bool {
        if !self.bounding_box.hit(r, t_min, t_max) {
            return false;
        }

        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let right_t_max = if hit_left { rec.t } else { *t_max };
        let hit_right = self.right.hit(r, t_min, &right_t_max, rec);

        return hit_left || hit_right;
    }

    fn bounding_box(
        &self,
        time0: &RayTracingFloat,
        time1: &RayTracingFloat,
        output_box: &mut aabb::AxisAlignedBoundingBoxes,
    ) -> bool {
        *output_box = self.bounding_box.clone();
        return true;
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

    fn bounding_box(
        &self,
        time0: &RayTracingFloat,
        time1: &RayTracingFloat,
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
