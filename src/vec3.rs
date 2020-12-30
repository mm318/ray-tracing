mod inner {
    pub trait Sqrt {
        fn square_root(&self) -> Self;
    }

    pub trait Vec3Elem:
        Clone
        + std::ops::Neg<Output = Self>
        + std::ops::Add<Output = Self>
        + std::ops::AddAssign<Self>
        + std::ops::Sub<Output = Self>
        + std::ops::SubAssign<Self>
        + std::ops::Mul<Output = Self>
        + std::ops::MulAssign<Self>
        + std::ops::Div<Output = Self>
        + std::ops::DivAssign<Self>
        + Sqrt
    {
    }
}

// TODO: make this part of a derive attribute macro
impl inner::Sqrt for f32 {
    fn square_root(&self) -> Self {
        return self.sqrt();
    }
}

// TODO: make this part of a derive attribute macro
impl inner::Vec3Elem for f32 {}

#[derive(Clone)]
pub struct Vec3<T: inner::Vec3Elem> {
    e: [T; 3],
}

impl<T: inner::Vec3Elem> Vec3<T> {
    pub fn new(e0: T, e1: T, e2: T) -> Self {
        return Self { e: [e0, e1, e2] };
    }

    pub fn x(&self) -> T {
        return self[0].clone();
    }

    pub fn y(&self) -> T {
        return self[1].clone();
    }

    pub fn z(&self) -> T {
        return self[2].clone();
    }

    pub fn length(&self) -> T {
        return self.length_squared().square_root();
    }

    pub fn length_squared(&self) -> T {
        return (self.x() * self.x()) + (self.y() * self.y()) + (self.z() * self.z());
    }

    pub fn unit_vector(&self) -> Self {
        return self / self.length();
    }
}

//
// Vec3 Index Operators
//
impl<T: inner::Vec3Elem> std::ops::Index<usize> for Vec3<T> {
    type Output = T;
    fn index(&self, i: usize) -> &Self::Output {
        return &self.e[i];
    }
}

impl<T: inner::Vec3Elem> std::ops::IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        return &mut self.e[i];
    }
}

//
// Vec3 Negate Operators
//
impl<T: inner::Vec3Elem> std::ops::Neg for Vec3<T> {
    type Output = Vec3<T>;
    fn neg(self) -> Self::Output {
        return Self::Output::new(-self.x(), -self.y(), -self.z());
    }
}

impl<T: inner::Vec3Elem> std::ops::Neg for &Vec3<T> {
    type Output = Vec3<T>;
    fn neg(self) -> Self::Output {
        return Self::Output::new(-self.x(), -self.y(), -self.z());
    }
}

//
// Vec3 Add Operators
//
impl<T: inner::Vec3Elem> std::ops::Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, other: Vec3<T>) -> Self::Output {
        return &self + &other;
    }
}

impl<T: inner::Vec3Elem> std::ops::Add<Vec3<T>> for &Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, other: Vec3<T>) -> Self::Output {
        return self + &other;
    }
}

impl<T: inner::Vec3Elem> std::ops::Add<&Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, other: &Vec3<T>) -> Self::Output {
        return &self + other;
    }
}

impl<T: inner::Vec3Elem> std::ops::Add<&Vec3<T>> for &Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, other: &Vec3<T>) -> Self::Output {
        return Self::Output::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        );
    }
}

impl<T: inner::Vec3Elem> std::ops::AddAssign<Vec3<T>> for Vec3<T> {
    fn add_assign(&mut self, other: Vec3<T>) {
        *self += &other;
    }
}

impl<T: inner::Vec3Elem> std::ops::AddAssign<&Vec3<T>> for Vec3<T> {
    fn add_assign(&mut self, other: &Vec3<T>) {
        self[0] += other.x();
        self[1] += other.y();
        self[2] += other.z();
    }
}

//
// Vec3 Subtract Operators
//
impl<T: inner::Vec3Elem> std::ops::Sub<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn sub(self, other: Vec3<T>) -> Self::Output {
        return &self - &other;
    }
}

impl<T: inner::Vec3Elem> std::ops::Sub<Vec3<T>> for &Vec3<T> {
    type Output = Vec3<T>;
    fn sub(self, other: Vec3<T>) -> Self::Output {
        return self - &other;
    }
}

impl<T: inner::Vec3Elem> std::ops::Sub<&Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn sub(self, other: &Vec3<T>) -> Self::Output {
        return &self - other;
    }
}

