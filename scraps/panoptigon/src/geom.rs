#![allow(dead_code)]

use std::ops;

#[derive(Copy, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}
impl Vec2 {
    pub fn zero() -> Self { Self { x: 0.0, y: 0.0 } }
    pub fn new(x: f64, y: f64) -> Self { Self { x, y } }
    pub fn from_array(a: [f64; 2]) -> Self { Self { x: a[0], y: a[1] } }

    pub fn sq_length(self) -> f64 { Self::dot(self, self) }
    pub fn length(self) -> f64 { self.sq_length().sqrt() }

    pub fn dot(a: Vec2, b: Vec2) -> f64 { a.x*b.x + a.y*b.y }
    pub fn det(a: Vec2, b: Vec2) -> f64 { a.x*b.y - a.y*b.x }

    pub fn normalized(self) -> Vec2 {
        let len = self.length();
        if len == 0.0 { self } else { self / len }
    }

    pub fn as_vec(self) -> Vec3 { Vec3::new(self.x, self.y, 0.0) }
    pub fn as_pt(self) -> Vec3 { Vec3::new(self.x, self.y, 1.0) }

    pub fn to_arr_f32(self) -> [f32; 2] { [self.x as f32, self.y as f32] }
    pub fn to_arr_f64(self) -> [f64; 2] { [self.x as f64, self.y as f64] }
}

impl ops::Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self { Self { x: -self.x, y: -self.y } }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Self;
    fn add(self, b: Self) -> Self { Self { x: self.x + b.x, y: self.y + b.y } }
}
impl ops::Sub<Vec2> for Vec2 {
    type Output = Self;
    fn sub(self, b: Self) -> Self { Self { x: self.x - b.x, y: self.y - b.y } }
}
impl ops::Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(self, b: f64) -> Self { Self { x: self.x * b, y: self.y * b } }
}
impl ops::Mul<Vec2> for f64 {
    type Output = Vec2;
    fn mul(self, b: Vec2) -> Vec2 { Vec2 { x: self * b.x, y: self * b.y } }
}
impl ops::Div<f64> for Vec2 {
    type Output = Self;
    fn div(self, b: f64) -> Self { Self { x: self.x / b, y: self.y / b } }
}


#[derive(Copy, Clone, PartialEq)]
pub struct Frame2 {
    pub origin: Vec2,
    pub u: Vec2,
    pub v: Vec2,
}
impl Frame2 {
    pub fn identity() -> Self {
        Self {
            origin: Vec2::zero(),
            u: Vec2::new(1.0, 0.0),
            v: Vec2::new(0.0, 1.0),
        }
    }

    pub fn to_local_vector(self, vec: Vec2) -> Vec2 {
        let det_uv = Vec2::det(self.u, self.v);
        Vec2 {
            x: Vec2::det(vec, self.v) / det_uv,
            y: Vec2::det(self.u, vec) / det_uv,
        }
    }

    pub fn inverse(self) -> Self {
        // This could be optimized to not recompute det(u,v).
        // Not sure if the compiler will help with that.
        Self { 
            origin: self.to_local_vector(-self.origin), 
            u: self.to_local_vector(Vec2::new(1.0, 0.0)), 
            v: self.to_local_vector(Vec2::new(0.0, 1.0)), 
        }
    }

    pub fn local_dilated(self, s: f64) -> Self {
        Self {
            origin: self.origin,
            u: self.u * s,
            v: self.v * s,
        }
    }
}

impl ops::Mul<Vec3> for Frame2 {
    type Output = Vec3;
    fn mul(self, b: Vec3) -> Vec3 {
        Vec3 {
            x: self.origin.x * b.z + self.u.x * b.x + self.v.x * b.y,
            y: self.origin.y * b.z + self.u.y * b.x + self.v.y * b.y,
            z: b.z,
        }
    }
}
impl ops::Mul<Frame2> for Frame2 {
    type Output = Self;
    fn mul(self, b: Self) -> Self {
        Self {
            origin: self.origin + self.u * b.origin.x + self.v * b.origin.y,
            u: self.u * b.u.x + self.v * b.u.y,
            v: self.u * b.v.x + self.v * b.v.y,
        }
    }
}


#[derive(Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vec3 {
    pub fn zero() -> Self { Self { x: 0.0, y: 0.0, z: 0.0 } }
    pub fn new(x: f64, y: f64, z: f64) -> Self { Self { x, y, z } }
    pub fn from_array(a: [f64; 3]) -> Self { Self { x: a[0], y: a[1], z: a[2] } }

    pub fn xy(self) -> Vec2 { Vec2::new(self.x, self.y) }
    pub fn xz(self) -> Vec2 { Vec2::new(self.x, self.z) }
    pub fn yz(self) -> Vec2 { Vec2::new(self.y, self.z) }

    pub fn sq_length(self) -> f64 { Self::dot(self, self) }
    pub fn length(self) -> f64 { self.sq_length().sqrt() }

    pub fn dot(a: Vec3, b: Vec3) -> f64 { a.x*b.x + a.y*b.y + a.z*b.z }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self { Self { x: -self.x, y: -self.y, z: -self.z } }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, b: Self) -> Self { Self { x: self.x + b.x, y: self.y + b.y, z: self.z + b.z } }
}
impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, b: Self) -> Self { Self { x: self.x - b.x, y: self.y - b.y, z: self.z - b.z } }
}
impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, b: f64) -> Self { Self { x: self.x * b, y: self.y * b, z: self.z * b } }
}
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, b: Vec3) -> Vec3 { Vec3 { x: self * b.x, y: self * b.y, z: self * b.z } }
}
impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, b: f64) -> Self { Self { x: self.x / b, y: self.y / b, z: self.z / b } }
}