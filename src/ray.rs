use super::vec3;

pub type Point = vec3::Vec3<f32>;
pub type Vector = vec3::Vec3<f32>;

pub struct Ray {
    orig: Point,
    dir: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        return Ray {
            orig: origin,
            dir: direction,
        };
    }

    pub fn origin(&self) -> &Point {
        return &self.orig;
    }

    pub fn direction(&self) -> &Vector {
        return &self.dir;
    }

    pub fn at(&self, t: &f32) -> Point {
        return self.origin() + self.direction() * t.clone();
    }
}