impl<T: inner::Vec3Elem> std::ops::Sub<&Vec3<T>> for &Vec3<T> {
    type Output = Vec3<T>;
    fn sub(self, other: &Vec3<T>) -> Self::Output {
        return Self::Output::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        );
    }
}

impl<T: inner::Vec3Elem> std::ops::SubAssign<Vec3<T>> for Vec3<T> {
    fn sub_assign(&mut self, other: Vec3<T>) {
        *self -= &other;
    }
}

impl<T: inner::Vec3Elem> std::ops::SubAssign<&Vec3<T>> for Vec3<T> {
    fn sub_assign(&mut self, other: &Vec3<T>) {
        self[0] -= other.x();
        self[1] -= other.y();
        self[2] -= other.z();
    }
}

//
// Vec3 Tensor Multiply Operators
//
impl<T: inner::Vec3Elem> std::ops::Mul<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, other: Vec3<T>) -> Self::Output {
        return &self * &other;
    }
}

impl<T: inner::Vec3Elem> std::ops::Mul<Vec3<T>> for &Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, other: Vec3<T>) -> Self::Output {
        return self * &other;
    }
}

impl<T: inner::Vec3Elem> std::ops::Mul<&Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, other: &Vec3<T>) -> Self::Output {
        return &self * other;
    }
}

impl<T: inner::Vec3Elem> std::ops::Mul<&Vec3<T>> for &Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, other: &Vec3<T>) -> Self::Output {
        return Self::Output::new(
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        );
    }
}

//
// Vec3 Scaling Operators
//
impl<T: inner::Vec3Elem> std::ops::Mul<T> for Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, other: T) -> Self::Output {
        return &self * &other;
    }
}

impl<T: inner::Vec3Elem> std::ops::Mul<T> for &Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, other: T) -> Self::Output {
        return self * &other;
    }
}

impl<T: inner::Vec3Elem> std::ops::Mul<&T> for Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, other: &T) -> Self::Output {
        return &self * other;
    }
}

impl<T: inner::Vec3Elem> std::ops::Mul<&T> for &Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, other: &T) -> Self::Output {
        return Self::Output::new(
            self.x() * other.clone(),
            self.y() * other.clone(),
            self.z() * other.clone(),
        );
    }
}

impl<T: inner::Vec3Elem> std::ops::MulAssign<&T> for Vec3<T> {
    fn mul_assign(&mut self, other: &T) {
        self[0] *= other.clone();
        self[1] *= other.clone();
        self[2] *= other.clone();
    }
}

//
// Vec3 Divide Operators
//
impl<T: inner::Vec3Elem> std::ops::Div<T> for Vec3<T> {
    type Output = Vec3<T>;
    fn div(self, other: T) -> Self::Output {
        return &self / &other;
    }
}

impl<T: inner::Vec3Elem> std::ops::Div<T> for &Vec3<T> {
    type Output = Vec3<T>;
    fn div(self, other: T) -> Self::Output {
        return self / &other;
    }
}

impl<T: inner::Vec3Elem> std::ops::Div<&T> for Vec3<T> {
    type Output = Vec3<T>;
    fn div(self, other: &T) -> Self::Output {
        return &self / other;
    }
}

impl<T: inner::Vec3Elem> std::ops::Div<&T> for &Vec3<T> {
    type Output = Vec3<T>;
    fn div(self, other: &T) -> Self::Output {
        return Self::Output::new(
            self.x() / other.clone(),
            self.y() / other.clone(),
            self.z() / other.clone(),
        );
    }
}

impl<T: inner::Vec3Elem> std::ops::DivAssign<&T> for Vec3<T> {
    fn div_assign(&mut self, other: &T) {
        self[0] /= other.clone();
        self[1] /= other.clone();
        self[2] /= other.clone();
    }
}

//
// Vec3 Vector Operations
//
pub fn dot<T: inner::Vec3Elem>(u: &Vec3<T>, v: &Vec3<T>) -> T {
    return (u.x() * v.x()) + (u.y() * v.y()) + (u.z() * v.z());
}

pub fn cross<T: inner::Vec3Elem>(u: &Vec3<T>, v: &Vec3<T>) -> Vec3<T> {
    return Vec3::new(
        u.y() * v.z() - u.z() * v.y(),
        u.z() * v.x() - u.x() * v.z(),
        u.x() * v.y() - u.y() * v.x(),
    );
}

pub fn reflect<T: inner::Vec3Elem>(v: &Vec3<T>, n: &Vec3<T>) -> Vec3<T> {
    return v - n * (dot(v, n) + dot(v, n));
}
