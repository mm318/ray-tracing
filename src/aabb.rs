use super::ray;
use super::utils::RayTracingFloat;

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
            let t0 = ((self.min()[a] - r.origin()[a]) / r.direction()[a])
                .min((self.max()[a] - r.origin()[a]) / r.direction()[a]);
            let t1 = ((self.min()[a] - r.origin()[a]) / r.direction()[a])
                .max((self.max()[a] - r.origin()[a]) / r.direction()[a]);
            let tmin = t0.max(*t_min);
            let tmax = t1.min(*t_max);
            if tmax <= tmin {
                return false;
            }
        }
        return true;
    }
}
