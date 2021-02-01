use super::hittable;
use super::ray;
use super::utils::RayTracingFloat;

#[derive(Clone)]
pub struct AxisAlignedBoundingBoxes {
    minimum: ray::Point,
    maximum: ray::Point,
}

impl AxisAlignedBoundingBoxes {
    pub fn new(a: ray::Point, b: ray::Point) -> AxisAlignedBoundingBoxes {
        return AxisAlignedBoundingBoxes {
            minimum: a,
            maximum: b,
        };
    }

    pub fn min(&self) -> &ray::Point {
        return &self.minimum;
    }

    pub fn max(&self) -> &ray::Point {
        return &self.maximum;
    }

    pub fn hit(&self, r: &ray::Ray, t_min: &RayTracingFloat, t_max: &RayTracingFloat) -> bool {
        for a in 0..ray::Point::NUM_DIMENSIONS {
            let invD = 1.0 / r.direction()[a];
            let mut t0 = (self.min()[a] - r.origin()[a]) * invD;
            let mut t1 = (self.max()[a] - r.origin()[a]) * invD;
            if invD < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let tmin = if t0 > *t_min { t0 } else { *t_min };
            let tmax = if t1 < *t_max { t1 } else { *t_max };
            if tmax <= tmin {
                return false;
            }
        }
        return true;
    }
}

pub fn surrounding_box(
    box0: &AxisAlignedBoundingBoxes,
    box1: &AxisAlignedBoundingBoxes,
) -> AxisAlignedBoundingBoxes {
    let small = ray::Point::new(
        box0.min().x().min(box1.min().x()),
        box0.min().y().min(box1.min().y()),
        box0.min().z().min(box1.min().z()),
    );

    let big = ray::Point::new(
        box0.max().x().min(box1.max().x()),
        box0.max().y().min(box1.max().y()),
        box0.max().z().min(box1.max().z()),
    );

    return AxisAlignedBoundingBoxes::new(small, big);
}

fn box_compare(
    a: &dyn hittable::Hittable,
    b: &dyn hittable::Hittable,
    axis: usize,
) -> std::cmp::Ordering {
    let mut box_a = AxisAlignedBoundingBoxes::new(ray::Point::zero(), ray::Point::zero());
    let mut box_b = AxisAlignedBoundingBoxes::new(ray::Point::zero(), ray::Point::zero());

    if !a.bounding_box(&0.0, &0.0, &mut box_a) || !b.bounding_box(&0.0, &0.0, &mut box_b) {
        eprintln!("No bounding box in bvh_node constructor.");
    }

    return box_a.min()[axis].partial_cmp(&box_b.min()[axis]).unwrap();
}

pub fn box_x_compare(a: &dyn hittable::Hittable, b: &dyn hittable::Hittable) -> std::cmp::Ordering {
    return box_compare(a, b, 0);
}

pub fn box_y_compare(a: &dyn hittable::Hittable, b: &dyn hittable::Hittable) -> std::cmp::Ordering {
    return box_compare(a, b, 1);
}

pub fn box_z_compare(a: &dyn hittable::Hittable, b: &dyn hittable::Hittable) -> std::cmp::Ordering {
    return box_compare(a, b, 2);
}
