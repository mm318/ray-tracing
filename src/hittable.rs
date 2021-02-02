use super::aabb;
use super::material;
use super::ray;
use super::utils;
use super::utils::RayTracingFloat;
use super::vec3;

pub struct HitRecord {
    pub p: ray::Point,
    normal: ray::Vector,
    pub mat: std::rc::Weak<dyn material::Material>,
    pub t: RayTracingFloat,
    pub u: RayTracingFloat,
    pub v: RayTracingFloat,
    front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        return Self {
            p: ray::Point::zero(),
            normal: ray::Vector::zero(),
            mat: std::rc::Weak::<material::Metal>::new(),
            t: RayTracingFloat::MIN,
            u: RayTracingFloat::MIN,
            v: RayTracingFloat::MIN,
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
    objects: Vec<std::rc::Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new_empty() -> Self {
        return Self {
            objects: Vec::new(),
        };
    }

    pub fn new(object: std::rc::Rc<dyn Hittable>) -> Self {
        return Self {
            objects: vec![object],
        };
    }

    pub fn get_objects(&self) -> &Vec<std::rc::Rc<dyn Hittable>> {
        return &self.objects;
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: std::rc::Rc<dyn Hittable>) {
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
    pub fn new_from_hittable_list(
        list: HittableList,
        time0: &RayTracingFloat,
        time1: &RayTracingFloat,
    ) -> Self {
        let objects = list.get_objects();
        return Self::new(objects, &0, &objects.len(), time0, time1);
    }

    pub fn new(
        src_objects: &Vec<std::rc::Rc<dyn Hittable>>,
        start: &usize,
        end: &usize,
        time0: &RayTracingFloat,
        time1: &RayTracingFloat,
    ) -> Self {
        let axis = utils::random_int(&0, &2);
        let comparator = match axis {
            0 => aabb::box_x_compare,
            1 => aabb::box_y_compare,
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

            let mid = objects.len() / 2;
            (
                std::rc::Rc::new(BVH_Node::new(&objects, &0, &mid, time0, time1))
                    as std::rc::Rc<dyn Hittable>,
                std::rc::Rc::new(BVH_Node::new(&objects, &mid, &objects.len(), time0, time1))
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

        return Self {
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
        // let hit_right = self.right.hit(r, t_min, t_max, rec);
        let right_t_max = if hit_left { rec.t } else { *t_max };
        let hit_right = self.right.hit(r, t_min, &right_t_max, rec);

        return hit_left || hit_right;
    }

    fn bounding_box(
        &self,
        _time0: &RayTracingFloat,
        _time1: &RayTracingFloat,
        output_box: &mut aabb::AxisAlignedBoundingBoxes,
    ) -> bool {
        *output_box = self.bounding_box.clone();
        return true;
    }
}
