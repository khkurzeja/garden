#![allow(dead_code)]
#![allow(unused_parens)]

use std::ops;

pub use crate::ga_traits::*;

#[derive(Copy, Clone, PartialEq)]
pub struct Vec2 {
    // Non-projective vector
    pub x: f64,
    pub y: f64,
}
impl Vec2 {
    pub fn sq_length(self) -> f64 { self.x * self.x + self.y * self.y }
    pub fn length(self) -> f64 { self.sq_length().sqrt() }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Univec {
    pub x: f64,
    pub y: f64,
    pub w: f64,
}
impl Univec {
    pub fn xy(self) -> Vec2 { Vec2{x: self.x, y: self.y} }  //**TODO: Replace Vec2 with Vga2 Univec
    pub fn unitized(self) -> Univec {
        let s = 1.0 / self.w;
        Univec { x: self.x*s, y: self.y*s, w: 1.0 }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Bivec {
    pub xy: f64,
    pub xw: f64,
    pub yw: f64,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Trivec {
    pub xyw: f64,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Evenvec {
    pub s: f64,
    pub xy: f64,
    pub xw: f64,
    pub yw: f64,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Oddvec {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub xyw: f64,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Magnitude {
    pub s: f64,
    pub xyw: f64,
}
impl Magnitude {
    pub fn value(self) -> f64 {
        return self.s / self.xyw;
    }
}

// Operator overloads
//**TODO: Add these to the code generator

impl ops::Neg for Univec {
  type Output = Univec;
  fn neg(self) -> Univec { neg(self) }
}

impl ops::Add<Univec> for Univec {
  type Output = Univec;
  fn add(self, b: Univec) -> Univec { add(self, b) }
}
impl ops::Add<Bivec> for Bivec {
  type Output = Bivec;
  fn add(self, b: Bivec) -> Bivec { add(self, b) }
}
impl ops::Add<Trivec> for Trivec {
  type Output = Trivec;
  fn add(self, b: Trivec) -> Trivec { add(self, b) }
}
impl ops::Add<Evenvec> for Evenvec {
  type Output = Evenvec;
  fn add(self, b: Evenvec) -> Evenvec { add(self, b) }
}
impl ops::Add<Oddvec> for Oddvec {
  type Output = Oddvec;
  fn add(self, b: Oddvec) -> Oddvec { add(self, b) }
}
impl ops::Add<Magnitude> for Magnitude {
  type Output = Magnitude;
  fn add(self, b: Magnitude) -> Magnitude { add(self, b) }
}

impl ops::Sub<Univec> for Univec {
  type Output = Univec;
  fn sub(self, b: Univec) -> Univec { sub(self, b) }
}
impl ops::Sub<Bivec> for Bivec {
  type Output = Bivec;
  fn sub(self, b: Bivec) -> Bivec { sub(self, b) }
}
impl ops::Sub<Trivec> for Trivec {
  type Output = Trivec;
  fn sub(self, b: Trivec) -> Trivec { sub(self, b) }
}
impl ops::Sub<Evenvec> for Evenvec {
  type Output = Evenvec;
  fn sub(self, b: Evenvec) -> Evenvec { sub(self, b) }
}
impl ops::Sub<Oddvec> for Oddvec {
  type Output = Oddvec;
  fn sub(self, b: Oddvec) -> Oddvec { sub(self, b) }
}
impl ops::Sub<Magnitude> for Magnitude {
  type Output = Magnitude;
  fn sub(self, b: Magnitude) -> Magnitude { sub(self, b) }
}

impl ops::Mul<f64> for Univec {
  type Output = Univec;
  fn mul(self, b: f64) -> Univec { mul(self, b) }
}
impl ops::Mul<f64> for Bivec {
  type Output = Bivec;
  fn mul(self, b: f64) -> Bivec { mul(self, b) }
}
impl ops::Mul<f64> for Trivec {
  type Output = Trivec;
  fn mul(self, b: f64) -> Trivec { mul(self, b) }
}
impl ops::Mul<f64> for Evenvec {
  type Output = Evenvec;
  fn mul(self, b: f64) -> Evenvec { mul(self, b) }
}
impl ops::Mul<f64> for Oddvec {
  type Output = Oddvec;
  fn mul(self, b: f64) -> Oddvec { mul(self, b) }
}
impl ops::Mul<f64> for Magnitude {
  type Output = Magnitude;
  fn mul(self, b: f64) -> Magnitude { mul(self, b) }
}

impl ops::Mul<Univec> for f64 {
  type Output = Univec;
  fn mul(self, b: Univec) -> Univec { mul(self, b) }
}
impl ops::Mul<Bivec> for f64 {
  type Output = Bivec;
  fn mul(self, b: Bivec) -> Bivec { mul(self, b) }
}
impl ops::Mul<Trivec> for f64 {
  type Output = Trivec;
  fn mul(self, b: Trivec) -> Trivec { mul(self, b) }
}
impl ops::Mul<Evenvec> for f64 {
  type Output = Evenvec;
  fn mul(self, b: Evenvec) -> Evenvec { mul(self, b) }
}
impl ops::Mul<Oddvec> for f64 {
  type Output = Oddvec;
  fn mul(self, b: Oddvec) -> Oddvec { mul(self, b) }
}
impl ops::Mul<Magnitude> for f64 {
  type Output = Magnitude;
  fn mul(self, b: Magnitude) -> Magnitude { mul(self, b) }
}

impl ops::Div<f64> for Univec {
  type Output = Univec;
  fn div(self, b: f64) -> Univec { mul(self, 1.0/b) }
}
impl ops::Div<f64> for Bivec {
  type Output = Bivec;
  fn div(self, b: f64) -> Bivec { mul(self, 1.0/b) }
}
impl ops::Div<f64> for Trivec {
  type Output = Trivec;
  fn div(self, b: f64) -> Trivec { mul(self, 1.0/b) }
}
impl ops::Div<f64> for Evenvec {
  type Output = Evenvec;
  fn div(self, b: f64) -> Evenvec { mul(self, 1.0/b) }
}
impl ops::Div<f64> for Oddvec {
  type Output = Oddvec;
  fn div(self, b: f64) -> Oddvec { mul(self, 1.0/b) }
}
impl ops::Div<f64> for Magnitude {
  type Output = Magnitude;
  fn div(self, b: f64) -> Magnitude { mul(self, 1.0/b) }
}





// --- Neg ---
//impl Neg<f64> for () {
//  type R = f64;
//  fn neg(a: f64) -> Self::R {
//    -a
//  }
//}
impl Neg<Univec> for () {
    type R = Univec;
    fn neg(a: Univec) -> Self::R {
      Self::R { x: -a.x, y: -a.y, w: -a.w }
    }
  }
  impl Neg<Bivec> for () {
    type R = Bivec;
    fn neg(a: Bivec) -> Self::R {
      Self::R { xy: -a.xy, xw: -a.xw, yw: -a.yw }
    }
  }
  impl Neg<Trivec> for () {
    type R = Trivec;
    fn neg(a: Trivec) -> Self::R {
      Self::R { xyw: -a.xyw }
    }
  }
  impl Neg<Evenvec> for () {
    type R = Evenvec;
    fn neg(a: Evenvec) -> Self::R {
      Self::R { s: -a.s, xy: -a.xy, xw: -a.xw, yw: -a.yw }
    }
  }
  impl Neg<Oddvec> for () {
    type R = Oddvec;
    fn neg(a: Oddvec) -> Self::R {
      Self::R { x: -a.x, y: -a.y, w: -a.w, xyw: -a.xyw }
    }
  }
  impl Neg<Magnitude> for () {
    type R = Magnitude;
    fn neg(a: Magnitude) -> Self::R {
      Self::R { s: -a.s, xyw: -a.xyw }
    }
  }
  // --- Add ---
  //impl Add<f64, f64> for () {
  //  type R = f64;
  //  fn add(a: f64, b: f64) -> Self::R {
  //    (a + b)
  //  }
  //}
  //impl Add<f64, Univec> for () {
  //  type R = <ERROR>;
  //  fn add(a: f64, b: Univec) -> Self::R {
  //    Self::R { s: a, x: b.x, y: b.y, w: b.w }
  //  }
  //}
  impl Add<f64, Bivec> for () {
    type R = Evenvec;
    fn add(a: f64, b: Bivec) -> Self::R {
      Self::R { s: a, xy: b.xy, xw: b.xw, yw: b.yw }
    }
  }
  impl Add<f64, Trivec> for () {
    type R = Magnitude;
    fn add(a: f64, b: Trivec) -> Self::R {
      Self::R { s: a, xyw: b.xyw }
    }
  }
  impl Add<f64, Evenvec> for () {
    type R = Evenvec;
    fn add(a: f64, b: Evenvec) -> Self::R {
      Self::R { s: (a + b.s), xy: b.xy, xw: b.xw, yw: b.yw }
    }
  }
  //impl Add<f64, Oddvec> for () {
  //  type R = <ERROR>;
  //  fn add(a: f64, b: Oddvec) -> Self::R {
  //    Self::R { s: a, x: b.x, y: b.y, w: b.w, xyw: b.xyw }
  //  }
  //}
  impl Add<f64, Magnitude> for () {
    type R = Magnitude;
    fn add(a: f64, b: Magnitude) -> Self::R {
      Self::R { s: (a + b.s), xyw: b.xyw }
    }
  }
  //impl Add<Univec, f64> for () {
  //  type R = <ERROR>;
  //  fn add(a: Univec, b: f64) -> Self::R {
  //    Self::R { s: b, x: a.x, y: a.y, w: a.w }
  //  }
  //}
  impl Add<Univec, Univec> for () {
    type R = Univec;
    fn add(a: Univec, b: Univec) -> Self::R {
      Self::R { x: (a.x + b.x), y: (a.y + b.y), w: (a.w + b.w) }
    }
  }
  //impl Add<Univec, Bivec> for () {
  //  type R = <ERROR>;
  //  fn add(a: Univec, b: Bivec) -> Self::R {
  //    Self::R { x: a.x, y: a.y, w: a.w, xy: b.xy, xw: b.xw, yw: b.yw }
  //  }
  //}
  impl Add<Univec, Trivec> for () {
    type R = Oddvec;
    fn add(a: Univec, b: Trivec) -> Self::R {
      Self::R { x: a.x, y: a.y, w: a.w, xyw: b.xyw }
    }
  }
  //impl Add<Univec, Evenvec> for () {
  //  type R = <ERROR>;
  //  fn add(a: Univec, b: Evenvec) -> Self::R {
  //    Self::R { s: b.s, x: a.x, y: a.y, w: a.w, xy: b.xy, xw: b.xw, yw: b.yw }
  //  }
  //}
  impl Add<Univec, Oddvec> for () {
    type R = Oddvec;
    fn add(a: Univec, b: Oddvec) -> Self::R {
      Self::R { x: (a.x + b.x), y: (a.y + b.y), w: (a.w + b.w), xyw: b.xyw }
    }
  }
  //impl Add<Univec, Magnitude> for () {
  //  type R = <ERROR>;
  //  fn add(a: Univec, b: Magnitude) -> Self::R {
  //    Self::R { s: b.s, x: a.x, y: a.y, w: a.w, xyw: b.xyw }
  //  }
  //}
  impl Add<Bivec, f64> for () {
    type R = Evenvec;
    fn add(a: Bivec, b: f64) -> Self::R {
      Self::R { s: b, xy: a.xy, xw: a.xw, yw: a.yw }
    }
  }
  //impl Add<Bivec, Univec> for () {
  //  type R = <ERROR>;
  //  fn add(a: Bivec, b: Univec) -> Self::R {
  //    Self::R { x: b.x, y: b.y, w: b.w, xy: a.xy, xw: a.xw, yw: a.yw }
  //  }
  //}
  impl Add<Bivec, Bivec> for () {
    type R = Bivec;
    fn add(a: Bivec, b: Bivec) -> Self::R {
      Self::R { xy: (a.xy + b.xy), xw: (a.xw + b.xw), yw: (a.yw + b.yw) }
    }
  }
  //impl Add<Bivec, Trivec> for () {
  //  type R = <ERROR>;
  //  fn add(a: Bivec, b: Trivec) -> Self::R {
  //    Self::R { xy: a.xy, xw: a.xw, yw: a.yw, xyw: b.xyw }
  //  }
  //}
  impl Add<Bivec, Evenvec> for () {
    type R = Evenvec;
    fn add(a: Bivec, b: Evenvec) -> Self::R {
      Self::R { s: b.s, xy: (a.xy + b.xy), xw: (a.xw + b.xw), yw: (a.yw + b.yw) }
    }
  }
  //impl Add<Bivec, Oddvec> for () {
  //  type R = <ERROR>;
  //  fn add(a: Bivec, b: Oddvec) -> Self::R {
  //    Self::R { x: b.x, y: b.y, w: b.w, xy: a.xy, xw: a.xw, yw: a.yw, xyw: b.xyw }
  //  }
  //}
  //impl Add<Bivec, Magnitude> for () {
  //  type R = <ERROR>;
  //  fn add(a: Bivec, b: Magnitude) -> Self::R {
  //    Self::R { s: b.s, xy: a.xy, xw: a.xw, yw: a.yw, xyw: b.xyw }
  //  }
  //}
  impl Add<Trivec, f64> for () {
    type R = Magnitude;
    fn add(a: Trivec, b: f64) -> Self::R {
      Self::R { s: b, xyw: a.xyw }
    }
  }
  impl Add<Trivec, Univec> for () {
    type R = Oddvec;
    fn add(a: Trivec, b: Univec) -> Self::R {
      Self::R { x: b.x, y: b.y, w: b.w, xyw: a.xyw }
    }
  }
  //impl Add<Trivec, Bivec> for () {
  //  type R = <ERROR>;
  //  fn add(a: Trivec, b: Bivec) -> Self::R {
  //    Self::R { xy: b.xy, xw: b.xw, yw: b.yw, xyw: a.xyw }
  //  }
  //}
  impl Add<Trivec, Trivec> for () {
    type R = Trivec;
    fn add(a: Trivec, b: Trivec) -> Self::R {
      Self::R { xyw: (a.xyw + b.xyw) }
    }
  }
  //impl Add<Trivec, Evenvec> for () {
  //  type R = <ERROR>;
  //  fn add(a: Trivec, b: Evenvec) -> Self::R {
  //    Self::R { s: b.s, xy: b.xy, xw: b.xw, yw: b.yw, xyw: a.xyw }
  //  }
  //}
  impl Add<Trivec, Oddvec> for () {
    type R = Oddvec;
    fn add(a: Trivec, b: Oddvec) -> Self::R {
      Self::R { x: b.x, y: b.y, w: b.w, xyw: (a.xyw + b.xyw) }
    }
  }
  impl Add<Trivec, Magnitude> for () {
    type R = Magnitude;
    fn add(a: Trivec, b: Magnitude) -> Self::R {
      Self::R { s: b.s, xyw: (a.xyw + b.xyw) }
    }
  }
  impl Add<Evenvec, f64> for () {
    type R = Evenvec;
    fn add(a: Evenvec, b: f64) -> Self::R {
      Self::R { s: (a.s + b), xy: a.xy, xw: a.xw, yw: a.yw }
    }
  }
  //impl Add<Evenvec, Univec> for () {
  //  type R = <ERROR>;
  //  fn add(a: Evenvec, b: Univec) -> Self::R {
  //    Self::R { s: a.s, x: b.x, y: b.y, w: b.w, xy: a.xy, xw: a.xw, yw: a.yw }
  //  }
  //}
  impl Add<Evenvec, Bivec> for () {
    type R = Evenvec;
    fn add(a: Evenvec, b: Bivec) -> Self::R {
      Self::R { s: a.s, xy: (a.xy + b.xy), xw: (a.xw + b.xw), yw: (a.yw + b.yw) }
    }
  }
  //impl Add<Evenvec, Trivec> for () {
  //  type R = <ERROR>;
  //  fn add(a: Evenvec, b: Trivec) -> Self::R {
  //    Self::R { s: a.s, xy: a.xy, xw: a.xw, yw: a.yw, xyw: b.xyw }
  //  }
  //}
  impl Add<Evenvec, Evenvec> for () {
    type R = Evenvec;
    fn add(a: Evenvec, b: Evenvec) -> Self::R {
      Self::R { s: (a.s + b.s), xy: (a.xy + b.xy), xw: (a.xw + b.xw), yw: (a.yw + b.yw) }
    }
  }
  //impl Add<Evenvec, Oddvec> for () {
  //  type R = <ERROR>;
  //  fn add(a: Evenvec, b: Oddvec) -> Self::R {
  //    Self::R { s: a.s, x: b.x, y: b.y, w: b.w, xy: a.xy, xw: a.xw, yw: a.yw, xyw: b.xyw }
  //  }
  //}
  //impl Add<Evenvec, Magnitude> for () {
  //  type R = <ERROR>;
  //  fn add(a: Evenvec, b: Magnitude) -> Self::R {
  //    Self::R { s: (a.s + b.s), xy: a.xy, xw: a.xw, yw: a.yw, xyw: b.xyw }
  //  }
  //}
  //impl Add<Oddvec, f64> for () {
  //  type R = <ERROR>;
  //  fn add(a: Oddvec, b: f64) -> Self::R {
  //    Self::R { s: b, x: a.x, y: a.y, w: a.w, xyw: a.xyw }
  //  }
  //}
  impl Add<Oddvec, Univec> for () {
    type R = Oddvec;
    fn add(a: Oddvec, b: Univec) -> Self::R {
      Self::R { x: (a.x + b.x), y: (a.y + b.y), w: (a.w + b.w), xyw: a.xyw }
    }
  }
  //impl Add<Oddvec, Bivec> for () {
  //  type R = <ERROR>;
  //  fn add(a: Oddvec, b: Bivec) -> Self::R {
  //    Self::R { x: a.x, y: a.y, w: a.w, xy: b.xy, xw: b.xw, yw: b.yw, xyw: a.xyw }
  //  }
  //}
  impl Add<Oddvec, Trivec> for () {
    type R = Oddvec;
    fn add(a: Oddvec, b: Trivec) -> Self::R {
      Self::R { x: a.x, y: a.y, w: a.w, xyw: (a.xyw + b.xyw) }
    }
  }
  //impl Add<Oddvec, Evenvec> for () {
  //  type R = <ERROR>;
  //  fn add(a: Oddvec, b: Evenvec) -> Self::R {
  //    Self::R { s: b.s, x: a.x, y: a.y, w: a.w, xy: b.xy, xw: b.xw, yw: b.yw, xyw: a.xyw }
  //  }
  //}
  impl Add<Oddvec, Oddvec> for () {
    type R = Oddvec;
    fn add(a: Oddvec, b: Oddvec) -> Self::R {
      Self::R { x: (a.x + b.x), y: (a.y + b.y), w: (a.w + b.w), xyw: (a.xyw + b.xyw) }
    }
  }
  //impl Add<Oddvec, Magnitude> for () {
  //  type R = <ERROR>;
  //  fn add(a: Oddvec, b: Magnitude) -> Self::R {
  //    Self::R { s: b.s, x: a.x, y: a.y, w: a.w, xyw: (a.xyw + b.xyw) }
  //  }
  //}
  impl Add<Magnitude, f64> for () {
    type R = Magnitude;
    fn add(a: Magnitude, b: f64) -> Self::R {
      Self::R { s: (a.s + b), xyw: a.xyw }
    }
  }
  //impl Add<Magnitude, Univec> for () {
  //  type R = <ERROR>;
  //  fn add(a: Magnitude, b: Univec) -> Self::R {
  //    Self::R { s: a.s, x: b.x, y: b.y, w: b.w, xyw: a.xyw }
  //  }
  //}
  //impl Add<Magnitude, Bivec> for () {
  //  type R = <ERROR>;
  //  fn add(a: Magnitude, b: Bivec) -> Self::R {
  //    Self::R { s: a.s, xy: b.xy, xw: b.xw, yw: b.yw, xyw: a.xyw }
  //  }
  //}
  impl Add<Magnitude, Trivec> for () {
    type R = Magnitude;
    fn add(a: Magnitude, b: Trivec) -> Self::R {
      Self::R { s: a.s, xyw: (a.xyw + b.xyw) }
    }
  }
  //impl Add<Magnitude, Evenvec> for () {
  //  type R = <ERROR>;
  //  fn add(a: Magnitude, b: Evenvec) -> Self::R {
  //    Self::R { s: (a.s + b.s), xy: b.xy, xw: b.xw, yw: b.yw, xyw: a.xyw }
  //  }
  //}
  //impl Add<Magnitude, Oddvec> for () {
  //  type R = <ERROR>;
  //  fn add(a: Magnitude, b: Oddvec) -> Self::R {
  //    Self::R { s: a.s, x: b.x, y: b.y, w: b.w, xyw: (a.xyw + b.xyw) }
  //  }
  //}
  impl Add<Magnitude, Magnitude> for () {
    type R = Magnitude;
    fn add(a: Magnitude, b: Magnitude) -> Self::R {
      Self::R { s: (a.s + b.s), xyw: (a.xyw + b.xyw) }
    }
  }
  // --- Sub ---
  //impl Sub<f64, f64> for () {
  //  type R = f64;
  //  fn sub(a: f64, b: f64) -> Self::R {
  //    (a + -b)
  //  }
  //}
  //impl Sub<f64, Univec> for () {
  //  type R = <ERROR>;
  //  fn sub(a: f64, b: Univec) -> Self::R {
  //    Self::R { s: a, x: -b.x, y: -b.y, w: -b.w }
  //  }
  //}
  impl Sub<f64, Bivec> for () {
    type R = Evenvec;
    fn sub(a: f64, b: Bivec) -> Self::R {
      Self::R { s: a, xy: -b.xy, xw: -b.xw, yw: -b.yw }
    }
  }
  impl Sub<f64, Trivec> for () {
    type R = Magnitude;
    fn sub(a: f64, b: Trivec) -> Self::R {
      Self::R { s: a, xyw: -b.xyw }
    }
  }
  impl Sub<f64, Evenvec> for () {
    type R = Evenvec;
    fn sub(a: f64, b: Evenvec) -> Self::R {
      Self::R { s: (a + -b.s), xy: -b.xy, xw: -b.xw, yw: -b.yw }
    }
  }
  //impl Sub<f64, Oddvec> for () {
  //  type R = <ERROR>;
  //  fn sub(a: f64, b: Oddvec) -> Self::R {
  //    Self::R { s: a, x: -b.x, y: -b.y, w: -b.w, xyw: -b.xyw }
  //  }
  //}
  impl Sub<f64, Magnitude> for () {
    type R = Magnitude;
    fn sub(a: f64, b: Magnitude) -> Self::R {
      Self::R { s: (a + -b.s), xyw: -b.xyw }
    }
  }
  //impl Sub<Univec, f64> for () {
  //  type R = <ERROR>;
  //  fn sub(a: Univec, b: f64) -> Self::R {
  //    Self::R { s: -b, x: a.x, y: a.y, w: a.w }
  //  }
  //}
  impl Sub<Univec, Univec> for () {
    type R = Univec;
    fn sub(a: Univec, b: Univec) -> Self::R {
      Self::R { x: (a.x + -b.x), y: (a.y + -b.y), w: (a.w + -b.w) }
    }
  }
  //impl Sub<Univec, Bivec> for () {
  //  type R = <ERROR>;
  //  fn sub(a: Univec, b: Bivec) -> Self::R {
  //    Self::R { x: a.x, y: a.y, w: a.w, xy: -b.xy, xw: -b.xw, yw: -b.yw }
  //  }
  //}
  impl Sub<Univec, Trivec> for () {
    type R = Oddvec;
    fn sub(a: Univec, b: Trivec) -> Self::R {
      Self::R { x: a.x, y: a.y, w: a.w, xyw: -b.xyw }
    }
  }
  //impl Sub<Univec, Evenvec> for () {
  //  type R = <ERROR>;
  //  fn sub(a: Univec, b: Evenvec) -> Self::R {
  //    Self::R { s: -b.s, x: a.x, y: a.y, w: a.w, xy: -b.xy, xw: -b.xw, yw: -b.yw }
  //  }
  //}
  impl Sub<Univec, Oddvec> for () {
    type R = Oddvec;
    fn sub(a: Univec, b: Oddvec) -> Self::R {
      Self::R { x: (a.x + -b.x), y: (a.y + -b.y), w: (a.w + -b.w), xyw: -b.xyw }
    }
  }
  //impl Sub<Univec, Magnitude> for () {
  //  type R = <ERROR>;
  //  fn sub(a: Univec, b: Magnitude) -> Self::R {
  //    Self::R { s: -b.s, x: a.x, y: a.y, w: a.w, xyw: -b.xyw }
  //  }
  //}
  impl Sub<Bivec, f64> for () {
    type R = Evenvec;
    fn sub(a: Bivec, b: f64) -> Self::R {
      Self::R { s: -b, xy: a.xy, xw: a.xw, yw: a.yw }
    }
  }
  //impl Sub<Bivec, Univec> for () {
  //  type R = <ERROR>;
  //  fn sub(a: Bivec, b: Univec) -> Self::R {
  //    Self::R { x: -b.x, y: -b.y, w: -b.w, xy: a.xy, xw: a.xw, yw: a.yw }
  //  }
  //}
  impl Sub<Bivec, Bivec> for () {
    type R = Bivec;
    fn sub(a: Bivec, b: Bivec) -> Self::R {
      Self::R { xy: (a.xy + -b.xy), xw: (a.xw + -b.xw), yw: (a.yw + -b.yw) }
    }
  }
  //impl Sub<Bivec, Trivec> for () {
  //  type R = <ERROR>;
  //  fn sub(a: Bivec, b: Trivec) -> Self::R {
  //    Self::R { xy: a.xy, xw: a.xw, yw: a.yw, xyw: -b.xyw }
  //  }
  //}
  impl Sub<Bivec, Evenvec> for () {
    type R = Evenvec;
    fn sub(a: Bivec, b: Evenvec) -> Self::R {
      Self::R { s: -b.s, xy: (a.xy + -b.xy), xw: (a.xw + -b.xw), yw: (a.yw + -b.yw) }
    }
  }
  //impl Sub<Bivec, Oddvec> for () {
  //  type R = <ERROR>;
  //  fn sub(a: Bivec, b: Oddvec) -> Self::R {
  //    Self::R { x: -b.x, y: -b.y, w: -b.w, xy: a.xy, xw: a.xw, yw: a.yw, xyw: -b.xyw }
  //  }
  //}
  //impl Sub<Bivec, Magnitude> for () {
  //  type R = <ERROR>;
  //  fn sub(a: Bivec, b: Magnitude) -> Self::R {
  //    Self::R { s: -b.s, xy: a.xy, xw: a.xw, yw: a.yw, xyw: -b.xyw }
  //  }
  //}
  impl Sub<Trivec, f64> for () {
    type R = Magnitude;
    fn sub(a: Trivec, b: f64) -> Self::R {
      Self::R { s: -b, xyw: a.xyw }
    }
  }
  impl Sub<Trivec, Univec> for () {
    type R = Oddvec;
    fn sub(a: Trivec, b: Univec) -> Self::R {
      Self::R { x: -b.x, y: -b.y, w: -b.w, xyw: a.xyw }
    }
  }
  //impl Sub<Trivec, Bivec> for () {
  //  type R = <ERROR>;
  //  fn sub(a: Trivec, b: Bivec) -> Self::R {
  //    Self::R { xy: -b.xy, xw: -b.xw, yw: -b.yw, xyw: a.xyw }
  //  }
  //}
  impl Sub<Trivec, Trivec> for () {
    type R = Trivec;
    fn sub(a: Trivec, b: Trivec) -> Self::R {
      Self::R { xyw: (a.xyw + -b.xyw) }
    }
  }
  //impl Sub<Trivec, Evenvec> for () {
  //  type R = <ERROR>;
  //  fn sub(a: Trivec, b: Evenvec) -> Self::R {
  //    Self::R { s: -b.s, xy: -b.xy, xw: -b.xw, yw: -b.yw, xyw: a.xyw }
  //  }
  //}
  impl Sub<Trivec, Oddvec> for () {
    type R = Oddvec;
    fn sub(a: Trivec, b: Oddvec) -> Self::R {
      Self::R { x: -b.x, y: -b.y, w: -b.w, xyw: (a.xyw + -b.xyw) }
    }
  }
  impl Sub<Trivec, Magnitude> for () {
    type R = Magnitude;
    fn sub(a: Trivec, b: Magnitude) -> Self::R {
      Self::R { s: -b.s, xyw: (a.xyw + -b.xyw) }
    }
  }
  impl Sub<Evenvec, f64> for () {
    type R = Evenvec;
    fn sub(a: Evenvec, b: f64) -> Self::R {
      Self::R { s: (a.s + -b), xy: a.xy, xw: a.xw, yw: a.yw }
    }
  }
  //impl Sub<Evenvec, Univec> for () {
  //  type R = <ERROR>;
  //  fn sub(a: Evenvec, b: Univec) -> Self::R {
  //    Self::R { s: a.s, x: -b.x, y: -b.y, w: -b.w, xy: a.xy, xw: a.xw, yw: a.yw }
  //  }
  //}
  impl Sub<Evenvec, Bivec> for () {
    type R = Evenvec;
    fn sub(a: Evenvec, b: Bivec) -> Self::R {
      Self::R { s: a.s, xy: (a.xy + -b.xy), xw: (a.xw + -b.xw), yw: (a.yw + -b.yw) }
    }
  }
  //impl Sub<Evenvec, Trivec> for () {
  //  type R = <ERROR>;
  //  fn sub(a: Evenvec, b: Trivec) -> Self::R {
  //    Self::R { s: a.s, xy: a.xy, xw: a.xw, yw: a.yw, xyw: -b.xyw }
  //  }
  //}
  impl Sub<Evenvec, Evenvec> for () {
    type R = Evenvec;
    fn sub(a: Evenvec, b: Evenvec) -> Self::R {
      Self::R { s: (a.s + -b.s), xy: (a.xy + -b.xy), xw: (a.xw + -b.xw), yw: (a.yw + -b.yw) }
    }
  }
  //impl Sub<Evenvec, Oddvec> for () {
  //  type R = <ERROR>;
  //  fn sub(a: Evenvec, b: Oddvec) -> Self::R {
  //    Self::R { s: a.s, x: -b.x, y: -b.y, w: -b.w, xy: a.xy, xw: a.xw, yw: a.yw, xyw: -b.xyw }
  //  }
  //}
  //impl Sub<Evenvec, Magnitude> for () {
  //  type R = <ERROR>;
  //  fn sub(a: Evenvec, b: Magnitude) -> Self::R {
  //    Self::R { s: (a.s + -b.s), xy: a.xy, xw: a.xw, yw: a.yw, xyw: -b.xyw }
  //  }
  //}
  //impl Sub<Oddvec, f64> for () {
  //  type R = <ERROR>;
  //  fn sub(a: Oddvec, b: f64) -> Self::R {
  //    Self::R { s: -b, x: a.x, y: a.y, w: a.w, xyw: a.xyw }
  //  }
  //}
  impl Sub<Oddvec, Univec> for () {
    type R = Oddvec;
    fn sub(a: Oddvec, b: Univec) -> Self::R {
      Self::R { x: (a.x + -b.x), y: (a.y + -b.y), w: (a.w + -b.w), xyw: a.xyw }
    }
  }
  //impl Sub<Oddvec, Bivec> for () {
  //  type R = <ERROR>;
  //  fn sub(a: Oddvec, b: Bivec) -> Self::R {
  //    Self::R { x: a.x, y: a.y, w: a.w, xy: -b.xy, xw: -b.xw, yw: -b.yw, xyw: a.xyw }
  //  }
  //}
  impl Sub<Oddvec, Trivec> for () {
    type R = Oddvec;
    fn sub(a: Oddvec, b: Trivec) -> Self::R {
      Self::R { x: a.x, y: a.y, w: a.w, xyw: (a.xyw + -b.xyw) }
    }
  }
  //impl Sub<Oddvec, Evenvec> for () {
  //  type R = <ERROR>;
  //  fn sub(a: Oddvec, b: Evenvec) -> Self::R {
  //    Self::R { s: -b.s, x: a.x, y: a.y, w: a.w, xy: -b.xy, xw: -b.xw, yw: -b.yw, xyw: a.xyw }
  //  }
  //}
  impl Sub<Oddvec, Oddvec> for () {
    type R = Oddvec;
    fn sub(a: Oddvec, b: Oddvec) -> Self::R {
      Self::R { x: (a.x + -b.x), y: (a.y + -b.y), w: (a.w + -b.w), xyw: (a.xyw + -b.xyw) }
    }
  }
  //impl Sub<Oddvec, Magnitude> for () {
  //  type R = <ERROR>;
  //  fn sub(a: Oddvec, b: Magnitude) -> Self::R {
  //    Self::R { s: -b.s, x: a.x, y: a.y, w: a.w, xyw: (a.xyw + -b.xyw) }
  //  }
  //}
  impl Sub<Magnitude, f64> for () {
    type R = Magnitude;
    fn sub(a: Magnitude, b: f64) -> Self::R {
      Self::R { s: (a.s + -b), xyw: a.xyw }
    }
  }
  //impl Sub<Magnitude, Univec> for () {
  //  type R = <ERROR>;
  //  fn sub(a: Magnitude, b: Univec) -> Self::R {
  //    Self::R { s: a.s, x: -b.x, y: -b.y, w: -b.w, xyw: a.xyw }
  //  }
  //}
  //impl Sub<Magnitude, Bivec> for () {
  //  type R = <ERROR>;
  //  fn sub(a: Magnitude, b: Bivec) -> Self::R {
  //    Self::R { s: a.s, xy: -b.xy, xw: -b.xw, yw: -b.yw, xyw: a.xyw }
  //  }
  //}
  impl Sub<Magnitude, Trivec> for () {
    type R = Magnitude;
    fn sub(a: Magnitude, b: Trivec) -> Self::R {
      Self::R { s: a.s, xyw: (a.xyw + -b.xyw) }
    }
  }
  //impl Sub<Magnitude, Evenvec> for () {
  //  type R = <ERROR>;
  //  fn sub(a: Magnitude, b: Evenvec) -> Self::R {
  //    Self::R { s: (a.s + -b.s), xy: -b.xy, xw: -b.xw, yw: -b.yw, xyw: a.xyw }
  //  }
  //}
  //impl Sub<Magnitude, Oddvec> for () {
  //  type R = <ERROR>;
  //  fn sub(a: Magnitude, b: Oddvec) -> Self::R {
  //    Self::R { s: a.s, x: -b.x, y: -b.y, w: -b.w, xyw: (a.xyw + -b.xyw) }
  //  }
  //}
  impl Sub<Magnitude, Magnitude> for () {
    type R = Magnitude;
    fn sub(a: Magnitude, b: Magnitude) -> Self::R {
      Self::R { s: (a.s + -b.s), xyw: (a.xyw + -b.xyw) }
    }
  }
  // --- Reverse ---
  //impl Reverse<f64> for () {
  //  type R = f64;
  //  fn reverse(a: f64) -> Self::R {
  //    a
  //  }
  //}
  impl Reverse<Univec> for () {
    type R = Univec;
    fn reverse(a: Univec) -> Self::R {
      Self::R { x: a.x, y: a.y, w: a.w }
    }
  }
  impl Reverse<Bivec> for () {
    type R = Bivec;
    fn reverse(a: Bivec) -> Self::R {
      Self::R { xy: -a.xy, xw: -a.xw, yw: -a.yw }
    }
  }
  impl Reverse<Trivec> for () {
    type R = Trivec;
    fn reverse(a: Trivec) -> Self::R {
      Self::R { xyw: -a.xyw }
    }
  }
  impl Reverse<Evenvec> for () {
    type R = Evenvec;
    fn reverse(a: Evenvec) -> Self::R {
      Self::R { s: a.s, xy: -a.xy, xw: -a.xw, yw: -a.yw }
    }
  }
  impl Reverse<Oddvec> for () {
    type R = Oddvec;
    fn reverse(a: Oddvec) -> Self::R {
      Self::R { x: a.x, y: a.y, w: a.w, xyw: -a.xyw }
    }
  }
  impl Reverse<Magnitude> for () {
    type R = Magnitude;
    fn reverse(a: Magnitude) -> Self::R {
      Self::R { s: a.s, xyw: -a.xyw }
    }
  }
  // --- RightComplement ---
  //impl RightComplement<f64> for () {
  //  type R = Trivec;
  //  fn right_complement(a: f64) -> Self::R {
  //    Self::R { xyw: a }
  //  }
  //}
  impl RightComplement<Univec> for () {
    type R = Bivec;
    fn right_complement(a: Univec) -> Self::R {
      Self::R { xy: a.w, xw: -a.y, yw: a.x }
    }
  }
  impl RightComplement<Bivec> for () {
    type R = Univec;
    fn right_complement(a: Bivec) -> Self::R {
      Self::R { x: a.yw, y: -a.xw, w: a.xy }
    }
  }
  impl RightComplement<Trivec> for () {
    type R = f64;
    fn right_complement(a: Trivec) -> Self::R {
      a.xyw
    }
  }
  impl RightComplement<Evenvec> for () {
    type R = Oddvec;
    fn right_complement(a: Evenvec) -> Self::R {
      Self::R { x: a.yw, y: -a.xw, w: a.xy, xyw: a.s }
    }
  }
  impl RightComplement<Oddvec> for () {
    type R = Evenvec;
    fn right_complement(a: Oddvec) -> Self::R {
      Self::R { s: a.xyw, xy: a.w, xw: -a.y, yw: a.x }
    }
  }
  impl RightComplement<Magnitude> for () {
    type R = Magnitude;
    fn right_complement(a: Magnitude) -> Self::R {
      Self::R { s: a.xyw, xyw: a.s }
    }
  }
  // --- LeftComplement ---
  //impl LeftComplement<f64> for () {
  //  type R = Trivec;
  //  fn left_complement(a: f64) -> Self::R {
  //    Self::R { xyw: a }
  //  }
  //}
  impl LeftComplement<Univec> for () {
    type R = Bivec;
    fn left_complement(a: Univec) -> Self::R {
      Self::R { xy: a.w, xw: -a.y, yw: a.x }
    }
  }
  impl LeftComplement<Bivec> for () {
    type R = Univec;
    fn left_complement(a: Bivec) -> Self::R {
      Self::R { x: a.yw, y: -a.xw, w: a.xy }
    }
  }
  impl LeftComplement<Trivec> for () {
    type R = f64;
    fn left_complement(a: Trivec) -> Self::R {
      a.xyw
    }
  }
  impl LeftComplement<Evenvec> for () {
    type R = Oddvec;
    fn left_complement(a: Evenvec) -> Self::R {
      Self::R { x: a.yw, y: -a.xw, w: a.xy, xyw: a.s }
    }
  }
  impl LeftComplement<Oddvec> for () {
    type R = Evenvec;
    fn left_complement(a: Oddvec) -> Self::R {
      Self::R { s: a.xyw, xy: a.w, xw: -a.y, yw: a.x }
    }
  }
  impl LeftComplement<Magnitude> for () {
    type R = Magnitude;
    fn left_complement(a: Magnitude) -> Self::R {
      Self::R { s: a.xyw, xyw: a.s }
    }
  }
  // --- Wedge ---
  //impl Wedge<f64, f64> for () {
  //  type R = f64;
  //  fn wedge(a: f64, b: f64) -> Self::R {
  //    (a * b)
  //  }
  //}
  impl Wedge<f64, Univec> for () {
    type R = Univec;
    fn wedge(a: f64, b: Univec) -> Self::R {
      Self::R { x: (a * b.x), y: (a * b.y), w: (a * b.w) }
    }
  }
  impl Wedge<f64, Bivec> for () {
    type R = Bivec;
    fn wedge(a: f64, b: Bivec) -> Self::R {
      Self::R { xy: (a * b.xy), xw: (a * b.xw), yw: (a * b.yw) }
    }
  }
  impl Wedge<f64, Trivec> for () {
    type R = Trivec;
    fn wedge(a: f64, b: Trivec) -> Self::R {
      Self::R { xyw: (a * b.xyw) }
    }
  }
  impl Wedge<f64, Evenvec> for () {
    type R = Evenvec;
    fn wedge(a: f64, b: Evenvec) -> Self::R {
      Self::R { s: (a * b.s), xy: (a * b.xy), xw: (a * b.xw), yw: (a * b.yw) }
    }
  }
  impl Wedge<f64, Oddvec> for () {
    type R = Oddvec;
    fn wedge(a: f64, b: Oddvec) -> Self::R {
      Self::R { x: (a * b.x), y: (a * b.y), w: (a * b.w), xyw: (a * b.xyw) }
    }
  }
  impl Wedge<f64, Magnitude> for () {
    type R = Magnitude;
    fn wedge(a: f64, b: Magnitude) -> Self::R {
      Self::R { s: (a * b.s), xyw: (a * b.xyw) }
    }
  }
  impl Wedge<Univec, f64> for () {
    type R = Univec;
    fn wedge(a: Univec, b: f64) -> Self::R {
      Self::R { x: (a.x * b), y: (a.y * b), w: (a.w * b) }
    }
  }
  impl Wedge<Univec, Univec> for () {
    type R = Bivec;
    fn wedge(a: Univec, b: Univec) -> Self::R {
      Self::R { xy: ((a.x * b.y) + -(a.y * b.x)), xw: ((a.x * b.w) + -(a.w * b.x)), yw: ((a.y * b.w) + -(a.w * b.y)) }
    }
  }
  impl Wedge<Univec, Bivec> for () {
    type R = Trivec;
    fn wedge(a: Univec, b: Bivec) -> Self::R {
      Self::R { xyw: (((a.x * b.yw) + -(a.y * b.xw)) + (a.w * b.xy)) }
    }
  }
  impl Wedge<Univec, Evenvec> for () {
    type R = Oddvec;
    fn wedge(a: Univec, b: Evenvec) -> Self::R {
      Self::R { x: (a.x * b.s), y: (a.y * b.s), w: (a.w * b.s), xyw: (((a.x * b.yw) + -(a.y * b.xw)) + (a.w * b.xy)) }
    }
  }
  impl Wedge<Univec, Oddvec> for () {
    type R = Bivec;
    fn wedge(a: Univec, b: Oddvec) -> Self::R {
      Self::R { xy: ((a.x * b.y) + -(a.y * b.x)), xw: ((a.x * b.w) + -(a.w * b.x)), yw: ((a.y * b.w) + -(a.w * b.y)) }
    }
  }
  impl Wedge<Univec, Magnitude> for () {
    type R = Univec;
    fn wedge(a: Univec, b: Magnitude) -> Self::R {
      Self::R { x: (a.x * b.s), y: (a.y * b.s), w: (a.w * b.s) }
    }
  }
  impl Wedge<Bivec, f64> for () {
    type R = Bivec;
    fn wedge(a: Bivec, b: f64) -> Self::R {
      Self::R { xy: (a.xy * b), xw: (a.xw * b), yw: (a.yw * b) }
    }
  }
  impl Wedge<Bivec, Univec> for () {
    type R = Trivec;
    fn wedge(a: Bivec, b: Univec) -> Self::R {
      Self::R { xyw: (((a.xy * b.w) + -(a.xw * b.y)) + (a.yw * b.x)) }
    }
  }
  impl Wedge<Bivec, Evenvec> for () {
    type R = Bivec;
    fn wedge(a: Bivec, b: Evenvec) -> Self::R {
      Self::R { xy: (a.xy * b.s), xw: (a.xw * b.s), yw: (a.yw * b.s) }
    }
  }
  impl Wedge<Bivec, Oddvec> for () {
    type R = Trivec;
    fn wedge(a: Bivec, b: Oddvec) -> Self::R {
      Self::R { xyw: (((a.xy * b.w) + -(a.xw * b.y)) + (a.yw * b.x)) }
    }
  }
  impl Wedge<Bivec, Magnitude> for () {
    type R = Bivec;
    fn wedge(a: Bivec, b: Magnitude) -> Self::R {
      Self::R { xy: (a.xy * b.s), xw: (a.xw * b.s), yw: (a.yw * b.s) }
    }
  }
  impl Wedge<Trivec, f64> for () {
    type R = Trivec;
    fn wedge(a: Trivec, b: f64) -> Self::R {
      Self::R { xyw: (a.xyw * b) }
    }
  }
  impl Wedge<Trivec, Evenvec> for () {
    type R = Trivec;
    fn wedge(a: Trivec, b: Evenvec) -> Self::R {
      Self::R { xyw: (a.xyw * b.s) }
    }
  }
  impl Wedge<Trivec, Magnitude> for () {
    type R = Trivec;
    fn wedge(a: Trivec, b: Magnitude) -> Self::R {
      Self::R { xyw: (a.xyw * b.s) }
    }
  }
  impl Wedge<Evenvec, f64> for () {
    type R = Evenvec;
    fn wedge(a: Evenvec, b: f64) -> Self::R {
      Self::R { s: (a.s * b), xy: (a.xy * b), xw: (a.xw * b), yw: (a.yw * b) }
    }
  }
  impl Wedge<Evenvec, Univec> for () {
    type R = Oddvec;
    fn wedge(a: Evenvec, b: Univec) -> Self::R {
      Self::R { x: (a.s * b.x), y: (a.s * b.y), w: (a.s * b.w), xyw: (((a.xy * b.w) + -(a.xw * b.y)) + (a.yw * b.x)) }
    }
  }
  impl Wedge<Evenvec, Bivec> for () {
    type R = Bivec;
    fn wedge(a: Evenvec, b: Bivec) -> Self::R {
      Self::R { xy: (a.s * b.xy), xw: (a.s * b.xw), yw: (a.s * b.yw) }
    }
  }
  impl Wedge<Evenvec, Trivec> for () {
    type R = Trivec;
    fn wedge(a: Evenvec, b: Trivec) -> Self::R {
      Self::R { xyw: (a.s * b.xyw) }
    }
  }
  impl Wedge<Evenvec, Evenvec> for () {
    type R = Evenvec;
    fn wedge(a: Evenvec, b: Evenvec) -> Self::R {
      Self::R { s: (a.s * b.s), xy: ((a.s * b.xy) + (a.xy * b.s)), xw: ((a.s * b.xw) + (a.xw * b.s)), yw: ((a.s * b.yw) + (a.yw * b.s)) }
    }
  }
  impl Wedge<Evenvec, Oddvec> for () {
    type R = Oddvec;
    fn wedge(a: Evenvec, b: Oddvec) -> Self::R {
      Self::R { x: (a.s * b.x), y: (a.s * b.y), w: (a.s * b.w), xyw: ((((a.s * b.xyw) + (a.xy * b.w)) + -(a.xw * b.y)) + (a.yw * b.x)) }
    }
  }
  //impl Wedge<Evenvec, Magnitude> for () {
  //  type R = <ERROR>;
  //  fn wedge(a: Evenvec, b: Magnitude) -> Self::R {
  //    Self::R { s: (a.s * b.s), xy: (a.xy * b.s), xw: (a.xw * b.s), yw: (a.yw * b.s), xyw: (a.s * b.xyw) }
  //  }
  //}
  impl Wedge<Oddvec, f64> for () {
    type R = Oddvec;
    fn wedge(a: Oddvec, b: f64) -> Self::R {
      Self::R { x: (a.x * b), y: (a.y * b), w: (a.w * b), xyw: (a.xyw * b) }
    }
  }
  impl Wedge<Oddvec, Univec> for () {
    type R = Bivec;
    fn wedge(a: Oddvec, b: Univec) -> Self::R {
      Self::R { xy: ((a.x * b.y) + -(a.y * b.x)), xw: ((a.x * b.w) + -(a.w * b.x)), yw: ((a.y * b.w) + -(a.w * b.y)) }
    }
  }
  impl Wedge<Oddvec, Bivec> for () {
    type R = Trivec;
    fn wedge(a: Oddvec, b: Bivec) -> Self::R {
      Self::R { xyw: (((a.x * b.yw) + -(a.y * b.xw)) + (a.w * b.xy)) }
    }
  }
  impl Wedge<Oddvec, Evenvec> for () {
    type R = Oddvec;
    fn wedge(a: Oddvec, b: Evenvec) -> Self::R {
      Self::R { x: (a.x * b.s), y: (a.y * b.s), w: (a.w * b.s), xyw: ((((a.x * b.yw) + -(a.y * b.xw)) + (a.w * b.xy)) + (a.xyw * b.s)) }
    }
  }
  impl Wedge<Oddvec, Oddvec> for () {
    type R = Bivec;
    fn wedge(a: Oddvec, b: Oddvec) -> Self::R {
      Self::R { xy: ((a.x * b.y) + -(a.y * b.x)), xw: ((a.x * b.w) + -(a.w * b.x)), yw: ((a.y * b.w) + -(a.w * b.y)) }
    }
  }
  impl Wedge<Oddvec, Magnitude> for () {
    type R = Oddvec;
    fn wedge(a: Oddvec, b: Magnitude) -> Self::R {
      Self::R { x: (a.x * b.s), y: (a.y * b.s), w: (a.w * b.s), xyw: (a.xyw * b.s) }
    }
  }
  impl Wedge<Magnitude, f64> for () {
    type R = Magnitude;
    fn wedge(a: Magnitude, b: f64) -> Self::R {
      Self::R { s: (a.s * b), xyw: (a.xyw * b) }
    }
  }
  impl Wedge<Magnitude, Univec> for () {
    type R = Univec;
    fn wedge(a: Magnitude, b: Univec) -> Self::R {
      Self::R { x: (a.s * b.x), y: (a.s * b.y), w: (a.s * b.w) }
    }
  }
  impl Wedge<Magnitude, Bivec> for () {
    type R = Bivec;
    fn wedge(a: Magnitude, b: Bivec) -> Self::R {
      Self::R { xy: (a.s * b.xy), xw: (a.s * b.xw), yw: (a.s * b.yw) }
    }
  }
  impl Wedge<Magnitude, Trivec> for () {
    type R = Trivec;
    fn wedge(a: Magnitude, b: Trivec) -> Self::R {
      Self::R { xyw: (a.s * b.xyw) }
    }
  }
  //impl Wedge<Magnitude, Evenvec> for () {
  //  type R = <ERROR>;
  //  fn wedge(a: Magnitude, b: Evenvec) -> Self::R {
  //    Self::R { s: (a.s * b.s), xy: (a.s * b.xy), xw: (a.s * b.xw), yw: (a.s * b.yw), xyw: (a.xyw * b.s) }
  //  }
  //}
  impl Wedge<Magnitude, Oddvec> for () {
    type R = Oddvec;
    fn wedge(a: Magnitude, b: Oddvec) -> Self::R {
      Self::R { x: (a.s * b.x), y: (a.s * b.y), w: (a.s * b.w), xyw: (a.s * b.xyw) }
    }
  }
  impl Wedge<Magnitude, Magnitude> for () {
    type R = Magnitude;
    fn wedge(a: Magnitude, b: Magnitude) -> Self::R {
      Self::R { s: (a.s * b.s), xyw: ((a.s * b.xyw) + (a.xyw * b.s)) }
    }
  }
  // --- Antiwedge ---
  impl Antiwedge<f64, Trivec> for () {
    type R = f64;
    fn antiwedge(a: f64, b: Trivec) -> Self::R {
      (a * b.xyw)
    }
  }
  impl Antiwedge<f64, Oddvec> for () {
    type R = f64;
    fn antiwedge(a: f64, b: Oddvec) -> Self::R {
      (a * b.xyw)
    }
  }
  impl Antiwedge<f64, Magnitude> for () {
    type R = f64;
    fn antiwedge(a: f64, b: Magnitude) -> Self::R {
      (a * b.xyw)
    }
  }
  impl Antiwedge<Univec, Bivec> for () {
    type R = f64;
    fn antiwedge(a: Univec, b: Bivec) -> Self::R {
      (((a.x * b.yw) + -(a.y * b.xw)) + (a.w * b.xy))
    }
  }
  impl Antiwedge<Univec, Trivec> for () {
    type R = Univec;
    fn antiwedge(a: Univec, b: Trivec) -> Self::R {
      Self::R { x: (a.x * b.xyw), y: (a.y * b.xyw), w: (a.w * b.xyw) }
    }
  }
  impl Antiwedge<Univec, Evenvec> for () {
    type R = f64;
    fn antiwedge(a: Univec, b: Evenvec) -> Self::R {
      (((a.x * b.yw) + -(a.y * b.xw)) + (a.w * b.xy))
    }
  }
  impl Antiwedge<Univec, Oddvec> for () {
    type R = Univec;
    fn antiwedge(a: Univec, b: Oddvec) -> Self::R {
      Self::R { x: (a.x * b.xyw), y: (a.y * b.xyw), w: (a.w * b.xyw) }
    }
  }
  impl Antiwedge<Univec, Magnitude> for () {
    type R = Univec;
    fn antiwedge(a: Univec, b: Magnitude) -> Self::R {
      Self::R { x: (a.x * b.xyw), y: (a.y * b.xyw), w: (a.w * b.xyw) }
    }
  }
  impl Antiwedge<Bivec, Univec> for () {
    type R = f64;
    fn antiwedge(a: Bivec, b: Univec) -> Self::R {
      (((a.xy * b.w) + -(a.xw * b.y)) + (a.yw * b.x))
    }
  }
  impl Antiwedge<Bivec, Bivec> for () {
    type R = Univec;
    fn antiwedge(a: Bivec, b: Bivec) -> Self::R {
      Self::R { x: (-(a.xy * b.xw) + (a.xw * b.xy)), y: (-(a.xy * b.yw) + (a.yw * b.xy)), w: (-(a.xw * b.yw) + (a.yw * b.xw)) }
    }
  }
  impl Antiwedge<Bivec, Trivec> for () {
    type R = Bivec;
    fn antiwedge(a: Bivec, b: Trivec) -> Self::R {
      Self::R { xy: (a.xy * b.xyw), xw: (a.xw * b.xyw), yw: (a.yw * b.xyw) }
    }
  }
  impl Antiwedge<Bivec, Evenvec> for () {
    type R = Univec;
    fn antiwedge(a: Bivec, b: Evenvec) -> Self::R {
      Self::R { x: (-(a.xy * b.xw) + (a.xw * b.xy)), y: (-(a.xy * b.yw) + (a.yw * b.xy)), w: (-(a.xw * b.yw) + (a.yw * b.xw)) }
    }
  }
  impl Antiwedge<Bivec, Oddvec> for () {
    type R = Evenvec;
    fn antiwedge(a: Bivec, b: Oddvec) -> Self::R {
      Self::R { s: (((a.xy * b.w) + -(a.xw * b.y)) + (a.yw * b.x)), xy: (a.xy * b.xyw), xw: (a.xw * b.xyw), yw: (a.yw * b.xyw) }
    }
  }
  impl Antiwedge<Bivec, Magnitude> for () {
    type R = Bivec;
    fn antiwedge(a: Bivec, b: Magnitude) -> Self::R {
      Self::R { xy: (a.xy * b.xyw), xw: (a.xw * b.xyw), yw: (a.yw * b.xyw) }
    }
  }
  impl Antiwedge<Trivec, f64> for () {
    type R = f64;
    fn antiwedge(a: Trivec, b: f64) -> Self::R {
      (a.xyw * b)
    }
  }
  impl Antiwedge<Trivec, Univec> for () {
    type R = Univec;
    fn antiwedge(a: Trivec, b: Univec) -> Self::R {
      Self::R { x: (a.xyw * b.x), y: (a.xyw * b.y), w: (a.xyw * b.w) }
    }
  }
  impl Antiwedge<Trivec, Bivec> for () {
    type R = Bivec;
    fn antiwedge(a: Trivec, b: Bivec) -> Self::R {
      Self::R { xy: (a.xyw * b.xy), xw: (a.xyw * b.xw), yw: (a.xyw * b.yw) }
    }
  }
  impl Antiwedge<Trivec, Trivec> for () {
    type R = Trivec;
    fn antiwedge(a: Trivec, b: Trivec) -> Self::R {
      Self::R { xyw: (a.xyw * b.xyw) }
    }
  }
  impl Antiwedge<Trivec, Evenvec> for () {
    type R = Evenvec;
    fn antiwedge(a: Trivec, b: Evenvec) -> Self::R {
      Self::R { s: (a.xyw * b.s), xy: (a.xyw * b.xy), xw: (a.xyw * b.xw), yw: (a.xyw * b.yw) }
    }
  }
  impl Antiwedge<Trivec, Oddvec> for () {
    type R = Oddvec;
    fn antiwedge(a: Trivec, b: Oddvec) -> Self::R {
      Self::R { x: (a.xyw * b.x), y: (a.xyw * b.y), w: (a.xyw * b.w), xyw: (a.xyw * b.xyw) }
    }
  }
  impl Antiwedge<Trivec, Magnitude> for () {
    type R = Magnitude;
    fn antiwedge(a: Trivec, b: Magnitude) -> Self::R {
      Self::R { s: (a.xyw * b.s), xyw: (a.xyw * b.xyw) }
    }
  }
  impl Antiwedge<Evenvec, Univec> for () {
    type R = f64;
    fn antiwedge(a: Evenvec, b: Univec) -> Self::R {
      (((a.xy * b.w) + -(a.xw * b.y)) + (a.yw * b.x))
    }
  }
  impl Antiwedge<Evenvec, Bivec> for () {
    type R = Univec;
    fn antiwedge(a: Evenvec, b: Bivec) -> Self::R {
      Self::R { x: (-(a.xy * b.xw) + (a.xw * b.xy)), y: (-(a.xy * b.yw) + (a.yw * b.xy)), w: (-(a.xw * b.yw) + (a.yw * b.xw)) }
    }
  }
  impl Antiwedge<Evenvec, Trivec> for () {
    type R = Evenvec;
    fn antiwedge(a: Evenvec, b: Trivec) -> Self::R {
      Self::R { s: (a.s * b.xyw), xy: (a.xy * b.xyw), xw: (a.xw * b.xyw), yw: (a.yw * b.xyw) }
    }
  }
  impl Antiwedge<Evenvec, Evenvec> for () {
    type R = Univec;
    fn antiwedge(a: Evenvec, b: Evenvec) -> Self::R {
      Self::R { x: (-(a.xy * b.xw) + (a.xw * b.xy)), y: (-(a.xy * b.yw) + (a.yw * b.xy)), w: (-(a.xw * b.yw) + (a.yw * b.xw)) }
    }
  }
  impl Antiwedge<Evenvec, Oddvec> for () {
    type R = Evenvec;
    fn antiwedge(a: Evenvec, b: Oddvec) -> Self::R {
      Self::R { s: ((((a.s * b.xyw) + (a.xy * b.w)) + -(a.xw * b.y)) + (a.yw * b.x)), xy: (a.xy * b.xyw), xw: (a.xw * b.xyw), yw: (a.yw * b.xyw) }
    }
  }
  impl Antiwedge<Evenvec, Magnitude> for () {
    type R = Evenvec;
    fn antiwedge(a: Evenvec, b: Magnitude) -> Self::R {
      Self::R { s: (a.s * b.xyw), xy: (a.xy * b.xyw), xw: (a.xw * b.xyw), yw: (a.yw * b.xyw) }
    }
  }
  impl Antiwedge<Oddvec, f64> for () {
    type R = f64;
    fn antiwedge(a: Oddvec, b: f64) -> Self::R {
      (a.xyw * b)
    }
  }
  impl Antiwedge<Oddvec, Univec> for () {
    type R = Univec;
    fn antiwedge(a: Oddvec, b: Univec) -> Self::R {
      Self::R { x: (a.xyw * b.x), y: (a.xyw * b.y), w: (a.xyw * b.w) }
    }
  }
  impl Antiwedge<Oddvec, Bivec> for () {
    type R = Evenvec;
    fn antiwedge(a: Oddvec, b: Bivec) -> Self::R {
      Self::R { s: (((a.x * b.yw) + -(a.y * b.xw)) + (a.w * b.xy)), xy: (a.xyw * b.xy), xw: (a.xyw * b.xw), yw: (a.xyw * b.yw) }
    }
  }
  impl Antiwedge<Oddvec, Trivec> for () {
    type R = Oddvec;
    fn antiwedge(a: Oddvec, b: Trivec) -> Self::R {
      Self::R { x: (a.x * b.xyw), y: (a.y * b.xyw), w: (a.w * b.xyw), xyw: (a.xyw * b.xyw) }
    }
  }
  impl Antiwedge<Oddvec, Evenvec> for () {
    type R = Evenvec;
    fn antiwedge(a: Oddvec, b: Evenvec) -> Self::R {
      Self::R { s: ((((a.x * b.yw) + -(a.y * b.xw)) + (a.w * b.xy)) + (a.xyw * b.s)), xy: (a.xyw * b.xy), xw: (a.xyw * b.xw), yw: (a.xyw * b.yw) }
    }
  }
  impl Antiwedge<Oddvec, Oddvec> for () {
    type R = Oddvec;
    fn antiwedge(a: Oddvec, b: Oddvec) -> Self::R {
      Self::R { x: ((a.x * b.xyw) + (a.xyw * b.x)), y: ((a.y * b.xyw) + (a.xyw * b.y)), w: ((a.w * b.xyw) + (a.xyw * b.w)), xyw: (a.xyw * b.xyw) }
    }
  }
  //impl Antiwedge<Oddvec, Magnitude> for () {
  //  type R = <ERROR>;
  //  fn antiwedge(a: Oddvec, b: Magnitude) -> Self::R {
  //    Self::R { s: (a.xyw * b.s), x: (a.x * b.xyw), y: (a.y * b.xyw), w: (a.w * b.xyw), xyw: (a.xyw * b.xyw) }
  //  }
  //}
  impl Antiwedge<Magnitude, f64> for () {
    type R = f64;
    fn antiwedge(a: Magnitude, b: f64) -> Self::R {
      (a.xyw * b)
    }
  }
  impl Antiwedge<Magnitude, Univec> for () {
    type R = Univec;
    fn antiwedge(a: Magnitude, b: Univec) -> Self::R {
      Self::R { x: (a.xyw * b.x), y: (a.xyw * b.y), w: (a.xyw * b.w) }
    }
  }
  impl Antiwedge<Magnitude, Bivec> for () {
    type R = Bivec;
    fn antiwedge(a: Magnitude, b: Bivec) -> Self::R {
      Self::R { xy: (a.xyw * b.xy), xw: (a.xyw * b.xw), yw: (a.xyw * b.yw) }
    }
  }
  impl Antiwedge<Magnitude, Trivec> for () {
    type R = Magnitude;
    fn antiwedge(a: Magnitude, b: Trivec) -> Self::R {
      Self::R { s: (a.s * b.xyw), xyw: (a.xyw * b.xyw) }
    }
  }
  impl Antiwedge<Magnitude, Evenvec> for () {
    type R = Evenvec;
    fn antiwedge(a: Magnitude, b: Evenvec) -> Self::R {
      Self::R { s: (a.xyw * b.s), xy: (a.xyw * b.xy), xw: (a.xyw * b.xw), yw: (a.xyw * b.yw) }
    }
  }
  //impl Antiwedge<Magnitude, Oddvec> for () {
  //  type R = <ERROR>;
  //  fn antiwedge(a: Magnitude, b: Oddvec) -> Self::R {
  //    Self::R { s: (a.s * b.xyw), x: (a.xyw * b.x), y: (a.xyw * b.y), w: (a.xyw * b.w), xyw: (a.xyw * b.xyw) }
  //  }
  //}
  impl Antiwedge<Magnitude, Magnitude> for () {
    type R = Magnitude;
    fn antiwedge(a: Magnitude, b: Magnitude) -> Self::R {
      Self::R { s: ((a.s * b.xyw) + (a.xyw * b.s)), xyw: (a.xyw * b.xyw) }
    }
  }
  // --- Bulk ---
  //impl Bulk<f64> for () {
  //  type R = f64;
  //  fn bulk(a: f64) -> Self::R {
  //    a
  //  }
  //}
  impl Bulk<Univec> for () {
    type R = Univec;
    fn bulk(a: Univec) -> Self::R {
      Self::R { x: a.x, y: a.y, w: 0.0 }
    }
  }
  impl Bulk<Bivec> for () {
    type R = Bivec;
    fn bulk(a: Bivec) -> Self::R {
      Self::R { xy: a.xy, xw: 0.0, yw: 0.0 }
    }
  }
  impl Bulk<Evenvec> for () {
    type R = Evenvec;
    fn bulk(a: Evenvec) -> Self::R {
      Self::R { s: a.s, xy: a.xy, xw: 0.0, yw: 0.0 }
    }
  }
  impl Bulk<Oddvec> for () {
    type R = Univec;
    fn bulk(a: Oddvec) -> Self::R {
      Self::R { x: a.x, y: a.y, w: 0.0 }
    }
  }
  impl Bulk<Magnitude> for () {
    type R = f64;
    fn bulk(a: Magnitude) -> Self::R {
      a.s
    }
  }
  // --- Weight ---
  impl Weight<Univec> for () {
    type R = Univec;
    fn weight(a: Univec) -> Self::R {
      Self::R { w: a.w, x: 0.0, y: 0.0 }
    }
  }
  impl Weight<Bivec> for () {
    type R = Bivec;
    fn weight(a: Bivec) -> Self::R {
      Self::R { xw: a.xw, yw: a.yw, xy: 0.0 }
    }
  }
  impl Weight<Trivec> for () {
    type R = Trivec;
    fn weight(a: Trivec) -> Self::R {
      Self::R { xyw: a.xyw }
    }
  }
  impl Weight<Evenvec> for () {
    type R = Bivec;
    fn weight(a: Evenvec) -> Self::R {
      Self::R { xw: a.xw, yw: a.yw, xy: 0.0 }
    }
  }
  impl Weight<Oddvec> for () {
    type R = Oddvec;
    fn weight(a: Oddvec) -> Self::R {
      Self::R { w: a.w, xyw: a.xyw, x: 0.0, y: 0.0 }
    }
  }
  impl Weight<Magnitude> for () {
    type R = Trivec;
    fn weight(a: Magnitude) -> Self::R {
      Self::R { xyw: a.xyw }
    }
  }
  // --- BulkDual ---
  //impl BulkDual<f64> for () {
  //  type R = Trivec;
  //  fn bulk_dual(a: f64) -> Self::R {
  //    Self::R { xyw: a }
  //  }
  //}
  impl BulkDual<Univec> for () {
    type R = Bivec;
    fn bulk_dual(a: Univec) -> Self::R {
      Self::R { xw: -a.y, yw: a.x, xy: 0.0 }
    }
  }
  impl BulkDual<Bivec> for () {
    type R = Univec;
    fn bulk_dual(a: Bivec) -> Self::R {
      Self::R { w: a.xy, x: 0.0, y: 0.0 }
    }
  }
  impl BulkDual<Evenvec> for () {
    type R = Oddvec;
    fn bulk_dual(a: Evenvec) -> Self::R {
      Self::R { w: a.xy, xyw: a.s, x: 0.0, y: 0.0 }
    }
  }
  impl BulkDual<Oddvec> for () {
    type R = Bivec;
    fn bulk_dual(a: Oddvec) -> Self::R {
      Self::R { xw: -a.y, yw: a.x, xy: 0.0 }
    }
  }
  impl BulkDual<Magnitude> for () {
    type R = Trivec;
    fn bulk_dual(a: Magnitude) -> Self::R {
      Self::R { xyw: a.s }
    }
  }
  // --- WeightDual ---
  impl WeightDual<Univec> for () {
    type R = Bivec;
    fn weight_dual(a: Univec) -> Self::R {
      Self::R { xy: a.w, xw: 0.0, yw: 0.0 }
    }
  }
  impl WeightDual<Bivec> for () {
    type R = Univec;
    fn weight_dual(a: Bivec) -> Self::R {
      Self::R { x: a.yw, y: -a.xw, w: 0.0 }
    }
  }
  impl WeightDual<Trivec> for () {
    type R = f64;
    fn weight_dual(a: Trivec) -> Self::R {
      a.xyw
    }
  }
  impl WeightDual<Evenvec> for () {
    type R = Univec;
    fn weight_dual(a: Evenvec) -> Self::R {
      Self::R { x: a.yw, y: -a.xw, w: 0.0 }
    }
  }
  impl WeightDual<Oddvec> for () {
    type R = Evenvec;
    fn weight_dual(a: Oddvec) -> Self::R {
      Self::R { s: a.xyw, xy: a.w, xw: 0.0, yw: 0.0 }
    }
  }
  impl WeightDual<Magnitude> for () {
    type R = f64;
    fn weight_dual(a: Magnitude) -> Self::R {
      a.xyw
    }
  }
  // --- BulkContract ---
  //impl BulkContract<f64, f64> for () {
  //  type R = f64;
  //  fn bulk_contract(a: f64, b: f64) -> Self::R {
  //    (a * b)
  //  }
  //}
  impl BulkContract<f64, Evenvec> for () {
    type R = f64;
    fn bulk_contract(a: f64, b: Evenvec) -> Self::R {
      (a * b.s)
    }
  }
  impl BulkContract<f64, Magnitude> for () {
    type R = f64;
    fn bulk_contract(a: f64, b: Magnitude) -> Self::R {
      (a * b.s)
    }
  }
  impl BulkContract<Univec, f64> for () {
    type R = Univec;
    fn bulk_contract(a: Univec, b: f64) -> Self::R {
      Self::R { x: (a.x * b), y: (a.y * b), w: (a.w * b) }
    }
  }
  impl BulkContract<Univec, Univec> for () {
    type R = f64;
    fn bulk_contract(a: Univec, b: Univec) -> Self::R {
      ((a.x * b.x) + -(a.y * -b.y))
    }
  }
  impl BulkContract<Univec, Evenvec> for () {
    type R = Univec;
    fn bulk_contract(a: Univec, b: Evenvec) -> Self::R {
      Self::R { x: (a.x * b.s), y: (a.y * b.s), w: (a.w * b.s) }
    }
  }
  impl BulkContract<Univec, Oddvec> for () {
    type R = f64;
    fn bulk_contract(a: Univec, b: Oddvec) -> Self::R {
      ((a.x * b.x) + -(a.y * -b.y))
    }
  }
  impl BulkContract<Univec, Magnitude> for () {
    type R = Univec;
    fn bulk_contract(a: Univec, b: Magnitude) -> Self::R {
      Self::R { x: (a.x * b.s), y: (a.y * b.s), w: (a.w * b.s) }
    }
  }
  impl BulkContract<Bivec, f64> for () {
    type R = Bivec;
    fn bulk_contract(a: Bivec, b: f64) -> Self::R {
      Self::R { xy: (a.xy * b), xw: (a.xw * b), yw: (a.yw * b) }
    }
  }
  impl BulkContract<Bivec, Univec> for () {
    type R = Univec;
    fn bulk_contract(a: Bivec, b: Univec) -> Self::R {
      Self::R { x: -(a.xy * -b.y), y: -(a.xy * b.x), w: (-(a.xw * b.x) + (a.yw * -b.y)) }
    }
  }
  impl BulkContract<Bivec, Bivec> for () {
    type R = f64;
    fn bulk_contract(a: Bivec, b: Bivec) -> Self::R {
      (a.xy * b.xy)
    }
  }
  impl BulkContract<Bivec, Evenvec> for () {
    type R = Evenvec;
    fn bulk_contract(a: Bivec, b: Evenvec) -> Self::R {
      Self::R { s: (a.xy * b.xy), xy: (a.xy * b.s), xw: (a.xw * b.s), yw: (a.yw * b.s) }
    }
  }
  impl BulkContract<Bivec, Oddvec> for () {
    type R = Univec;
    fn bulk_contract(a: Bivec, b: Oddvec) -> Self::R {
      Self::R { x: -(a.xy * -b.y), y: -(a.xy * b.x), w: (-(a.xw * b.x) + (a.yw * -b.y)) }
    }
  }
  impl BulkContract<Bivec, Magnitude> for () {
    type R = Bivec;
    fn bulk_contract(a: Bivec, b: Magnitude) -> Self::R {
      Self::R { xy: (a.xy * b.s), xw: (a.xw * b.s), yw: (a.yw * b.s) }
    }
  }
  impl BulkContract<Trivec, f64> for () {
    type R = Trivec;
    fn bulk_contract(a: Trivec, b: f64) -> Self::R {
      Self::R { xyw: (a.xyw * b) }
    }
  }
  impl BulkContract<Trivec, Univec> for () {
    type R = Bivec;
    fn bulk_contract(a: Trivec, b: Univec) -> Self::R {
      Self::R { xw: (a.xyw * -b.y), yw: (a.xyw * b.x), xy: 0.0 }
    }
  }
  impl BulkContract<Trivec, Bivec> for () {
    type R = Univec;
    fn bulk_contract(a: Trivec, b: Bivec) -> Self::R {
      Self::R { w: (a.xyw * b.xy), x: 0.0, y: 0.0 }
    }
  }
  impl BulkContract<Trivec, Evenvec> for () {
    type R = Oddvec;
    fn bulk_contract(a: Trivec, b: Evenvec) -> Self::R {
      Self::R { w: (a.xyw * b.xy), xyw: (a.xyw * b.s), x: 0.0, y: 0.0 }
    }
  }
  impl BulkContract<Trivec, Oddvec> for () {
    type R = Bivec;
    fn bulk_contract(a: Trivec, b: Oddvec) -> Self::R {
      Self::R { xw: (a.xyw * -b.y), yw: (a.xyw * b.x), xy: 0.0 }
    }
  }
  impl BulkContract<Trivec, Magnitude> for () {
    type R = Trivec;
    fn bulk_contract(a: Trivec, b: Magnitude) -> Self::R {
      Self::R { xyw: (a.xyw * b.s) }
    }
  }
  impl BulkContract<Evenvec, f64> for () {
    type R = Evenvec;
    fn bulk_contract(a: Evenvec, b: f64) -> Self::R {
      Self::R { s: (a.s * b), xy: (a.xy * b), xw: (a.xw * b), yw: (a.yw * b) }
    }
  }
  impl BulkContract<Evenvec, Univec> for () {
    type R = Univec;
    fn bulk_contract(a: Evenvec, b: Univec) -> Self::R {
      Self::R { x: -(a.xy * -b.y), y: -(a.xy * b.x), w: (-(a.xw * b.x) + (a.yw * -b.y)) }
    }
  }
  impl BulkContract<Evenvec, Bivec> for () {
    type R = f64;
    fn bulk_contract(a: Evenvec, b: Bivec) -> Self::R {
      (a.xy * b.xy)
    }
  }
  impl BulkContract<Evenvec, Evenvec> for () {
    type R = Evenvec;
    fn bulk_contract(a: Evenvec, b: Evenvec) -> Self::R {
      Self::R { s: ((a.s * b.s) + (a.xy * b.xy)), xy: (a.xy * b.s), xw: (a.xw * b.s), yw: (a.yw * b.s) }
    }
  }
  impl BulkContract<Evenvec, Oddvec> for () {
    type R = Univec;
    fn bulk_contract(a: Evenvec, b: Oddvec) -> Self::R {
      Self::R { x: -(a.xy * -b.y), y: -(a.xy * b.x), w: (-(a.xw * b.x) + (a.yw * -b.y)) }
    }
  }
  impl BulkContract<Evenvec, Magnitude> for () {
    type R = Evenvec;
    fn bulk_contract(a: Evenvec, b: Magnitude) -> Self::R {
      Self::R { s: (a.s * b.s), xy: (a.xy * b.s), xw: (a.xw * b.s), yw: (a.yw * b.s) }
    }
  }
  impl BulkContract<Oddvec, f64> for () {
    type R = Oddvec;
    fn bulk_contract(a: Oddvec, b: f64) -> Self::R {
      Self::R { x: (a.x * b), y: (a.y * b), w: (a.w * b), xyw: (a.xyw * b) }
    }
  }
  impl BulkContract<Oddvec, Univec> for () {
    type R = Evenvec;
    fn bulk_contract(a: Oddvec, b: Univec) -> Self::R {
      Self::R { s: ((a.x * b.x) + -(a.y * -b.y)), xw: (a.xyw * -b.y), yw: (a.xyw * b.x), xy: 0.0 }
    }
  }
  impl BulkContract<Oddvec, Bivec> for () {
    type R = Univec;
    fn bulk_contract(a: Oddvec, b: Bivec) -> Self::R {
      Self::R { w: (a.xyw * b.xy), x: 0.0, y: 0.0 }
    }
  }
  impl BulkContract<Oddvec, Evenvec> for () {
    type R = Oddvec;
    fn bulk_contract(a: Oddvec, b: Evenvec) -> Self::R {
      Self::R { x: (a.x * b.s), y: (a.y * b.s), w: ((a.w * b.s) + (a.xyw * b.xy)), xyw: (a.xyw * b.s) }
    }
  }
  impl BulkContract<Oddvec, Oddvec> for () {
    type R = Evenvec;
    fn bulk_contract(a: Oddvec, b: Oddvec) -> Self::R {
      Self::R { s: ((a.x * b.x) + -(a.y * -b.y)), xw: (a.xyw * -b.y), yw: (a.xyw * b.x), xy: 0.0 }
    }
  }
  impl BulkContract<Oddvec, Magnitude> for () {
    type R = Oddvec;
    fn bulk_contract(a: Oddvec, b: Magnitude) -> Self::R {
      Self::R { x: (a.x * b.s), y: (a.y * b.s), w: (a.w * b.s), xyw: (a.xyw * b.s) }
    }
  }
  impl BulkContract<Magnitude, f64> for () {
    type R = Magnitude;
    fn bulk_contract(a: Magnitude, b: f64) -> Self::R {
      Self::R { s: (a.s * b), xyw: (a.xyw * b) }
    }
  }
  impl BulkContract<Magnitude, Univec> for () {
    type R = Bivec;
    fn bulk_contract(a: Magnitude, b: Univec) -> Self::R {
      Self::R { xw: (a.xyw * -b.y), yw: (a.xyw * b.x), xy: 0.0 }
    }
  }
  impl BulkContract<Magnitude, Bivec> for () {
    type R = Univec;
    fn bulk_contract(a: Magnitude, b: Bivec) -> Self::R {
      Self::R { w: (a.xyw * b.xy), x: 0.0, y: 0.0 }
    }
  }
  //impl BulkContract<Magnitude, Evenvec> for () {
  //  type R = <ERROR>;
  //  fn bulk_contract(a: Magnitude, b: Evenvec) -> Self::R {
  //    Self::R { s: (a.s * b.s), w: (a.xyw * b.xy), xyw: (a.xyw * b.s), x: 0.0, y: 0.0 }
  //  }
  //}
  impl BulkContract<Magnitude, Oddvec> for () {
    type R = Bivec;
    fn bulk_contract(a: Magnitude, b: Oddvec) -> Self::R {
      Self::R { xw: (a.xyw * -b.y), yw: (a.xyw * b.x), xy: 0.0 }
    }
  }
  impl BulkContract<Magnitude, Magnitude> for () {
    type R = Magnitude;
    fn bulk_contract(a: Magnitude, b: Magnitude) -> Self::R {
      Self::R { s: (a.s * b.s), xyw: (a.xyw * b.s) }
    }
  }
  // --- WeightContract ---
  impl WeightContract<Univec, Univec> for () {
    type R = f64;
    fn weight_contract(a: Univec, b: Univec) -> Self::R {
      (a.w * b.w)
    }
  }
  impl WeightContract<Univec, Oddvec> for () {
    type R = f64;
    fn weight_contract(a: Univec, b: Oddvec) -> Self::R {
      (a.w * b.w)
    }
  }
  impl WeightContract<Bivec, Univec> for () {
    type R = Univec;
    fn weight_contract(a: Bivec, b: Univec) -> Self::R {
      Self::R { x: (a.xw * b.w), y: (a.yw * b.w), w: 0.0 }
    }
  }
  impl WeightContract<Bivec, Bivec> for () {
    type R = f64;
    fn weight_contract(a: Bivec, b: Bivec) -> Self::R {
      (-(a.xw * -b.xw) + (a.yw * b.yw))
    }
  }
  impl WeightContract<Bivec, Evenvec> for () {
    type R = f64;
    fn weight_contract(a: Bivec, b: Evenvec) -> Self::R {
      (-(a.xw * -b.xw) + (a.yw * b.yw))
    }
  }
  impl WeightContract<Bivec, Oddvec> for () {
    type R = Univec;
    fn weight_contract(a: Bivec, b: Oddvec) -> Self::R {
      Self::R { x: (a.xw * b.w), y: (a.yw * b.w), w: 0.0 }
    }
  }
  impl WeightContract<Trivec, Univec> for () {
    type R = Bivec;
    fn weight_contract(a: Trivec, b: Univec) -> Self::R {
      Self::R { xy: (a.xyw * b.w), xw: 0.0, yw: 0.0 }
    }
  }
  impl WeightContract<Trivec, Bivec> for () {
    type R = Univec;
    fn weight_contract(a: Trivec, b: Bivec) -> Self::R {
      Self::R { x: (a.xyw * b.yw), y: (a.xyw * -b.xw), w: 0.0 }
    }
  }
  impl WeightContract<Trivec, Trivec> for () {
    type R = f64;
    fn weight_contract(a: Trivec, b: Trivec) -> Self::R {
      (a.xyw * b.xyw)
    }
  }
  impl WeightContract<Trivec, Evenvec> for () {
    type R = Univec;
    fn weight_contract(a: Trivec, b: Evenvec) -> Self::R {
      Self::R { x: (a.xyw * b.yw), y: (a.xyw * -b.xw), w: 0.0 }
    }
  }
  impl WeightContract<Trivec, Oddvec> for () {
    type R = Evenvec;
    fn weight_contract(a: Trivec, b: Oddvec) -> Self::R {
      Self::R { s: (a.xyw * b.xyw), xy: (a.xyw * b.w), xw: 0.0, yw: 0.0 }
    }
  }
  impl WeightContract<Trivec, Magnitude> for () {
    type R = f64;
    fn weight_contract(a: Trivec, b: Magnitude) -> Self::R {
      (a.xyw * b.xyw)
    }
  }
  impl WeightContract<Evenvec, Univec> for () {
    type R = Univec;
    fn weight_contract(a: Evenvec, b: Univec) -> Self::R {
      Self::R { x: (a.xw * b.w), y: (a.yw * b.w), w: 0.0 }
    }
  }
  impl WeightContract<Evenvec, Bivec> for () {
    type R = f64;
    fn weight_contract(a: Evenvec, b: Bivec) -> Self::R {
      (-(a.xw * -b.xw) + (a.yw * b.yw))
    }
  }
  impl WeightContract<Evenvec, Evenvec> for () {
    type R = f64;
    fn weight_contract(a: Evenvec, b: Evenvec) -> Self::R {
      (-(a.xw * -b.xw) + (a.yw * b.yw))
    }
  }
  impl WeightContract<Evenvec, Oddvec> for () {
    type R = Univec;
    fn weight_contract(a: Evenvec, b: Oddvec) -> Self::R {
      Self::R { x: (a.xw * b.w), y: (a.yw * b.w), w: 0.0 }
    }
  }
  impl WeightContract<Oddvec, Univec> for () {
    type R = Evenvec;
    fn weight_contract(a: Oddvec, b: Univec) -> Self::R {
      Self::R { s: (a.w * b.w), xy: (a.xyw * b.w), xw: 0.0, yw: 0.0 }
    }
  }
  impl WeightContract<Oddvec, Bivec> for () {
    type R = Univec;
    fn weight_contract(a: Oddvec, b: Bivec) -> Self::R {
      Self::R { x: (a.xyw * b.yw), y: (a.xyw * -b.xw), w: 0.0 }
    }
  }
  impl WeightContract<Oddvec, Trivec> for () {
    type R = f64;
    fn weight_contract(a: Oddvec, b: Trivec) -> Self::R {
      (a.xyw * b.xyw)
    }
  }
  impl WeightContract<Oddvec, Evenvec> for () {
    type R = Univec;
    fn weight_contract(a: Oddvec, b: Evenvec) -> Self::R {
      Self::R { x: (a.xyw * b.yw), y: (a.xyw * -b.xw), w: 0.0 }
    }
  }
  impl WeightContract<Oddvec, Oddvec> for () {
    type R = Evenvec;
    fn weight_contract(a: Oddvec, b: Oddvec) -> Self::R {
      Self::R { s: ((a.w * b.w) + (a.xyw * b.xyw)), xy: (a.xyw * b.w), xw: 0.0, yw: 0.0 }
    }
  }
  impl WeightContract<Oddvec, Magnitude> for () {
    type R = f64;
    fn weight_contract(a: Oddvec, b: Magnitude) -> Self::R {
      (a.xyw * b.xyw)
    }
  }
  impl WeightContract<Magnitude, Univec> for () {
    type R = Bivec;
    fn weight_contract(a: Magnitude, b: Univec) -> Self::R {
      Self::R { xy: (a.xyw * b.w), xw: 0.0, yw: 0.0 }
    }
  }
  impl WeightContract<Magnitude, Bivec> for () {
    type R = Univec;
    fn weight_contract(a: Magnitude, b: Bivec) -> Self::R {
      Self::R { x: (a.xyw * b.yw), y: (a.xyw * -b.xw), w: 0.0 }
    }
  }
  impl WeightContract<Magnitude, Trivec> for () {
    type R = f64;
    fn weight_contract(a: Magnitude, b: Trivec) -> Self::R {
      (a.xyw * b.xyw)
    }
  }
  impl WeightContract<Magnitude, Evenvec> for () {
    type R = Univec;
    fn weight_contract(a: Magnitude, b: Evenvec) -> Self::R {
      Self::R { x: (a.xyw * b.yw), y: (a.xyw * -b.xw), w: 0.0 }
    }
  }
  impl WeightContract<Magnitude, Oddvec> for () {
    type R = Evenvec;
    fn weight_contract(a: Magnitude, b: Oddvec) -> Self::R {
      Self::R { s: (a.xyw * b.xyw), xy: (a.xyw * b.w), xw: 0.0, yw: 0.0 }
    }
  }
  impl WeightContract<Magnitude, Magnitude> for () {
    type R = f64;
    fn weight_contract(a: Magnitude, b: Magnitude) -> Self::R {
      (a.xyw * b.xyw)
    }
  }
  // --- BulkExpand ---
  //impl BulkExpand<f64, f64> for () {
  //  type R = Trivec;
  //  fn bulk_expand(a: f64, b: f64) -> Self::R {
  //    Self::R { xyw: (a * b) }
  //  }
  //}
  impl BulkExpand<f64, Univec> for () {
    type R = Bivec;
    fn bulk_expand(a: f64, b: Univec) -> Self::R {
      Self::R { xw: (a * -b.y), yw: (a * b.x), xy: 0.0 }
    }
  }
  impl BulkExpand<f64, Bivec> for () {
    type R = Univec;
    fn bulk_expand(a: f64, b: Bivec) -> Self::R {
      Self::R { w: (a * b.xy), x: 0.0, y: 0.0 }
    }
  }
  impl BulkExpand<f64, Evenvec> for () {
    type R = Oddvec;
    fn bulk_expand(a: f64, b: Evenvec) -> Self::R {
      Self::R { w: (a * b.xy), xyw: (a * b.s), x: 0.0, y: 0.0 }
    }
  }
  impl BulkExpand<f64, Oddvec> for () {
    type R = Bivec;
    fn bulk_expand(a: f64, b: Oddvec) -> Self::R {
      Self::R { xw: (a * -b.y), yw: (a * b.x), xy: 0.0 }
    }
  }
  impl BulkExpand<f64, Magnitude> for () {
    type R = Trivec;
    fn bulk_expand(a: f64, b: Magnitude) -> Self::R {
      Self::R { xyw: (a * b.s) }
    }
  }
  impl BulkExpand<Univec, Univec> for () {
    type R = Trivec;
    fn bulk_expand(a: Univec, b: Univec) -> Self::R {
      Self::R { xyw: ((a.x * b.x) + -(a.y * -b.y)) }
    }
  }
  impl BulkExpand<Univec, Bivec> for () {
    type R = Bivec;
    fn bulk_expand(a: Univec, b: Bivec) -> Self::R {
      Self::R { xw: (a.x * b.xy), yw: (a.y * b.xy), xy: 0.0 }
    }
  }
  impl BulkExpand<Univec, Evenvec> for () {
    type R = Bivec;
    fn bulk_expand(a: Univec, b: Evenvec) -> Self::R {
      Self::R { xw: (a.x * b.xy), yw: (a.y * b.xy), xy: 0.0 }
    }
  }
  impl BulkExpand<Univec, Oddvec> for () {
    type R = Trivec;
    fn bulk_expand(a: Univec, b: Oddvec) -> Self::R {
      Self::R { xyw: ((a.x * b.x) + -(a.y * -b.y)) }
    }
  }
  impl BulkExpand<Bivec, Bivec> for () {
    type R = Trivec;
    fn bulk_expand(a: Bivec, b: Bivec) -> Self::R {
      Self::R { xyw: (a.xy * b.xy) }
    }
  }
  impl BulkExpand<Bivec, Evenvec> for () {
    type R = Trivec;
    fn bulk_expand(a: Bivec, b: Evenvec) -> Self::R {
      Self::R { xyw: (a.xy * b.xy) }
    }
  }
  impl BulkExpand<Evenvec, f64> for () {
    type R = Trivec;
    fn bulk_expand(a: Evenvec, b: f64) -> Self::R {
      Self::R { xyw: (a.s * b) }
    }
  }
  impl BulkExpand<Evenvec, Univec> for () {
    type R = Bivec;
    fn bulk_expand(a: Evenvec, b: Univec) -> Self::R {
      Self::R { xw: (a.s * -b.y), yw: (a.s * b.x), xy: 0.0 }
    }
  }
  impl BulkExpand<Evenvec, Bivec> for () {
    type R = Oddvec;
    fn bulk_expand(a: Evenvec, b: Bivec) -> Self::R {
      Self::R { w: (a.s * b.xy), xyw: (a.xy * b.xy), x: 0.0, y: 0.0 }
    }
  }
  impl BulkExpand<Evenvec, Evenvec> for () {
    type R = Oddvec;
    fn bulk_expand(a: Evenvec, b: Evenvec) -> Self::R {
      Self::R { w: (a.s * b.xy), xyw: ((a.s * b.s) + (a.xy * b.xy)), x: 0.0, y: 0.0 }
    }
  }
  impl BulkExpand<Evenvec, Oddvec> for () {
    type R = Bivec;
    fn bulk_expand(a: Evenvec, b: Oddvec) -> Self::R {
      Self::R { xw: (a.s * -b.y), yw: (a.s * b.x), xy: 0.0 }
    }
  }
  impl BulkExpand<Evenvec, Magnitude> for () {
    type R = Trivec;
    fn bulk_expand(a: Evenvec, b: Magnitude) -> Self::R {
      Self::R { xyw: (a.s * b.s) }
    }
  }
  impl BulkExpand<Oddvec, Univec> for () {
    type R = Trivec;
    fn bulk_expand(a: Oddvec, b: Univec) -> Self::R {
      Self::R { xyw: ((a.x * b.x) + -(a.y * -b.y)) }
    }
  }
  impl BulkExpand<Oddvec, Bivec> for () {
    type R = Bivec;
    fn bulk_expand(a: Oddvec, b: Bivec) -> Self::R {
      Self::R { xw: (a.x * b.xy), yw: (a.y * b.xy), xy: 0.0 }
    }
  }
  impl BulkExpand<Oddvec, Evenvec> for () {
    type R = Bivec;
    fn bulk_expand(a: Oddvec, b: Evenvec) -> Self::R {
      Self::R { xw: (a.x * b.xy), yw: (a.y * b.xy), xy: 0.0 }
    }
  }
  impl BulkExpand<Oddvec, Oddvec> for () {
    type R = Trivec;
    fn bulk_expand(a: Oddvec, b: Oddvec) -> Self::R {
      Self::R { xyw: ((a.x * b.x) + -(a.y * -b.y)) }
    }
  }
  impl BulkExpand<Magnitude, f64> for () {
    type R = Trivec;
    fn bulk_expand(a: Magnitude, b: f64) -> Self::R {
      Self::R { xyw: (a.s * b) }
    }
  }
  impl BulkExpand<Magnitude, Univec> for () {
    type R = Bivec;
    fn bulk_expand(a: Magnitude, b: Univec) -> Self::R {
      Self::R { xw: (a.s * -b.y), yw: (a.s * b.x), xy: 0.0 }
    }
  }
  impl BulkExpand<Magnitude, Bivec> for () {
    type R = Univec;
    fn bulk_expand(a: Magnitude, b: Bivec) -> Self::R {
      Self::R { w: (a.s * b.xy), x: 0.0, y: 0.0 }
    }
  }
  impl BulkExpand<Magnitude, Evenvec> for () {
    type R = Oddvec;
    fn bulk_expand(a: Magnitude, b: Evenvec) -> Self::R {
      Self::R { w: (a.s * b.xy), xyw: (a.s * b.s), x: 0.0, y: 0.0 }
    }
  }
  impl BulkExpand<Magnitude, Oddvec> for () {
    type R = Bivec;
    fn bulk_expand(a: Magnitude, b: Oddvec) -> Self::R {
      Self::R { xw: (a.s * -b.y), yw: (a.s * b.x), xy: 0.0 }
    }
  }
  impl BulkExpand<Magnitude, Magnitude> for () {
    type R = Trivec;
    fn bulk_expand(a: Magnitude, b: Magnitude) -> Self::R {
      Self::R { xyw: (a.s * b.s) }
    }
  }
  // --- WeightExpand ---
  impl WeightExpand<f64, Univec> for () {
    type R = Bivec;
    fn weight_expand(a: f64, b: Univec) -> Self::R {
      Self::R { xy: (a * b.w), xw: 0.0, yw: 0.0 }
    }
  }
  impl WeightExpand<f64, Bivec> for () {
    type R = Univec;
    fn weight_expand(a: f64, b: Bivec) -> Self::R {
      Self::R { x: (a * b.yw), y: (a * -b.xw), w: 0.0 }
    }
  }
  impl WeightExpand<f64, Trivec> for () {
    type R = f64;
    fn weight_expand(a: f64, b: Trivec) -> Self::R {
      (a * b.xyw)
    }
  }
  impl WeightExpand<f64, Evenvec> for () {
    type R = Univec;
    fn weight_expand(a: f64, b: Evenvec) -> Self::R {
      Self::R { x: (a * b.yw), y: (a * -b.xw), w: 0.0 }
    }
  }
  impl WeightExpand<f64, Oddvec> for () {
    type R = Evenvec;
    fn weight_expand(a: f64, b: Oddvec) -> Self::R {
      Self::R { s: (a * b.xyw), xy: (a * b.w), xw: 0.0, yw: 0.0 }
    }
  }
  impl WeightExpand<f64, Magnitude> for () {
    type R = f64;
    fn weight_expand(a: f64, b: Magnitude) -> Self::R {
      (a * b.xyw)
    }
  }
  impl WeightExpand<Univec, Univec> for () {
    type R = Trivec;
    fn weight_expand(a: Univec, b: Univec) -> Self::R {
      Self::R { xyw: (a.w * b.w) }
    }
  }
  impl WeightExpand<Univec, Bivec> for () {
    type R = Bivec;
    fn weight_expand(a: Univec, b: Bivec) -> Self::R {
      Self::R { xy: ((a.x * -b.xw) + -(a.y * b.yw)), xw: -(a.w * b.yw), yw: -(a.w * -b.xw) }
    }
  }
  impl WeightExpand<Univec, Trivec> for () {
    type R = Univec;
    fn weight_expand(a: Univec, b: Trivec) -> Self::R {
      Self::R { x: (a.x * b.xyw), y: (a.y * b.xyw), w: (a.w * b.xyw) }
    }
  }
  impl WeightExpand<Univec, Evenvec> for () {
    type R = Bivec;
    fn weight_expand(a: Univec, b: Evenvec) -> Self::R {
      Self::R { xy: ((a.x * -b.xw) + -(a.y * b.yw)), xw: -(a.w * b.yw), yw: -(a.w * -b.xw) }
    }
  }
  impl WeightExpand<Univec, Oddvec> for () {
    type R = Oddvec;
    fn weight_expand(a: Univec, b: Oddvec) -> Self::R {
      Self::R { x: (a.x * b.xyw), y: (a.y * b.xyw), w: (a.w * b.xyw), xyw: (a.w * b.w) }
    }
  }
  impl WeightExpand<Univec, Magnitude> for () {
    type R = Univec;
    fn weight_expand(a: Univec, b: Magnitude) -> Self::R {
      Self::R { x: (a.x * b.xyw), y: (a.y * b.xyw), w: (a.w * b.xyw) }
    }
  }
  impl WeightExpand<Bivec, Bivec> for () {
    type R = Trivec;
    fn weight_expand(a: Bivec, b: Bivec) -> Self::R {
      Self::R { xyw: (-(a.xw * -b.xw) + (a.yw * b.yw)) }
    }
  }
  impl WeightExpand<Bivec, Trivec> for () {
    type R = Bivec;
    fn weight_expand(a: Bivec, b: Trivec) -> Self::R {
      Self::R { xy: (a.xy * b.xyw), xw: (a.xw * b.xyw), yw: (a.yw * b.xyw) }
    }
  }
  impl WeightExpand<Bivec, Evenvec> for () {
    type R = Trivec;
    fn weight_expand(a: Bivec, b: Evenvec) -> Self::R {
      Self::R { xyw: (-(a.xw * -b.xw) + (a.yw * b.yw)) }
    }
  }
  impl WeightExpand<Bivec, Oddvec> for () {
    type R = Bivec;
    fn weight_expand(a: Bivec, b: Oddvec) -> Self::R {
      Self::R { xy: (a.xy * b.xyw), xw: (a.xw * b.xyw), yw: (a.yw * b.xyw) }
    }
  }
  impl WeightExpand<Bivec, Magnitude> for () {
    type R = Bivec;
    fn weight_expand(a: Bivec, b: Magnitude) -> Self::R {
      Self::R { xy: (a.xy * b.xyw), xw: (a.xw * b.xyw), yw: (a.yw * b.xyw) }
    }
  }
  impl WeightExpand<Trivec, Trivec> for () {
    type R = Trivec;
    fn weight_expand(a: Trivec, b: Trivec) -> Self::R {
      Self::R { xyw: (a.xyw * b.xyw) }
    }
  }
  impl WeightExpand<Trivec, Oddvec> for () {
    type R = Trivec;
    fn weight_expand(a: Trivec, b: Oddvec) -> Self::R {
      Self::R { xyw: (a.xyw * b.xyw) }
    }
  }
  impl WeightExpand<Trivec, Magnitude> for () {
    type R = Trivec;
    fn weight_expand(a: Trivec, b: Magnitude) -> Self::R {
      Self::R { xyw: (a.xyw * b.xyw) }
    }
  }
  impl WeightExpand<Evenvec, Univec> for () {
    type R = Bivec;
    fn weight_expand(a: Evenvec, b: Univec) -> Self::R {
      Self::R { xy: (a.s * b.w), xw: 0.0, yw: 0.0 }
    }
  }
  impl WeightExpand<Evenvec, Bivec> for () {
    type R = Oddvec;
    fn weight_expand(a: Evenvec, b: Bivec) -> Self::R {
      Self::R { x: (a.s * b.yw), y: (a.s * -b.xw), xyw: (-(a.xw * -b.xw) + (a.yw * b.yw)), w: 0.0 }
    }
  }
  impl WeightExpand<Evenvec, Trivec> for () {
    type R = Evenvec;
    fn weight_expand(a: Evenvec, b: Trivec) -> Self::R {
      Self::R { s: (a.s * b.xyw), xy: (a.xy * b.xyw), xw: (a.xw * b.xyw), yw: (a.yw * b.xyw) }
    }
  }
  impl WeightExpand<Evenvec, Evenvec> for () {
    type R = Oddvec;
    fn weight_expand(a: Evenvec, b: Evenvec) -> Self::R {
      Self::R { x: (a.s * b.yw), y: (a.s * -b.xw), xyw: (-(a.xw * -b.xw) + (a.yw * b.yw)), w: 0.0 }
    }
  }
  impl WeightExpand<Evenvec, Oddvec> for () {
    type R = Evenvec;
    fn weight_expand(a: Evenvec, b: Oddvec) -> Self::R {
      Self::R { s: (a.s * b.xyw), xy: ((a.s * b.w) + (a.xy * b.xyw)), xw: (a.xw * b.xyw), yw: (a.yw * b.xyw) }
    }
  }
  impl WeightExpand<Evenvec, Magnitude> for () {
    type R = Evenvec;
    fn weight_expand(a: Evenvec, b: Magnitude) -> Self::R {
      Self::R { s: (a.s * b.xyw), xy: (a.xy * b.xyw), xw: (a.xw * b.xyw), yw: (a.yw * b.xyw) }
    }
  }
  impl WeightExpand<Oddvec, Univec> for () {
    type R = Trivec;
    fn weight_expand(a: Oddvec, b: Univec) -> Self::R {
      Self::R { xyw: (a.w * b.w) }
    }
  }
  impl WeightExpand<Oddvec, Bivec> for () {
    type R = Bivec;
    fn weight_expand(a: Oddvec, b: Bivec) -> Self::R {
      Self::R { xy: ((a.x * -b.xw) + -(a.y * b.yw)), xw: -(a.w * b.yw), yw: -(a.w * -b.xw) }
    }
  }
  impl WeightExpand<Oddvec, Trivec> for () {
    type R = Oddvec;
    fn weight_expand(a: Oddvec, b: Trivec) -> Self::R {
      Self::R { x: (a.x * b.xyw), y: (a.y * b.xyw), w: (a.w * b.xyw), xyw: (a.xyw * b.xyw) }
    }
  }
  impl WeightExpand<Oddvec, Evenvec> for () {
    type R = Bivec;
    fn weight_expand(a: Oddvec, b: Evenvec) -> Self::R {
      Self::R { xy: ((a.x * -b.xw) + -(a.y * b.yw)), xw: -(a.w * b.yw), yw: -(a.w * -b.xw) }
    }
  }
  impl WeightExpand<Oddvec, Oddvec> for () {
    type R = Oddvec;
    fn weight_expand(a: Oddvec, b: Oddvec) -> Self::R {
      Self::R { x: (a.x * b.xyw), y: (a.y * b.xyw), w: (a.w * b.xyw), xyw: ((a.w * b.w) + (a.xyw * b.xyw)) }
    }
  }
  impl WeightExpand<Oddvec, Magnitude> for () {
    type R = Oddvec;
    fn weight_expand(a: Oddvec, b: Magnitude) -> Self::R {
      Self::R { x: (a.x * b.xyw), y: (a.y * b.xyw), w: (a.w * b.xyw), xyw: (a.xyw * b.xyw) }
    }
  }
  impl WeightExpand<Magnitude, Univec> for () {
    type R = Bivec;
    fn weight_expand(a: Magnitude, b: Univec) -> Self::R {
      Self::R { xy: (a.s * b.w), xw: 0.0, yw: 0.0 }
    }
  }
  impl WeightExpand<Magnitude, Bivec> for () {
    type R = Univec;
    fn weight_expand(a: Magnitude, b: Bivec) -> Self::R {
      Self::R { x: (a.s * b.yw), y: (a.s * -b.xw), w: 0.0 }
    }
  }
  impl WeightExpand<Magnitude, Trivec> for () {
    type R = Magnitude;
    fn weight_expand(a: Magnitude, b: Trivec) -> Self::R {
      Self::R { s: (a.s * b.xyw), xyw: (a.xyw * b.xyw) }
    }
  }
  impl WeightExpand<Magnitude, Evenvec> for () {
    type R = Univec;
    fn weight_expand(a: Magnitude, b: Evenvec) -> Self::R {
      Self::R { x: (a.s * b.yw), y: (a.s * -b.xw), w: 0.0 }
    }
  }
  //impl WeightExpand<Magnitude, Oddvec> for () {
  //  type R = <ERROR>;
  //  fn weight_expand(a: Magnitude, b: Oddvec) -> Self::R {
  //    Self::R { s: (a.s * b.xyw), xy: (a.s * b.w), xyw: (a.xyw * b.xyw), xw: 0.0, yw: 0.0 }
  //  }
  //}
  impl WeightExpand<Magnitude, Magnitude> for () {
    type R = Magnitude;
    fn weight_expand(a: Magnitude, b: Magnitude) -> Self::R {
      Self::R { s: (a.s * b.xyw), xyw: (a.xyw * b.xyw) }
    }
  }
  // --- Project ---
  impl Project<f64, Univec> for () {
    type R = f64;
    fn project(a: f64, b: Univec) -> Self::R {
      (b.w * (a * b.w))
    }
  }
  impl Project<f64, Bivec> for () {
    type R = f64;
    fn project(a: f64, b: Bivec) -> Self::R {
      (-(b.xw * (a * -b.xw)) + (b.yw * (a * b.yw)))
    }
  }
  impl Project<f64, Trivec> for () {
    type R = f64;
    fn project(a: f64, b: Trivec) -> Self::R {
      (b.xyw * (a * b.xyw))
    }
  }
  impl Project<f64, Evenvec> for () {
    type R = f64;
    fn project(a: f64, b: Evenvec) -> Self::R {
      (-(b.xw * (a * -b.xw)) + (b.yw * (a * b.yw)))
    }
  }
  impl Project<f64, Oddvec> for () {
    type R = Evenvec;
    fn project(a: f64, b: Oddvec) -> Self::R {
      Self::R { s: ((b.w * (a * b.w)) + (b.xyw * (a * b.xyw))), xy: (b.xyw * (a * b.w)), xw: 0.0, yw: 0.0 }
    }
  }
  impl Project<f64, Magnitude> for () {
    type R = f64;
    fn project(a: f64, b: Magnitude) -> Self::R {
      (b.xyw * (a * b.xyw))
    }
  }
  impl Project<Univec, Univec> for () {
    type R = Univec;
    fn project(a: Univec, b: Univec) -> Self::R {
      Self::R { x: (b.x * (a.w * b.w)), y: (b.y * (a.w * b.w)), w: (b.w * (a.w * b.w)) }
    }
  }
  impl Project<Univec, Bivec> for () {
    type R = Univec;
    fn project(a: Univec, b: Bivec) -> Self::R {
      Self::R { x: (-(b.xy * -(a.w * b.yw)) + (b.xw * ((a.x * -b.xw) + -(a.y * b.yw)))), y: (-(b.xy * -(a.w * -b.xw)) + (b.yw * ((a.x * -b.xw) + -(a.y * b.yw)))), w: (-(b.xw * -(a.w * -b.xw)) + (b.yw * -(a.w * b.yw))) }
    }
  }
  impl Project<Univec, Trivec> for () {
    type R = Univec;
    fn project(a: Univec, b: Trivec) -> Self::R {
      Self::R { x: (b.xyw * (a.x * b.xyw)), y: (b.xyw * (a.y * b.xyw)), w: (b.xyw * (a.w * b.xyw)) }
    }
  }
  impl Project<Univec, Evenvec> for () {
    type R = Univec;
    fn project(a: Univec, b: Evenvec) -> Self::R {
      Self::R { x: (-(b.xy * -(a.w * b.yw)) + (b.xw * ((a.x * -b.xw) + -(a.y * b.yw)))), y: (-(b.xy * -(a.w * -b.xw)) + (b.yw * ((a.x * -b.xw) + -(a.y * b.yw)))), w: (-(b.xw * -(a.w * -b.xw)) + (b.yw * -(a.w * b.yw))) }
    }
  }
  impl Project<Univec, Oddvec> for () {
    type R = Oddvec;
    fn project(a: Univec, b: Oddvec) -> Self::R {
      Self::R { x: ((b.x * (a.w * b.w)) + (b.xyw * (a.x * b.xyw))), y: ((b.y * (a.w * b.w)) + (b.xyw * (a.y * b.xyw))), w: ((b.w * (a.w * b.w)) + (b.xyw * (a.w * b.xyw))), xyw: (b.xyw * (a.w * b.w)) }
    }
  }
  impl Project<Univec, Magnitude> for () {
    type R = Univec;
    fn project(a: Univec, b: Magnitude) -> Self::R {
      Self::R { x: (b.xyw * (a.x * b.xyw)), y: (b.xyw * (a.y * b.xyw)), w: (b.xyw * (a.w * b.xyw)) }
    }
  }
  impl Project<Bivec, Bivec> for () {
    type R = Bivec;
    fn project(a: Bivec, b: Bivec) -> Self::R {
      Self::R { xy: (b.xy * (-(a.xw * -b.xw) + (a.yw * b.yw))), xw: (b.xw * (-(a.xw * -b.xw) + (a.yw * b.yw))), yw: (b.yw * (-(a.xw * -b.xw) + (a.yw * b.yw))) }
    }
  }
  impl Project<Bivec, Trivec> for () {
    type R = Bivec;
    fn project(a: Bivec, b: Trivec) -> Self::R {
      Self::R { xy: (b.xyw * (a.xy * b.xyw)), xw: (b.xyw * (a.xw * b.xyw)), yw: (b.xyw * (a.yw * b.xyw)) }
    }
  }
  impl Project<Bivec, Evenvec> for () {
    type R = Evenvec;
    fn project(a: Bivec, b: Evenvec) -> Self::R {
      Self::R { s: (b.s * (-(a.xw * -b.xw) + (a.yw * b.yw))), xy: (b.xy * (-(a.xw * -b.xw) + (a.yw * b.yw))), xw: (b.xw * (-(a.xw * -b.xw) + (a.yw * b.yw))), yw: (b.yw * (-(a.xw * -b.xw) + (a.yw * b.yw))) }
    }
  }
  impl Project<Bivec, Oddvec> for () {
    type R = Evenvec;
    fn project(a: Bivec, b: Oddvec) -> Self::R {
      Self::R { s: (((b.x * (a.yw * b.xyw)) + -(b.y * (a.xw * b.xyw))) + (b.w * (a.xy * b.xyw))), xy: (b.xyw * (a.xy * b.xyw)), xw: (b.xyw * (a.xw * b.xyw)), yw: (b.xyw * (a.yw * b.xyw)) }
    }
  }
  impl Project<Bivec, Magnitude> for () {
    type R = Bivec;
    fn project(a: Bivec, b: Magnitude) -> Self::R {
      Self::R { xy: (b.xyw * (a.xy * b.xyw)), xw: (b.xyw * (a.xw * b.xyw)), yw: (b.xyw * (a.yw * b.xyw)) }
    }
  }
  impl Project<Trivec, Trivec> for () {
    type R = Trivec;
    fn project(a: Trivec, b: Trivec) -> Self::R {
      Self::R { xyw: (b.xyw * (a.xyw * b.xyw)) }
    }
  }
  impl Project<Trivec, Oddvec> for () {
    type R = Oddvec;
    fn project(a: Trivec, b: Oddvec) -> Self::R {
      Self::R { x: (b.x * (a.xyw * b.xyw)), y: (b.y * (a.xyw * b.xyw)), w: (b.w * (a.xyw * b.xyw)), xyw: (b.xyw * (a.xyw * b.xyw)) }
    }
  }
  impl Project<Trivec, Magnitude> for () {
    type R = Magnitude;
    fn project(a: Trivec, b: Magnitude) -> Self::R {
      Self::R { s: (b.s * (a.xyw * b.xyw)), xyw: (b.xyw * (a.xyw * b.xyw)) }
    }
  }
  impl Project<Evenvec, Univec> for () {
    type R = f64;
    fn project(a: Evenvec, b: Univec) -> Self::R {
      (b.w * (a.s * b.w))
    }
  }
  impl Project<Evenvec, Bivec> for () {
    type R = Evenvec;
    fn project(a: Evenvec, b: Bivec) -> Self::R {
      Self::R { s: (-(b.xw * (a.s * -b.xw)) + (b.yw * (a.s * b.yw))), xy: (b.xy * (-(a.xw * -b.xw) + (a.yw * b.yw))), xw: (b.xw * (-(a.xw * -b.xw) + (a.yw * b.yw))), yw: (b.yw * (-(a.xw * -b.xw) + (a.yw * b.yw))) }
    }
  }
  impl Project<Evenvec, Trivec> for () {
    type R = Evenvec;
    fn project(a: Evenvec, b: Trivec) -> Self::R {
      Self::R { s: (b.xyw * (a.s * b.xyw)), xy: (b.xyw * (a.xy * b.xyw)), xw: (b.xyw * (a.xw * b.xyw)), yw: (b.xyw * (a.yw * b.xyw)) }
    }
  }
  impl Project<Evenvec, Evenvec> for () {
    type R = Evenvec;
    fn project(a: Evenvec, b: Evenvec) -> Self::R {
      Self::R { s: (((b.s * (-(a.xw * -b.xw) + (a.yw * b.yw))) + -(b.xw * (a.s * -b.xw))) + (b.yw * (a.s * b.yw))), xy: (b.xy * (-(a.xw * -b.xw) + (a.yw * b.yw))), xw: (b.xw * (-(a.xw * -b.xw) + (a.yw * b.yw))), yw: (b.yw * (-(a.xw * -b.xw) + (a.yw * b.yw))) }
    }
  }
  impl Project<Evenvec, Oddvec> for () {
    type R = Evenvec;
    fn project(a: Evenvec, b: Oddvec) -> Self::R {
      Self::R { s: ((((b.x * (a.yw * b.xyw)) + -(b.y * (a.xw * b.xyw))) + (b.w * ((a.s * b.w) + (a.xy * b.xyw)))) + (b.xyw * (a.s * b.xyw))), xy: (b.xyw * ((a.s * b.w) + (a.xy * b.xyw))), xw: (b.xyw * (a.xw * b.xyw)), yw: (b.xyw * (a.yw * b.xyw)) }
    }
  }
  impl Project<Evenvec, Magnitude> for () {
    type R = Evenvec;
    fn project(a: Evenvec, b: Magnitude) -> Self::R {
      Self::R { s: (b.xyw * (a.s * b.xyw)), xy: (b.xyw * (a.xy * b.xyw)), xw: (b.xyw * (a.xw * b.xyw)), yw: (b.xyw * (a.yw * b.xyw)) }
    }
  }
  impl Project<Oddvec, Univec> for () {
    type R = Univec;
    fn project(a: Oddvec, b: Univec) -> Self::R {
      Self::R { x: (b.x * (a.w * b.w)), y: (b.y * (a.w * b.w)), w: (b.w * (a.w * b.w)) }
    }
  }
  impl Project<Oddvec, Bivec> for () {
    type R = Univec;
    fn project(a: Oddvec, b: Bivec) -> Self::R {
      Self::R { x: (-(b.xy * -(a.w * b.yw)) + (b.xw * ((a.x * -b.xw) + -(a.y * b.yw)))), y: (-(b.xy * -(a.w * -b.xw)) + (b.yw * ((a.x * -b.xw) + -(a.y * b.yw)))), w: (-(b.xw * -(a.w * -b.xw)) + (b.yw * -(a.w * b.yw))) }
    }
  }
  impl Project<Oddvec, Trivec> for () {
    type R = Oddvec;
    fn project(a: Oddvec, b: Trivec) -> Self::R {
      Self::R { x: (b.xyw * (a.x * b.xyw)), y: (b.xyw * (a.y * b.xyw)), w: (b.xyw * (a.w * b.xyw)), xyw: (b.xyw * (a.xyw * b.xyw)) }
    }
  }
  impl Project<Oddvec, Evenvec> for () {
    type R = Univec;
    fn project(a: Oddvec, b: Evenvec) -> Self::R {
      Self::R { x: (-(b.xy * -(a.w * b.yw)) + (b.xw * ((a.x * -b.xw) + -(a.y * b.yw)))), y: (-(b.xy * -(a.w * -b.xw)) + (b.yw * ((a.x * -b.xw) + -(a.y * b.yw)))), w: (-(b.xw * -(a.w * -b.xw)) + (b.yw * -(a.w * b.yw))) }
    }
  }
  impl Project<Oddvec, Oddvec> for () {
    type R = Oddvec;
    fn project(a: Oddvec, b: Oddvec) -> Self::R {
      Self::R { x: ((b.x * ((a.w * b.w) + (a.xyw * b.xyw))) + (b.xyw * (a.x * b.xyw))), y: ((b.y * ((a.w * b.w) + (a.xyw * b.xyw))) + (b.xyw * (a.y * b.xyw))), w: ((b.w * ((a.w * b.w) + (a.xyw * b.xyw))) + (b.xyw * (a.w * b.xyw))), xyw: (b.xyw * ((a.w * b.w) + (a.xyw * b.xyw))) }
    }
  }
  //impl Project<Oddvec, Magnitude> for () {
  //  type R = <ERROR>;
  //  fn project(a: Oddvec, b: Magnitude) -> Self::R {
  //    Self::R { s: (b.s * (a.xyw * b.xyw)), x: (b.xyw * (a.x * b.xyw)), y: (b.xyw * (a.y * b.xyw)), w: (b.xyw * (a.w * b.xyw)), xyw: (b.xyw * (a.xyw * b.xyw)) }
  //  }
  //}
  impl Project<Magnitude, Univec> for () {
    type R = f64;
    fn project(a: Magnitude, b: Univec) -> Self::R {
      (b.w * (a.s * b.w))
    }
  }
  impl Project<Magnitude, Bivec> for () {
    type R = f64;
    fn project(a: Magnitude, b: Bivec) -> Self::R {
      (-(b.xw * (a.s * -b.xw)) + (b.yw * (a.s * b.yw)))
    }
  }
  impl Project<Magnitude, Trivec> for () {
    type R = Magnitude;
    fn project(a: Magnitude, b: Trivec) -> Self::R {
      Self::R { s: (b.xyw * (a.s * b.xyw)), xyw: (b.xyw * (a.xyw * b.xyw)) }
    }
  }
  impl Project<Magnitude, Evenvec> for () {
    type R = f64;
    fn project(a: Magnitude, b: Evenvec) -> Self::R {
      (-(b.xw * (a.s * -b.xw)) + (b.yw * (a.s * b.yw)))
    }
  }
  //impl Project<Magnitude, Oddvec> for () {
  //  type R = <ERROR>;
  //  fn project(a: Magnitude, b: Oddvec) -> Self::R {
  //    Self::R { s: ((b.w * (a.s * b.w)) + (b.xyw * (a.s * b.xyw))), x: (b.x * (a.xyw * b.xyw)), y: (b.y * (a.xyw * b.xyw)), w: (b.w * (a.xyw * b.xyw)), xy: (b.xyw * (a.s * b.w)), xyw: (b.xyw * (a.xyw * b.xyw)), xw: 0.0, yw: 0.0 }
  //  }
  //}
  impl Project<Magnitude, Magnitude> for () {
    type R = Magnitude;
    fn project(a: Magnitude, b: Magnitude) -> Self::R {
      Self::R { s: ((b.s * (a.xyw * b.xyw)) + (b.xyw * (a.s * b.xyw))), xyw: (b.xyw * (a.xyw * b.xyw)) }
    }
  }
  // --- Antiproject ---
  impl Antiproject<Univec, Univec> for () {
    type R = Univec;
    fn antiproject(a: Univec, b: Univec) -> Self::R {
      Self::R { x: (b.x * (a.w * b.w)), y: (b.y * (a.w * b.w)), w: (b.w * (a.w * b.w)) }
    }
  }
  impl Antiproject<Univec, Oddvec> for () {
    type R = Oddvec;
    fn antiproject(a: Univec, b: Oddvec) -> Self::R {
      Self::R { x: (b.x * (a.w * b.w)), y: (b.y * (a.w * b.w)), w: (b.w * (a.w * b.w)), xyw: (b.xyw * (a.w * b.w)) }
    }
  }
  impl Antiproject<Bivec, Univec> for () {
    type R = Bivec;
    fn antiproject(a: Bivec, b: Univec) -> Self::R {
      Self::R { xy: ((b.x * (a.yw * b.w)) + -(b.y * (a.xw * b.w))), xw: -(b.w * (a.xw * b.w)), yw: -(b.w * (a.yw * b.w)) }
    }
  }
  impl Antiproject<Bivec, Bivec> for () {
    type R = Bivec;
    fn antiproject(a: Bivec, b: Bivec) -> Self::R {
      Self::R { xy: (b.xy * (-(a.xw * -b.xw) + (a.yw * b.yw))), xw: (b.xw * (-(a.xw * -b.xw) + (a.yw * b.yw))), yw: (b.yw * (-(a.xw * -b.xw) + (a.yw * b.yw))) }
    }
  }
  impl Antiproject<Bivec, Evenvec> for () {
    type R = Evenvec;
    fn antiproject(a: Bivec, b: Evenvec) -> Self::R {
      Self::R { s: (b.s * (-(a.xw * -b.xw) + (a.yw * b.yw))), xy: (b.xy * (-(a.xw * -b.xw) + (a.yw * b.yw))), xw: (b.xw * (-(a.xw * -b.xw) + (a.yw * b.yw))), yw: (b.yw * (-(a.xw * -b.xw) + (a.yw * b.yw))) }
    }
  }
  impl Antiproject<Bivec, Oddvec> for () {
    type R = Bivec;
    fn antiproject(a: Bivec, b: Oddvec) -> Self::R {
      Self::R { xy: ((b.x * (a.yw * b.w)) + -(b.y * (a.xw * b.w))), xw: -(b.w * (a.xw * b.w)), yw: -(b.w * (a.yw * b.w)) }
    }
  }
  impl Antiproject<Trivec, Univec> for () {
    type R = Trivec;
    fn antiproject(a: Trivec, b: Univec) -> Self::R {
      Self::R { xyw: (b.w * (a.xyw * b.w)) }
    }
  }
  impl Antiproject<Trivec, Bivec> for () {
    type R = Trivec;
    fn antiproject(a: Trivec, b: Bivec) -> Self::R {
      Self::R { xyw: (-(b.xw * (a.xyw * -b.xw)) + (b.yw * (a.xyw * b.yw))) }
    }
  }
  impl Antiproject<Trivec, Trivec> for () {
    type R = Trivec;
    fn antiproject(a: Trivec, b: Trivec) -> Self::R {
      Self::R { xyw: (b.xyw * (a.xyw * b.xyw)) }
    }
  }
  impl Antiproject<Trivec, Evenvec> for () {
    type R = Oddvec;
    fn antiproject(a: Trivec, b: Evenvec) -> Self::R {
      Self::R { x: (b.s * (a.xyw * b.yw)), y: (b.s * (a.xyw * -b.xw)), xyw: (-(b.xw * (a.xyw * -b.xw)) + (b.yw * (a.xyw * b.yw))), w: 0.0 }
    }
  }
  impl Antiproject<Trivec, Oddvec> for () {
    type R = Oddvec;
    fn antiproject(a: Trivec, b: Oddvec) -> Self::R {
      Self::R { x: (b.x * (a.xyw * b.xyw)), y: (b.y * (a.xyw * b.xyw)), w: (b.w * (a.xyw * b.xyw)), xyw: ((b.w * (a.xyw * b.w)) + (b.xyw * (a.xyw * b.xyw))) }
    }
  }
  impl Antiproject<Trivec, Magnitude> for () {
    type R = Magnitude;
    fn antiproject(a: Trivec, b: Magnitude) -> Self::R {
      Self::R { s: (b.s * (a.xyw * b.xyw)), xyw: (b.xyw * (a.xyw * b.xyw)) }
    }
  }
  impl Antiproject<Evenvec, Univec> for () {
    type R = Bivec;
    fn antiproject(a: Evenvec, b: Univec) -> Self::R {
      Self::R { xy: ((b.x * (a.yw * b.w)) + -(b.y * (a.xw * b.w))), xw: -(b.w * (a.xw * b.w)), yw: -(b.w * (a.yw * b.w)) }
    }
  }
  impl Antiproject<Evenvec, Bivec> for () {
    type R = Bivec;
    fn antiproject(a: Evenvec, b: Bivec) -> Self::R {
      Self::R { xy: (b.xy * (-(a.xw * -b.xw) + (a.yw * b.yw))), xw: (b.xw * (-(a.xw * -b.xw) + (a.yw * b.yw))), yw: (b.yw * (-(a.xw * -b.xw) + (a.yw * b.yw))) }
    }
  }
  impl Antiproject<Evenvec, Evenvec> for () {
    type R = Evenvec;
    fn antiproject(a: Evenvec, b: Evenvec) -> Self::R {
      Self::R { s: (b.s * (-(a.xw * -b.xw) + (a.yw * b.yw))), xy: (b.xy * (-(a.xw * -b.xw) + (a.yw * b.yw))), xw: (b.xw * (-(a.xw * -b.xw) + (a.yw * b.yw))), yw: (b.yw * (-(a.xw * -b.xw) + (a.yw * b.yw))) }
    }
  }
  impl Antiproject<Evenvec, Oddvec> for () {
    type R = Bivec;
    fn antiproject(a: Evenvec, b: Oddvec) -> Self::R {
      Self::R { xy: ((b.x * (a.yw * b.w)) + -(b.y * (a.xw * b.w))), xw: -(b.w * (a.xw * b.w)), yw: -(b.w * (a.yw * b.w)) }
    }
  }
  impl Antiproject<Oddvec, Univec> for () {
    type R = Oddvec;
    fn antiproject(a: Oddvec, b: Univec) -> Self::R {
      Self::R { x: (b.x * (a.w * b.w)), y: (b.y * (a.w * b.w)), w: (b.w * (a.w * b.w)), xyw: (b.w * (a.xyw * b.w)) }
    }
  }
  impl Antiproject<Oddvec, Bivec> for () {
    type R = Trivec;
    fn antiproject(a: Oddvec, b: Bivec) -> Self::R {
      Self::R { xyw: (-(b.xw * (a.xyw * -b.xw)) + (b.yw * (a.xyw * b.yw))) }
    }
  }
  impl Antiproject<Oddvec, Trivec> for () {
    type R = Trivec;
    fn antiproject(a: Oddvec, b: Trivec) -> Self::R {
      Self::R { xyw: (b.xyw * (a.xyw * b.xyw)) }
    }
  }
  impl Antiproject<Oddvec, Evenvec> for () {
    type R = Oddvec;
    fn antiproject(a: Oddvec, b: Evenvec) -> Self::R {
      Self::R { x: (b.s * (a.xyw * b.yw)), y: (b.s * (a.xyw * -b.xw)), xyw: (-(b.xw * (a.xyw * -b.xw)) + (b.yw * (a.xyw * b.yw))), w: 0.0 }
    }
  }
  impl Antiproject<Oddvec, Oddvec> for () {
    type R = Oddvec;
    fn antiproject(a: Oddvec, b: Oddvec) -> Self::R {
      Self::R { x: (b.x * ((a.w * b.w) + (a.xyw * b.xyw))), y: (b.y * ((a.w * b.w) + (a.xyw * b.xyw))), w: (b.w * ((a.w * b.w) + (a.xyw * b.xyw))), xyw: ((b.w * (a.xyw * b.w)) + (b.xyw * ((a.w * b.w) + (a.xyw * b.xyw)))) }
    }
  }
  impl Antiproject<Oddvec, Magnitude> for () {
    type R = Magnitude;
    fn antiproject(a: Oddvec, b: Magnitude) -> Self::R {
      Self::R { s: (b.s * (a.xyw * b.xyw)), xyw: (b.xyw * (a.xyw * b.xyw)) }
    }
  }
  impl Antiproject<Magnitude, Univec> for () {
    type R = Trivec;
    fn antiproject(a: Magnitude, b: Univec) -> Self::R {
      Self::R { xyw: (b.w * (a.xyw * b.w)) }
    }
  }
  impl Antiproject<Magnitude, Bivec> for () {
    type R = Trivec;
    fn antiproject(a: Magnitude, b: Bivec) -> Self::R {
      Self::R { xyw: (-(b.xw * (a.xyw * -b.xw)) + (b.yw * (a.xyw * b.yw))) }
    }
  }
  impl Antiproject<Magnitude, Trivec> for () {
    type R = Trivec;
    fn antiproject(a: Magnitude, b: Trivec) -> Self::R {
      Self::R { xyw: (b.xyw * (a.xyw * b.xyw)) }
    }
  }
  impl Antiproject<Magnitude, Evenvec> for () {
    type R = Oddvec;
    fn antiproject(a: Magnitude, b: Evenvec) -> Self::R {
      Self::R { x: (b.s * (a.xyw * b.yw)), y: (b.s * (a.xyw * -b.xw)), xyw: (-(b.xw * (a.xyw * -b.xw)) + (b.yw * (a.xyw * b.yw))), w: 0.0 }
    }
  }
  impl Antiproject<Magnitude, Oddvec> for () {
    type R = Oddvec;
    fn antiproject(a: Magnitude, b: Oddvec) -> Self::R {
      Self::R { x: (b.x * (a.xyw * b.xyw)), y: (b.y * (a.xyw * b.xyw)), w: (b.w * (a.xyw * b.xyw)), xyw: ((b.w * (a.xyw * b.w)) + (b.xyw * (a.xyw * b.xyw))) }
    }
  }
  impl Antiproject<Magnitude, Magnitude> for () {
    type R = Magnitude;
    fn antiproject(a: Magnitude, b: Magnitude) -> Self::R {
      Self::R { s: (b.s * (a.xyw * b.xyw)), xyw: (b.xyw * (a.xyw * b.xyw)) }
    }
  }
  // --- Dot ---
  //impl Dot<f64, f64> for () {
  //  type R = f64;
  //  fn dot(a: f64, b: f64) -> Self::R {
  //    (a * b)
  //  }
  //}
  impl Dot<f64, Evenvec> for () {
    type R = f64;
    fn dot(a: f64, b: Evenvec) -> Self::R {
      (a * b.s)
    }
  }
  impl Dot<f64, Magnitude> for () {
    type R = f64;
    fn dot(a: f64, b: Magnitude) -> Self::R {
      (a * b.s)
    }
  }
  impl Dot<Univec, Univec> for () {
    type R = f64;
    fn dot(a: Univec, b: Univec) -> Self::R {
      ((a.x * b.x) + (a.y * b.y))
    }
  }
  impl Dot<Univec, Oddvec> for () {
    type R = f64;
    fn dot(a: Univec, b: Oddvec) -> Self::R {
      ((a.x * b.x) + (a.y * b.y))
    }
  }
  impl Dot<Bivec, Bivec> for () {
    type R = f64;
    fn dot(a: Bivec, b: Bivec) -> Self::R {
      (a.xy * b.xy)
    }
  }
  impl Dot<Bivec, Evenvec> for () {
    type R = f64;
    fn dot(a: Bivec, b: Evenvec) -> Self::R {
      (a.xy * b.xy)
    }
  }
  impl Dot<Evenvec, f64> for () {
    type R = f64;
    fn dot(a: Evenvec, b: f64) -> Self::R {
      (a.s * b)
    }
  }
  impl Dot<Evenvec, Bivec> for () {
    type R = f64;
    fn dot(a: Evenvec, b: Bivec) -> Self::R {
      (a.xy * b.xy)
    }
  }
  impl Dot<Evenvec, Evenvec> for () {
    type R = f64;
    fn dot(a: Evenvec, b: Evenvec) -> Self::R {
      ((a.s * b.s) + (a.xy * b.xy))
    }
  }
  impl Dot<Evenvec, Magnitude> for () {
    type R = f64;
    fn dot(a: Evenvec, b: Magnitude) -> Self::R {
      (a.s * b.s)
    }
  }
  impl Dot<Oddvec, Univec> for () {
    type R = f64;
    fn dot(a: Oddvec, b: Univec) -> Self::R {
      ((a.x * b.x) + (a.y * b.y))
    }
  }
  impl Dot<Oddvec, Oddvec> for () {
    type R = f64;
    fn dot(a: Oddvec, b: Oddvec) -> Self::R {
      ((a.x * b.x) + (a.y * b.y))
    }
  }
  impl Dot<Magnitude, f64> for () {
    type R = f64;
    fn dot(a: Magnitude, b: f64) -> Self::R {
      (a.s * b)
    }
  }
  impl Dot<Magnitude, Evenvec> for () {
    type R = f64;
    fn dot(a: Magnitude, b: Evenvec) -> Self::R {
      (a.s * b.s)
    }
  }
  impl Dot<Magnitude, Magnitude> for () {
    type R = f64;
    fn dot(a: Magnitude, b: Magnitude) -> Self::R {
      (a.s * b.s)
    }
  }
  // --- Antidot ---
  impl Antidot<Univec, Univec> for () {
    type R = Trivec;
    fn antidot(a: Univec, b: Univec) -> Self::R {
      Self::R { xyw: (a.w * b.w) }
    }
  }
  impl Antidot<Univec, Oddvec> for () {
    type R = Trivec;
    fn antidot(a: Univec, b: Oddvec) -> Self::R {
      Self::R { xyw: (a.w * b.w) }
    }
  }
  impl Antidot<Bivec, Bivec> for () {
    type R = Trivec;
    fn antidot(a: Bivec, b: Bivec) -> Self::R {
      Self::R { xyw: ((a.xw * b.xw) + (a.yw * b.yw)) }
    }
  }
  impl Antidot<Bivec, Evenvec> for () {
    type R = Trivec;
    fn antidot(a: Bivec, b: Evenvec) -> Self::R {
      Self::R { xyw: ((a.xw * b.xw) + (a.yw * b.yw)) }
    }
  }
  impl Antidot<Trivec, Trivec> for () {
    type R = Trivec;
    fn antidot(a: Trivec, b: Trivec) -> Self::R {
      Self::R { xyw: (a.xyw * b.xyw) }
    }
  }
  impl Antidot<Trivec, Oddvec> for () {
    type R = Trivec;
    fn antidot(a: Trivec, b: Oddvec) -> Self::R {
      Self::R { xyw: (a.xyw * b.xyw) }
    }
  }
  impl Antidot<Trivec, Magnitude> for () {
    type R = Trivec;
    fn antidot(a: Trivec, b: Magnitude) -> Self::R {
      Self::R { xyw: (a.xyw * b.xyw) }
    }
  }
  impl Antidot<Evenvec, Bivec> for () {
    type R = Trivec;
    fn antidot(a: Evenvec, b: Bivec) -> Self::R {
      Self::R { xyw: ((a.xw * b.xw) + (a.yw * b.yw)) }
    }
  }
  impl Antidot<Evenvec, Evenvec> for () {
    type R = Trivec;
    fn antidot(a: Evenvec, b: Evenvec) -> Self::R {
      Self::R { xyw: ((a.xw * b.xw) + (a.yw * b.yw)) }
    }
  }
  impl Antidot<Oddvec, Univec> for () {
    type R = Trivec;
    fn antidot(a: Oddvec, b: Univec) -> Self::R {
      Self::R { xyw: (a.w * b.w) }
    }
  }
  impl Antidot<Oddvec, Trivec> for () {
    type R = Trivec;
    fn antidot(a: Oddvec, b: Trivec) -> Self::R {
      Self::R { xyw: (a.xyw * b.xyw) }
    }
  }
  impl Antidot<Oddvec, Oddvec> for () {
    type R = Trivec;
    fn antidot(a: Oddvec, b: Oddvec) -> Self::R {
      Self::R { xyw: ((a.w * b.w) + (a.xyw * b.xyw)) }
    }
  }
  impl Antidot<Oddvec, Magnitude> for () {
    type R = Trivec;
    fn antidot(a: Oddvec, b: Magnitude) -> Self::R {
      Self::R { xyw: (a.xyw * b.xyw) }
    }
  }
  impl Antidot<Magnitude, Trivec> for () {
    type R = Trivec;
    fn antidot(a: Magnitude, b: Trivec) -> Self::R {
      Self::R { xyw: (a.xyw * b.xyw) }
    }
  }
  impl Antidot<Magnitude, Oddvec> for () {
    type R = Trivec;
    fn antidot(a: Magnitude, b: Oddvec) -> Self::R {
      Self::R { xyw: (a.xyw * b.xyw) }
    }
  }
  impl Antidot<Magnitude, Magnitude> for () {
    type R = Trivec;
    fn antidot(a: Magnitude, b: Magnitude) -> Self::R {
      Self::R { xyw: (a.xyw * b.xyw) }
    }
  }
  // --- Mul ---
  //impl Mul<f64, f64> for () {
  //  type R = f64;
  //  fn mul(a: f64, b: f64) -> Self::R {
  //    ((a * b) + (a * b))
  //  }
  //}
  impl Mul<f64, Univec> for () {
    type R = Univec;
    fn mul(a: f64, b: Univec) -> Self::R {
      Self::R { x: (a * b.x), y: (a * b.y), w: (a * b.w) }
    }
  }
  impl Mul<f64, Bivec> for () {
    type R = Bivec;
    fn mul(a: f64, b: Bivec) -> Self::R {
      Self::R { xy: (a * b.xy), xw: (a * b.xw), yw: (a * b.yw) }
    }
  }
  impl Mul<f64, Trivec> for () {
    type R = Trivec;
    fn mul(a: f64, b: Trivec) -> Self::R {
      Self::R { xyw: (a * b.xyw) }
    }
  }
  impl Mul<f64, Evenvec> for () {
    type R = Evenvec;
    fn mul(a: f64, b: Evenvec) -> Self::R {
      Self::R { s: ((a * b.s) + (a * b.s)), xy: (a * b.xy), xw: (a * b.xw), yw: (a * b.yw) }
    }
  }
  impl Mul<f64, Oddvec> for () {
    type R = Oddvec;
    fn mul(a: f64, b: Oddvec) -> Self::R {
      Self::R { x: (a * b.x), y: (a * b.y), w: (a * b.w), xyw: (a * b.xyw) }
    }
  }
  impl Mul<f64, Magnitude> for () {
    type R = Magnitude;
    fn mul(a: f64, b: Magnitude) -> Self::R {
      Self::R { s: ((a * b.s) + (a * b.s)), xyw: (a * b.xyw) }
    }
  }
  impl Mul<Univec, f64> for () {
    type R = Univec;
    fn mul(a: Univec, b: f64) -> Self::R {
      Self::R { x: (a.x * b), y: (a.y * b), w: (a.w * b) }
    }
  }
  impl Mul<Univec, Univec> for () {
    type R = Evenvec;
    fn mul(a: Univec, b: Univec) -> Self::R {
      Self::R { s: ((a.x * b.x) + (a.y * b.y)), xy: ((a.x * b.y) + -(a.y * b.x)), xw: ((a.x * b.w) + -(a.w * b.x)), yw: ((a.y * b.w) + -(a.w * b.y)) }
    }
  }
  impl Mul<Univec, Bivec> for () {
    type R = Trivec;
    fn mul(a: Univec, b: Bivec) -> Self::R {
      Self::R { xyw: (((a.x * b.yw) + -(a.y * b.xw)) + (a.w * b.xy)) }
    }
  }
  impl Mul<Univec, Evenvec> for () {
    type R = Oddvec;
    fn mul(a: Univec, b: Evenvec) -> Self::R {
      Self::R { x: (a.x * b.s), y: (a.y * b.s), w: (a.w * b.s), xyw: (((a.x * b.yw) + -(a.y * b.xw)) + (a.w * b.xy)) }
    }
  }
  impl Mul<Univec, Oddvec> for () {
    type R = Evenvec;
    fn mul(a: Univec, b: Oddvec) -> Self::R {
      Self::R { s: ((a.x * b.x) + (a.y * b.y)), xy: ((a.x * b.y) + -(a.y * b.x)), xw: ((a.x * b.w) + -(a.w * b.x)), yw: ((a.y * b.w) + -(a.w * b.y)) }
    }
  }
  impl Mul<Univec, Magnitude> for () {
    type R = Univec;
    fn mul(a: Univec, b: Magnitude) -> Self::R {
      Self::R { x: (a.x * b.s), y: (a.y * b.s), w: (a.w * b.s) }
    }
  }
  impl Mul<Bivec, f64> for () {
    type R = Bivec;
    fn mul(a: Bivec, b: f64) -> Self::R {
      Self::R { xy: (a.xy * b), xw: (a.xw * b), yw: (a.yw * b) }
    }
  }
  impl Mul<Bivec, Univec> for () {
    type R = Trivec;
    fn mul(a: Bivec, b: Univec) -> Self::R {
      Self::R { xyw: (((a.xy * b.w) + -(a.xw * b.y)) + (a.yw * b.x)) }
    }
  }
  impl Mul<Bivec, Bivec> for () {
    type R = f64;
    fn mul(a: Bivec, b: Bivec) -> Self::R {
      (a.xy * b.xy)
    }
  }
  impl Mul<Bivec, Evenvec> for () {
    type R = Evenvec;
    fn mul(a: Bivec, b: Evenvec) -> Self::R {
      Self::R { s: (a.xy * b.xy), xy: (a.xy * b.s), xw: (a.xw * b.s), yw: (a.yw * b.s) }
    }
  }
  impl Mul<Bivec, Oddvec> for () {
    type R = Trivec;
    fn mul(a: Bivec, b: Oddvec) -> Self::R {
      Self::R { xyw: (((a.xy * b.w) + -(a.xw * b.y)) + (a.yw * b.x)) }
    }
  }
  impl Mul<Bivec, Magnitude> for () {
    type R = Bivec;
    fn mul(a: Bivec, b: Magnitude) -> Self::R {
      Self::R { xy: (a.xy * b.s), xw: (a.xw * b.s), yw: (a.yw * b.s) }
    }
  }
  impl Mul<Trivec, f64> for () {
    type R = Trivec;
    fn mul(a: Trivec, b: f64) -> Self::R {
      Self::R { xyw: (a.xyw * b) }
    }
  }
  impl Mul<Trivec, Evenvec> for () {
    type R = Trivec;
    fn mul(a: Trivec, b: Evenvec) -> Self::R {
      Self::R { xyw: (a.xyw * b.s) }
    }
  }
  impl Mul<Trivec, Magnitude> for () {
    type R = Trivec;
    fn mul(a: Trivec, b: Magnitude) -> Self::R {
      Self::R { xyw: (a.xyw * b.s) }
    }
  }
  impl Mul<Evenvec, f64> for () {
    type R = Evenvec;
    fn mul(a: Evenvec, b: f64) -> Self::R {
      Self::R { s: ((a.s * b) + (a.s * b)), xy: (a.xy * b), xw: (a.xw * b), yw: (a.yw * b) }
    }
  }
  impl Mul<Evenvec, Univec> for () {
    type R = Oddvec;
    fn mul(a: Evenvec, b: Univec) -> Self::R {
      Self::R { x: (a.s * b.x), y: (a.s * b.y), w: (a.s * b.w), xyw: (((a.xy * b.w) + -(a.xw * b.y)) + (a.yw * b.x)) }
    }
  }
  impl Mul<Evenvec, Bivec> for () {
    type R = Evenvec;
    fn mul(a: Evenvec, b: Bivec) -> Self::R {
      Self::R { s: (a.xy * b.xy), xy: (a.s * b.xy), xw: (a.s * b.xw), yw: (a.s * b.yw) }
    }
  }
  impl Mul<Evenvec, Trivec> for () {
    type R = Trivec;
    fn mul(a: Evenvec, b: Trivec) -> Self::R {
      Self::R { xyw: (a.s * b.xyw) }
    }
  }
  impl Mul<Evenvec, Evenvec> for () {
    type R = Evenvec;
    fn mul(a: Evenvec, b: Evenvec) -> Self::R {
      Self::R { s: ((a.s * b.s) + ((a.s * b.s) + (a.xy * b.xy))), xy: ((a.s * b.xy) + (a.xy * b.s)), xw: ((a.s * b.xw) + (a.xw * b.s)), yw: ((a.s * b.yw) + (a.yw * b.s)) }
    }
  }
  impl Mul<Evenvec, Oddvec> for () {
    type R = Oddvec;
    fn mul(a: Evenvec, b: Oddvec) -> Self::R {
      Self::R { x: (a.s * b.x), y: (a.s * b.y), w: (a.s * b.w), xyw: ((((a.s * b.xyw) + (a.xy * b.w)) + -(a.xw * b.y)) + (a.yw * b.x)) }
    }
  }
  //impl Mul<Evenvec, Magnitude> for () {
  //  type R = <ERROR>;
  //  fn mul(a: Evenvec, b: Magnitude) -> Self::R {
  //    Self::R { s: ((a.s * b.s) + (a.s * b.s)), xy: (a.xy * b.s), xw: (a.xw * b.s), yw: (a.yw * b.s), xyw: (a.s * b.xyw) }
  //  }
  //}
  impl Mul<Oddvec, f64> for () {
    type R = Oddvec;
    fn mul(a: Oddvec, b: f64) -> Self::R {
      Self::R { x: (a.x * b), y: (a.y * b), w: (a.w * b), xyw: (a.xyw * b) }
    }
  }
  impl Mul<Oddvec, Univec> for () {
    type R = Evenvec;
    fn mul(a: Oddvec, b: Univec) -> Self::R {
      Self::R { s: ((a.x * b.x) + (a.y * b.y)), xy: ((a.x * b.y) + -(a.y * b.x)), xw: ((a.x * b.w) + -(a.w * b.x)), yw: ((a.y * b.w) + -(a.w * b.y)) }
    }
  }
  impl Mul<Oddvec, Bivec> for () {
    type R = Trivec;
    fn mul(a: Oddvec, b: Bivec) -> Self::R {
      Self::R { xyw: (((a.x * b.yw) + -(a.y * b.xw)) + (a.w * b.xy)) }
    }
  }
  impl Mul<Oddvec, Evenvec> for () {
    type R = Oddvec;
    fn mul(a: Oddvec, b: Evenvec) -> Self::R {
      Self::R { x: (a.x * b.s), y: (a.y * b.s), w: (a.w * b.s), xyw: ((((a.x * b.yw) + -(a.y * b.xw)) + (a.w * b.xy)) + (a.xyw * b.s)) }
    }
  }
  impl Mul<Oddvec, Oddvec> for () {
    type R = Evenvec;
    fn mul(a: Oddvec, b: Oddvec) -> Self::R {
      Self::R { s: ((a.x * b.x) + (a.y * b.y)), xy: ((a.x * b.y) + -(a.y * b.x)), xw: ((a.x * b.w) + -(a.w * b.x)), yw: ((a.y * b.w) + -(a.w * b.y)) }
    }
  }
  impl Mul<Oddvec, Magnitude> for () {
    type R = Oddvec;
    fn mul(a: Oddvec, b: Magnitude) -> Self::R {
      Self::R { x: (a.x * b.s), y: (a.y * b.s), w: (a.w * b.s), xyw: (a.xyw * b.s) }
    }
  }
  impl Mul<Magnitude, f64> for () {
    type R = Magnitude;
    fn mul(a: Magnitude, b: f64) -> Self::R {
      Self::R { s: ((a.s * b) + (a.s * b)), xyw: (a.xyw * b) }
    }
  }
  impl Mul<Magnitude, Univec> for () {
    type R = Univec;
    fn mul(a: Magnitude, b: Univec) -> Self::R {
      Self::R { x: (a.s * b.x), y: (a.s * b.y), w: (a.s * b.w) }
    }
  }
  impl Mul<Magnitude, Bivec> for () {
    type R = Bivec;
    fn mul(a: Magnitude, b: Bivec) -> Self::R {
      Self::R { xy: (a.s * b.xy), xw: (a.s * b.xw), yw: (a.s * b.yw) }
    }
  }
  impl Mul<Magnitude, Trivec> for () {
    type R = Trivec;
    fn mul(a: Magnitude, b: Trivec) -> Self::R {
      Self::R { xyw: (a.s * b.xyw) }
    }
  }
  //impl Mul<Magnitude, Evenvec> for () {
  //  type R = <ERROR>;
  //  fn mul(a: Magnitude, b: Evenvec) -> Self::R {
  //    Self::R { s: ((a.s * b.s) + (a.s * b.s)), xy: (a.s * b.xy), xw: (a.s * b.xw), yw: (a.s * b.yw), xyw: (a.xyw * b.s) }
  //  }
  //}
  impl Mul<Magnitude, Oddvec> for () {
    type R = Oddvec;
    fn mul(a: Magnitude, b: Oddvec) -> Self::R {
      Self::R { x: (a.s * b.x), y: (a.s * b.y), w: (a.s * b.w), xyw: (a.s * b.xyw) }
    }
  }
  impl Mul<Magnitude, Magnitude> for () {
    type R = Magnitude;
    fn mul(a: Magnitude, b: Magnitude) -> Self::R {
      Self::R { s: ((a.s * b.s) + (a.s * b.s)), xyw: ((a.s * b.xyw) + (a.xyw * b.s)) }
    }
  }
  // --- Antimul ---
  impl Antimul<f64, Trivec> for () {
    type R = f64;
    fn antimul(a: f64, b: Trivec) -> Self::R {
      (a * b.xyw)
    }
  }
  impl Antimul<f64, Oddvec> for () {
    type R = f64;
    fn antimul(a: f64, b: Oddvec) -> Self::R {
      (a * b.xyw)
    }
  }
  impl Antimul<f64, Magnitude> for () {
    type R = f64;
    fn antimul(a: f64, b: Magnitude) -> Self::R {
      (a * b.xyw)
    }
  }
  impl Antimul<Univec, Univec> for () {
    type R = Trivec;
    fn antimul(a: Univec, b: Univec) -> Self::R {
      Self::R { xyw: (a.w * b.w) }
    }
  }
  impl Antimul<Univec, Bivec> for () {
    type R = f64;
    fn antimul(a: Univec, b: Bivec) -> Self::R {
      (((a.x * b.yw) + -(a.y * b.xw)) + (a.w * b.xy))
    }
  }
  impl Antimul<Univec, Trivec> for () {
    type R = Univec;
    fn antimul(a: Univec, b: Trivec) -> Self::R {
      Self::R { x: (a.x * b.xyw), y: (a.y * b.xyw), w: (a.w * b.xyw) }
    }
  }
  impl Antimul<Univec, Evenvec> for () {
    type R = f64;
    fn antimul(a: Univec, b: Evenvec) -> Self::R {
      (((a.x * b.yw) + -(a.y * b.xw)) + (a.w * b.xy))
    }
  }
  impl Antimul<Univec, Oddvec> for () {
    type R = Oddvec;
    fn antimul(a: Univec, b: Oddvec) -> Self::R {
      Self::R { x: (a.x * b.xyw), y: (a.y * b.xyw), w: (a.w * b.xyw), xyw: (a.w * b.w) }
    }
  }
  impl Antimul<Univec, Magnitude> for () {
    type R = Univec;
    fn antimul(a: Univec, b: Magnitude) -> Self::R {
      Self::R { x: (a.x * b.xyw), y: (a.y * b.xyw), w: (a.w * b.xyw) }
    }
  }
  impl Antimul<Bivec, Univec> for () {
    type R = f64;
    fn antimul(a: Bivec, b: Univec) -> Self::R {
      (((a.xy * b.w) + -(a.xw * b.y)) + (a.yw * b.x))
    }
  }
  impl Antimul<Bivec, Bivec> for () {
    type R = Oddvec;
    fn antimul(a: Bivec, b: Bivec) -> Self::R {
      Self::R { x: (-(a.xy * b.xw) + (a.xw * b.xy)), y: (-(a.xy * b.yw) + (a.yw * b.xy)), w: (-(a.xw * b.yw) + (a.yw * b.xw)), xyw: ((a.xw * b.xw) + (a.yw * b.yw)) }
    }
  }
  impl Antimul<Bivec, Trivec> for () {
    type R = Bivec;
    fn antimul(a: Bivec, b: Trivec) -> Self::R {
      Self::R { xy: (a.xy * b.xyw), xw: (a.xw * b.xyw), yw: (a.yw * b.xyw) }
    }
  }
  impl Antimul<Bivec, Evenvec> for () {
    type R = Oddvec;
    fn antimul(a: Bivec, b: Evenvec) -> Self::R {
      Self::R { x: (-(a.xy * b.xw) + (a.xw * b.xy)), y: (-(a.xy * b.yw) + (a.yw * b.xy)), w: (-(a.xw * b.yw) + (a.yw * b.xw)), xyw: ((a.xw * b.xw) + (a.yw * b.yw)) }
    }
  }
  impl Antimul<Bivec, Oddvec> for () {
    type R = Evenvec;
    fn antimul(a: Bivec, b: Oddvec) -> Self::R {
      Self::R { s: (((a.xy * b.w) + -(a.xw * b.y)) + (a.yw * b.x)), xy: (a.xy * b.xyw), xw: (a.xw * b.xyw), yw: (a.yw * b.xyw) }
    }
  }
  impl Antimul<Bivec, Magnitude> for () {
    type R = Bivec;
    fn antimul(a: Bivec, b: Magnitude) -> Self::R {
      Self::R { xy: (a.xy * b.xyw), xw: (a.xw * b.xyw), yw: (a.yw * b.xyw) }
    }
  }
  impl Antimul<Trivec, f64> for () {
    type R = f64;
    fn antimul(a: Trivec, b: f64) -> Self::R {
      (a.xyw * b)
    }
  }
  impl Antimul<Trivec, Univec> for () {
    type R = Univec;
    fn antimul(a: Trivec, b: Univec) -> Self::R {
      Self::R { x: (a.xyw * b.x), y: (a.xyw * b.y), w: (a.xyw * b.w) }
    }
  }
  impl Antimul<Trivec, Bivec> for () {
    type R = Bivec;
    fn antimul(a: Trivec, b: Bivec) -> Self::R {
      Self::R { xy: (a.xyw * b.xy), xw: (a.xyw * b.xw), yw: (a.xyw * b.yw) }
    }
  }
  impl Antimul<Trivec, Trivec> for () {
    type R = Trivec;
    fn antimul(a: Trivec, b: Trivec) -> Self::R {
      Self::R { xyw: ((a.xyw * b.xyw) + (a.xyw * b.xyw)) }
    }
  }
  impl Antimul<Trivec, Evenvec> for () {
    type R = Evenvec;
    fn antimul(a: Trivec, b: Evenvec) -> Self::R {
      Self::R { s: (a.xyw * b.s), xy: (a.xyw * b.xy), xw: (a.xyw * b.xw), yw: (a.xyw * b.yw) }
    }
  }
  impl Antimul<Trivec, Oddvec> for () {
    type R = Oddvec;
    fn antimul(a: Trivec, b: Oddvec) -> Self::R {
      Self::R { x: (a.xyw * b.x), y: (a.xyw * b.y), w: (a.xyw * b.w), xyw: ((a.xyw * b.xyw) + (a.xyw * b.xyw)) }
    }
  }
  impl Antimul<Trivec, Magnitude> for () {
    type R = Magnitude;
    fn antimul(a: Trivec, b: Magnitude) -> Self::R {
      Self::R { s: (a.xyw * b.s), xyw: ((a.xyw * b.xyw) + (a.xyw * b.xyw)) }
    }
  }
  impl Antimul<Evenvec, Univec> for () {
    type R = f64;
    fn antimul(a: Evenvec, b: Univec) -> Self::R {
      (((a.xy * b.w) + -(a.xw * b.y)) + (a.yw * b.x))
    }
  }
  impl Antimul<Evenvec, Bivec> for () {
    type R = Oddvec;
    fn antimul(a: Evenvec, b: Bivec) -> Self::R {
      Self::R { x: (-(a.xy * b.xw) + (a.xw * b.xy)), y: (-(a.xy * b.yw) + (a.yw * b.xy)), w: (-(a.xw * b.yw) + (a.yw * b.xw)), xyw: ((a.xw * b.xw) + (a.yw * b.yw)) }
    }
  }
  impl Antimul<Evenvec, Trivec> for () {
    type R = Evenvec;
    fn antimul(a: Evenvec, b: Trivec) -> Self::R {
      Self::R { s: (a.s * b.xyw), xy: (a.xy * b.xyw), xw: (a.xw * b.xyw), yw: (a.yw * b.xyw) }
    }
  }
  impl Antimul<Evenvec, Evenvec> for () {
    type R = Oddvec;
    fn antimul(a: Evenvec, b: Evenvec) -> Self::R {
      Self::R { x: (-(a.xy * b.xw) + (a.xw * b.xy)), y: (-(a.xy * b.yw) + (a.yw * b.xy)), w: (-(a.xw * b.yw) + (a.yw * b.xw)), xyw: ((a.xw * b.xw) + (a.yw * b.yw)) }
    }
  }
  impl Antimul<Evenvec, Oddvec> for () {
    type R = Evenvec;
    fn antimul(a: Evenvec, b: Oddvec) -> Self::R {
      Self::R { s: ((((a.s * b.xyw) + (a.xy * b.w)) + -(a.xw * b.y)) + (a.yw * b.x)), xy: (a.xy * b.xyw), xw: (a.xw * b.xyw), yw: (a.yw * b.xyw) }
    }
  }
  impl Antimul<Evenvec, Magnitude> for () {
    type R = Evenvec;
    fn antimul(a: Evenvec, b: Magnitude) -> Self::R {
      Self::R { s: (a.s * b.xyw), xy: (a.xy * b.xyw), xw: (a.xw * b.xyw), yw: (a.yw * b.xyw) }
    }
  }
  impl Antimul<Oddvec, f64> for () {
    type R = f64;
    fn antimul(a: Oddvec, b: f64) -> Self::R {
      (a.xyw * b)
    }
  }
  impl Antimul<Oddvec, Univec> for () {
    type R = Oddvec;
    fn antimul(a: Oddvec, b: Univec) -> Self::R {
      Self::R { x: (a.xyw * b.x), y: (a.xyw * b.y), w: (a.xyw * b.w), xyw: (a.w * b.w) }
    }
  }
  impl Antimul<Oddvec, Bivec> for () {
    type R = Evenvec;
    fn antimul(a: Oddvec, b: Bivec) -> Self::R {
      Self::R { s: (((a.x * b.yw) + -(a.y * b.xw)) + (a.w * b.xy)), xy: (a.xyw * b.xy), xw: (a.xyw * b.xw), yw: (a.xyw * b.yw) }
    }
  }
  impl Antimul<Oddvec, Trivec> for () {
    type R = Oddvec;
    fn antimul(a: Oddvec, b: Trivec) -> Self::R {
      Self::R { x: (a.x * b.xyw), y: (a.y * b.xyw), w: (a.w * b.xyw), xyw: ((a.xyw * b.xyw) + (a.xyw * b.xyw)) }
    }
  }
  impl Antimul<Oddvec, Evenvec> for () {
    type R = Evenvec;
    fn antimul(a: Oddvec, b: Evenvec) -> Self::R {
      Self::R { s: ((((a.x * b.yw) + -(a.y * b.xw)) + (a.w * b.xy)) + (a.xyw * b.s)), xy: (a.xyw * b.xy), xw: (a.xyw * b.xw), yw: (a.xyw * b.yw) }
    }
  }
  impl Antimul<Oddvec, Oddvec> for () {
    type R = Oddvec;
    fn antimul(a: Oddvec, b: Oddvec) -> Self::R {
      Self::R { x: ((a.x * b.xyw) + (a.xyw * b.x)), y: ((a.y * b.xyw) + (a.xyw * b.y)), w: ((a.w * b.xyw) + (a.xyw * b.w)), xyw: ((a.xyw * b.xyw) + ((a.w * b.w) + (a.xyw * b.xyw))) }
    }
  }
  //impl Antimul<Oddvec, Magnitude> for () {
  //  type R = <ERROR>;
  //  fn antimul(a: Oddvec, b: Magnitude) -> Self::R {
  //    Self::R { s: (a.xyw * b.s), x: (a.x * b.xyw), y: (a.y * b.xyw), w: (a.w * b.xyw), xyw: ((a.xyw * b.xyw) + (a.xyw * b.xyw)) }
  //  }
  //}
  impl Antimul<Magnitude, f64> for () {
    type R = f64;
    fn antimul(a: Magnitude, b: f64) -> Self::R {
      (a.xyw * b)
    }
  }
  impl Antimul<Magnitude, Univec> for () {
    type R = Univec;
    fn antimul(a: Magnitude, b: Univec) -> Self::R {
      Self::R { x: (a.xyw * b.x), y: (a.xyw * b.y), w: (a.xyw * b.w) }
    }
  }
  impl Antimul<Magnitude, Bivec> for () {
    type R = Bivec;
    fn antimul(a: Magnitude, b: Bivec) -> Self::R {
      Self::R { xy: (a.xyw * b.xy), xw: (a.xyw * b.xw), yw: (a.xyw * b.yw) }
    }
  }
  impl Antimul<Magnitude, Trivec> for () {
    type R = Magnitude;
    fn antimul(a: Magnitude, b: Trivec) -> Self::R {
      Self::R { s: (a.s * b.xyw), xyw: ((a.xyw * b.xyw) + (a.xyw * b.xyw)) }
    }
  }
  impl Antimul<Magnitude, Evenvec> for () {
    type R = Evenvec;
    fn antimul(a: Magnitude, b: Evenvec) -> Self::R {
      Self::R { s: (a.xyw * b.s), xy: (a.xyw * b.xy), xw: (a.xyw * b.xw), yw: (a.xyw * b.yw) }
    }
  }
  //impl Antimul<Magnitude, Oddvec> for () {
  //  type R = <ERROR>;
  //  fn antimul(a: Magnitude, b: Oddvec) -> Self::R {
  //    Self::R { s: (a.s * b.xyw), x: (a.xyw * b.x), y: (a.xyw * b.y), w: (a.xyw * b.w), xyw: ((a.xyw * b.xyw) + (a.xyw * b.xyw)) }
  //  }
  //}
  impl Antimul<Magnitude, Magnitude> for () {
    type R = Magnitude;
    fn antimul(a: Magnitude, b: Magnitude) -> Self::R {
      Self::R { s: ((a.s * b.xyw) + (a.xyw * b.s)), xyw: ((a.xyw * b.xyw) + (a.xyw * b.xyw)) }
    }
  }
  // --- Attitude ---
  impl Attitude<Univec> for () {
    type R = f64;
    fn attitude(a: Univec) -> Self::R {
      a.w
    }
  }
  impl Attitude<Bivec> for () {
    type R = Univec;
    fn attitude(a: Bivec) -> Self::R {
      Self::R { x: a.xw, y: a.yw, w: 0.0 }
    }
  }
  impl Attitude<Trivec> for () {
    type R = Bivec;
    fn attitude(a: Trivec) -> Self::R {
      Self::R { xy: a.xyw, xw: 0.0, yw: 0.0 }
    }
  }
  impl Attitude<Evenvec> for () {
    type R = Univec;
    fn attitude(a: Evenvec) -> Self::R {
      Self::R { x: a.xw, y: a.yw, w: 0.0 }
    }
  }
  impl Attitude<Oddvec> for () {
    type R = Evenvec;
    fn attitude(a: Oddvec) -> Self::R {
      Self::R { s: a.w, xy: a.xyw, xw: 0.0, yw: 0.0 }
    }
  }
  impl Attitude<Magnitude> for () {
    type R = Bivec;
    fn attitude(a: Magnitude) -> Self::R {
      Self::R { xy: a.xyw, xw: 0.0, yw: 0.0 }
    }
  }
  // --- BulkNormSq ---
  //impl BulkNormSq<f64> for () {
  //  type R = f64;
  //  fn bulk_norm_sq(a: f64) -> Self::R {
  //    (a * a)
  //  }
  //}
  impl BulkNormSq<Univec> for () {
    type R = f64;
    fn bulk_norm_sq(a: Univec) -> Self::R {
      ((a.x * a.x) + (a.y * a.y))
    }
  }
  impl BulkNormSq<Bivec> for () {
    type R = f64;
    fn bulk_norm_sq(a: Bivec) -> Self::R {
      (a.xy * a.xy)
    }
  }
  impl BulkNormSq<Evenvec> for () {
    type R = f64;
    fn bulk_norm_sq(a: Evenvec) -> Self::R {
      ((a.s * a.s) + (a.xy * a.xy))
    }
  }
  impl BulkNormSq<Oddvec> for () {
    type R = f64;
    fn bulk_norm_sq(a: Oddvec) -> Self::R {
      ((a.x * a.x) + (a.y * a.y))
    }
  }
  impl BulkNormSq<Magnitude> for () {
    type R = f64;
    fn bulk_norm_sq(a: Magnitude) -> Self::R {
      (a.s * a.s)
    }
  }
  // --- WeightNormSq ---
  impl WeightNormSq<Univec> for () {
    type R = Trivec;
    fn weight_norm_sq(a: Univec) -> Self::R {
      Self::R { xyw: (a.w * a.w) }
    }
  }
  impl WeightNormSq<Bivec> for () {
    type R = Trivec;
    fn weight_norm_sq(a: Bivec) -> Self::R {
      Self::R { xyw: ((a.xw * a.xw) + (a.yw * a.yw)) }
    }
  }
  impl WeightNormSq<Trivec> for () {
    type R = Trivec;
    fn weight_norm_sq(a: Trivec) -> Self::R {
      Self::R { xyw: (a.xyw * a.xyw) }
    }
  }
  impl WeightNormSq<Evenvec> for () {
    type R = Trivec;
    fn weight_norm_sq(a: Evenvec) -> Self::R {
      Self::R { xyw: ((a.xw * a.xw) + (a.yw * a.yw)) }
    }
  }
  impl WeightNormSq<Oddvec> for () {
    type R = Trivec;
    fn weight_norm_sq(a: Oddvec) -> Self::R {
      Self::R { xyw: ((a.w * a.w) + (a.xyw * a.xyw)) }
    }
  }
  impl WeightNormSq<Magnitude> for () {
    type R = Trivec;
    fn weight_norm_sq(a: Magnitude) -> Self::R {
      Self::R { xyw: (a.xyw * a.xyw) }
    }
  }
  // --- NormSq ---
  //impl NormSq<f64> for () {
  //  type R = f64;
  //  fn norm_sq(a: f64) -> Self::R {
  //    (a * a)
  //  }
  //}
  impl NormSq<Univec> for () {
    type R = Magnitude;
    fn norm_sq(a: Univec) -> Self::R {
      Self::R { s: ((a.x * a.x) + (a.y * a.y)), xyw: (a.w * a.w) }
    }
  }
  impl NormSq<Bivec> for () {
    type R = Magnitude;
    fn norm_sq(a: Bivec) -> Self::R {
      Self::R { s: (a.xy * a.xy), xyw: ((a.xw * a.xw) + (a.yw * a.yw)) }
    }
  }
  impl NormSq<Trivec> for () {
    type R = Trivec;
    fn norm_sq(a: Trivec) -> Self::R {
      Self::R { xyw: (a.xyw * a.xyw) }
    }
  }
  impl NormSq<Evenvec> for () {
    type R = Magnitude;
    fn norm_sq(a: Evenvec) -> Self::R {
      Self::R { s: ((a.s * a.s) + (a.xy * a.xy)), xyw: ((a.xw * a.xw) + (a.yw * a.yw)) }
    }
  }
  impl NormSq<Oddvec> for () {
    type R = Magnitude;
    fn norm_sq(a: Oddvec) -> Self::R {
      Self::R { s: ((a.x * a.x) + (a.y * a.y)), xyw: ((a.w * a.w) + (a.xyw * a.xyw)) }
    }
  }
  impl NormSq<Magnitude> for () {
    type R = Magnitude;
    fn norm_sq(a: Magnitude) -> Self::R {
      Self::R { s: (a.s * a.s), xyw: (a.xyw * a.xyw) }
    }
  }
  // --- DistanceSq ---
  impl DistanceSq<f64, Univec> for () {
    type R = f64;
    fn distance_sq(a: f64, b: Univec) -> Self::R {
      ((a * b.w) * (a * b.w))
    }
  }
  impl DistanceSq<f64, Bivec> for () {
    type R = f64;
    fn distance_sq(a: f64, b: Bivec) -> Self::R {
      (((a * b.xw) * (a * b.xw)) + ((a * b.yw) * (a * b.yw)))
    }
  }
  impl DistanceSq<f64, Trivec> for () {
    type R = f64;
    fn distance_sq(a: f64, b: Trivec) -> Self::R {
      ((a * b.xyw) * (a * b.xyw))
    }
  }
  impl DistanceSq<f64, Evenvec> for () {
    type R = f64;
    fn distance_sq(a: f64, b: Evenvec) -> Self::R {
      (((a * b.xw) * (a * b.xw)) + ((a * b.yw) * (a * b.yw)))
    }
  }
  impl DistanceSq<f64, Oddvec> for () {
    type R = f64;
    fn distance_sq(a: f64, b: Oddvec) -> Self::R {
      (((a * b.w) * (a * b.w)) + ((a * b.xyw) * (a * b.xyw)))
    }
  }
  impl DistanceSq<f64, Magnitude> for () {
    type R = f64;
    fn distance_sq(a: f64, b: Magnitude) -> Self::R {
      ((a * b.xyw) * (a * b.xyw))
    }
  }
  impl DistanceSq<Univec, f64> for () {
    type R = f64;
    fn distance_sq(a: Univec, b: f64) -> Self::R {
      ((a.w * b) * (a.w * b))
    }
  }
  impl DistanceSq<Univec, Univec> for () {
    type R = Magnitude;
    fn distance_sq(a: Univec, b: Univec) -> Self::R {
      Self::R { s: ((((a.x * b.w) + -(a.w * b.x)) * ((a.x * b.w) + -(a.w * b.x))) + (((a.y * b.w) + -(a.w * b.y)) * ((a.y * b.w) + -(a.w * b.y)))), xyw: ((a.w * b.w) * (a.w * b.w)) }
    }
  }
  impl DistanceSq<Univec, Bivec> for () {
    type R = Magnitude;
    fn distance_sq(a: Univec, b: Bivec) -> Self::R {
      Self::R { s: ((((a.x * b.yw) + -(a.y * b.xw)) + (a.w * b.xy)) * (((a.x * b.yw) + -(a.y * b.xw)) + (a.w * b.xy))), xyw: ((-(a.w * b.xw) * -(a.w * b.xw)) + (-(a.w * b.yw) * -(a.w * b.yw))) }
    }
  }
  impl DistanceSq<Univec, Trivec> for () {
    type R = Trivec;
    fn distance_sq(a: Univec, b: Trivec) -> Self::R {
      Self::R { xyw: ((a.w * b.xyw) * (a.w * b.xyw)) }
    }
  }
  impl DistanceSq<Univec, Evenvec> for () {
    type R = Magnitude;
    fn distance_sq(a: Univec, b: Evenvec) -> Self::R {
      Self::R { s: (((a.w * b.s) * (a.w * b.s)) + ((((a.x * b.yw) + -(a.y * b.xw)) + (a.w * b.xy)) * (((a.x * b.yw) + -(a.y * b.xw)) + (a.w * b.xy)))), xyw: ((-(a.w * b.xw) * -(a.w * b.xw)) + (-(a.w * b.yw) * -(a.w * b.yw))) }
    }
  }
  impl DistanceSq<Univec, Oddvec> for () {
    type R = Magnitude;
    fn distance_sq(a: Univec, b: Oddvec) -> Self::R {
      Self::R { s: ((((a.x * b.w) + -(a.w * b.x)) * ((a.x * b.w) + -(a.w * b.x))) + (((a.y * b.w) + -(a.w * b.y)) * ((a.y * b.w) + -(a.w * b.y)))), xyw: (((a.w * b.w) * (a.w * b.w)) + ((a.w * b.xyw) * (a.w * b.xyw))) }
    }
  }
  impl DistanceSq<Univec, Magnitude> for () {
    type R = Magnitude;
    fn distance_sq(a: Univec, b: Magnitude) -> Self::R {
      Self::R { s: ((a.w * b.s) * (a.w * b.s)), xyw: ((a.w * b.xyw) * (a.w * b.xyw)) }
    }
  }
  impl DistanceSq<Bivec, f64> for () {
    type R = f64;
    fn distance_sq(a: Bivec, b: f64) -> Self::R {
      (((a.xw * b) * (a.xw * b)) + ((a.yw * b) * (a.yw * b)))
    }
  }
  impl DistanceSq<Bivec, Univec> for () {
    type R = Magnitude;
    fn distance_sq(a: Bivec, b: Univec) -> Self::R {
      Self::R { s: ((((a.xy * b.w) + -(a.xw * b.y)) + (a.yw * b.x)) * (((a.xy * b.w) + -(a.xw * b.y)) + (a.yw * b.x))), xyw: (((a.xw * b.w) * (a.xw * b.w)) + ((a.yw * b.w) * (a.yw * b.w))) }
    }
  }
  impl DistanceSq<Bivec, Bivec> for () {
    type R = Trivec;
    fn distance_sq(a: Bivec, b: Bivec) -> Self::R {
      Self::R { xyw: ((-(a.xw * b.yw) + (a.yw * b.xw)) * (-(a.xw * b.yw) + (a.yw * b.xw))) }
    }
  }
  impl DistanceSq<Bivec, Evenvec> for () {
    type R = Magnitude;
    fn distance_sq(a: Bivec, b: Evenvec) -> Self::R {
      Self::R { s: (((a.xw * b.s) * (a.xw * b.s)) + ((a.yw * b.s) * (a.yw * b.s))), xyw: ((-(a.xw * b.yw) + (a.yw * b.xw)) * (-(a.xw * b.yw) + (a.yw * b.xw))) }
    }
  }
  impl DistanceSq<Bivec, Oddvec> for () {
    type R = Magnitude;
    fn distance_sq(a: Bivec, b: Oddvec) -> Self::R {
      Self::R { s: ((((a.xy * b.w) + -(a.xw * b.y)) + (a.yw * b.x)) * (((a.xy * b.w) + -(a.xw * b.y)) + (a.yw * b.x))), xyw: (((a.xw * b.w) * (a.xw * b.w)) + ((a.yw * b.w) * (a.yw * b.w))) }
    }
  }
  impl DistanceSq<Bivec, Magnitude> for () {
    type R = f64;
    fn distance_sq(a: Bivec, b: Magnitude) -> Self::R {
      (((a.xw * b.s) * (a.xw * b.s)) + ((a.yw * b.s) * (a.yw * b.s)))
    }
  }
  impl DistanceSq<Trivec, f64> for () {
    type R = f64;
    fn distance_sq(a: Trivec, b: f64) -> Self::R {
      ((a.xyw * b) * (a.xyw * b))
    }
  }
  impl DistanceSq<Trivec, Univec> for () {
    type R = Trivec;
    fn distance_sq(a: Trivec, b: Univec) -> Self::R {
      Self::R { xyw: ((a.xyw * b.w) * (a.xyw * b.w)) }
    }
  }
  impl DistanceSq<Trivec, Evenvec> for () {
    type R = f64;
    fn distance_sq(a: Trivec, b: Evenvec) -> Self::R {
      ((a.xyw * b.s) * (a.xyw * b.s))
    }
  }
  impl DistanceSq<Trivec, Oddvec> for () {
    type R = Trivec;
    fn distance_sq(a: Trivec, b: Oddvec) -> Self::R {
      Self::R { xyw: ((a.xyw * b.w) * (a.xyw * b.w)) }
    }
  }
  impl DistanceSq<Trivec, Magnitude> for () {
    type R = f64;
    fn distance_sq(a: Trivec, b: Magnitude) -> Self::R {
      ((a.xyw * b.s) * (a.xyw * b.s))
    }
  }
  impl DistanceSq<Evenvec, f64> for () {
    type R = f64;
    fn distance_sq(a: Evenvec, b: f64) -> Self::R {
      (((a.xw * b) * (a.xw * b)) + ((a.yw * b) * (a.yw * b)))
    }
  }
  impl DistanceSq<Evenvec, Univec> for () {
    type R = Magnitude;
    fn distance_sq(a: Evenvec, b: Univec) -> Self::R {
      Self::R { s: (((a.s * b.w) * (a.s * b.w)) + ((((a.xy * b.w) + -(a.xw * b.y)) + (a.yw * b.x)) * (((a.xy * b.w) + -(a.xw * b.y)) + (a.yw * b.x)))), xyw: (((a.xw * b.w) * (a.xw * b.w)) + ((a.yw * b.w) * (a.yw * b.w))) }
    }
  }
  impl DistanceSq<Evenvec, Bivec> for () {
    type R = Magnitude;
    fn distance_sq(a: Evenvec, b: Bivec) -> Self::R {
      Self::R { s: (((a.s * b.xw) * (a.s * b.xw)) + ((a.s * b.yw) * (a.s * b.yw))), xyw: ((-(a.xw * b.yw) + (a.yw * b.xw)) * (-(a.xw * b.yw) + (a.yw * b.xw))) }
    }
  }
  impl DistanceSq<Evenvec, Trivec> for () {
    type R = f64;
    fn distance_sq(a: Evenvec, b: Trivec) -> Self::R {
      ((a.s * b.xyw) * (a.s * b.xyw))
    }
  }
  impl DistanceSq<Evenvec, Evenvec> for () {
    type R = Magnitude;
    fn distance_sq(a: Evenvec, b: Evenvec) -> Self::R {
      Self::R { s: ((((a.s * b.xw) + (a.xw * b.s)) * ((a.s * b.xw) + (a.xw * b.s))) + (((a.s * b.yw) + (a.yw * b.s)) * ((a.s * b.yw) + (a.yw * b.s)))), xyw: ((-(a.xw * b.yw) + (a.yw * b.xw)) * (-(a.xw * b.yw) + (a.yw * b.xw))) }
    }
  }
  impl DistanceSq<Evenvec, Oddvec> for () {
    type R = Magnitude;
    fn distance_sq(a: Evenvec, b: Oddvec) -> Self::R {
      Self::R { s: (((a.s * b.w) * (a.s * b.w)) + (((((a.s * b.xyw) + (a.xy * b.w)) + -(a.xw * b.y)) + (a.yw * b.x)) * ((((a.s * b.xyw) + (a.xy * b.w)) + -(a.xw * b.y)) + (a.yw * b.x)))), xyw: (((a.xw * b.w) * (a.xw * b.w)) + ((a.yw * b.w) * (a.yw * b.w))) }
    }
  }
  impl DistanceSq<Evenvec, Magnitude> for () {
    type R = f64;
    fn distance_sq(a: Evenvec, b: Magnitude) -> Self::R {
      ((((a.xw * b.s) * (a.xw * b.s)) + ((a.yw * b.s) * (a.yw * b.s))) + ((a.s * b.xyw) * (a.s * b.xyw)))
    }
  }
  impl DistanceSq<Oddvec, f64> for () {
    type R = f64;
    fn distance_sq(a: Oddvec, b: f64) -> Self::R {
      (((a.w * b) * (a.w * b)) + ((a.xyw * b) * (a.xyw * b)))
    }
  }
  impl DistanceSq<Oddvec, Univec> for () {
    type R = Magnitude;
    fn distance_sq(a: Oddvec, b: Univec) -> Self::R {
      Self::R { s: ((((a.x * b.w) + -(a.w * b.x)) * ((a.x * b.w) + -(a.w * b.x))) + (((a.y * b.w) + -(a.w * b.y)) * ((a.y * b.w) + -(a.w * b.y)))), xyw: (((a.w * b.w) * (a.w * b.w)) + ((a.xyw * b.w) * (a.xyw * b.w))) }
    }
  }
  impl DistanceSq<Oddvec, Bivec> for () {
    type R = Magnitude;
    fn distance_sq(a: Oddvec, b: Bivec) -> Self::R {
      Self::R { s: ((((a.x * b.yw) + -(a.y * b.xw)) + (a.w * b.xy)) * (((a.x * b.yw) + -(a.y * b.xw)) + (a.w * b.xy))), xyw: ((-(a.w * b.xw) * -(a.w * b.xw)) + (-(a.w * b.yw) * -(a.w * b.yw))) }
    }
  }
  impl DistanceSq<Oddvec, Trivec> for () {
    type R = Trivec;
    fn distance_sq(a: Oddvec, b: Trivec) -> Self::R {
      Self::R { xyw: ((a.w * b.xyw) * (a.w * b.xyw)) }
    }
  }
  impl DistanceSq<Oddvec, Evenvec> for () {
    type R = Magnitude;
    fn distance_sq(a: Oddvec, b: Evenvec) -> Self::R {
      Self::R { s: (((a.w * b.s) * (a.w * b.s)) + (((((a.x * b.yw) + -(a.y * b.xw)) + (a.w * b.xy)) + (a.xyw * b.s)) * ((((a.x * b.yw) + -(a.y * b.xw)) + (a.w * b.xy)) + (a.xyw * b.s)))), xyw: ((-(a.w * b.xw) * -(a.w * b.xw)) + (-(a.w * b.yw) * -(a.w * b.yw))) }
    }
  }
  impl DistanceSq<Oddvec, Oddvec> for () {
    type R = Magnitude;
    fn distance_sq(a: Oddvec, b: Oddvec) -> Self::R {
      Self::R { s: ((((a.x * b.w) + -(a.w * b.x)) * ((a.x * b.w) + -(a.w * b.x))) + (((a.y * b.w) + -(a.w * b.y)) * ((a.y * b.w) + -(a.w * b.y)))), xyw: (((a.w * b.w) * (a.w * b.w)) + (((a.w * b.xyw) + (a.xyw * b.w)) * ((a.w * b.xyw) + (a.xyw * b.w)))) }
    }
  }
  impl DistanceSq<Oddvec, Magnitude> for () {
    type R = Magnitude;
    fn distance_sq(a: Oddvec, b: Magnitude) -> Self::R {
      Self::R { s: (((a.w * b.s) * (a.w * b.s)) + ((a.xyw * b.s) * (a.xyw * b.s))), xyw: ((a.w * b.xyw) * (a.w * b.xyw)) }
    }
  }
  impl DistanceSq<Magnitude, f64> for () {
    type R = f64;
    fn distance_sq(a: Magnitude, b: f64) -> Self::R {
      ((a.xyw * b) * (a.xyw * b))
    }
  }
  impl DistanceSq<Magnitude, Univec> for () {
    type R = Magnitude;
    fn distance_sq(a: Magnitude, b: Univec) -> Self::R {
      Self::R { s: ((a.s * b.w) * (a.s * b.w)), xyw: ((a.xyw * b.w) * (a.xyw * b.w)) }
    }
  }
  impl DistanceSq<Magnitude, Bivec> for () {
    type R = f64;
    fn distance_sq(a: Magnitude, b: Bivec) -> Self::R {
      (((a.s * b.xw) * (a.s * b.xw)) + ((a.s * b.yw) * (a.s * b.yw)))
    }
  }
  impl DistanceSq<Magnitude, Trivec> for () {
    type R = f64;
    fn distance_sq(a: Magnitude, b: Trivec) -> Self::R {
      ((a.s * b.xyw) * (a.s * b.xyw))
    }
  }
  impl DistanceSq<Magnitude, Evenvec> for () {
    type R = f64;
    fn distance_sq(a: Magnitude, b: Evenvec) -> Self::R {
      ((((a.s * b.xw) * (a.s * b.xw)) + ((a.s * b.yw) * (a.s * b.yw))) + ((a.xyw * b.s) * (a.xyw * b.s)))
    }
  }
  impl DistanceSq<Magnitude, Oddvec> for () {
    type R = Magnitude;
    fn distance_sq(a: Magnitude, b: Oddvec) -> Self::R {
      Self::R { s: (((a.s * b.w) * (a.s * b.w)) + ((a.s * b.xyw) * (a.s * b.xyw))), xyw: ((a.xyw * b.w) * (a.xyw * b.w)) }
    }
  }
  impl DistanceSq<Magnitude, Magnitude> for () {
    type R = f64;
    fn distance_sq(a: Magnitude, b: Magnitude) -> Self::R {
      (((a.s * b.xyw) + (a.xyw * b.s)) * ((a.s * b.xyw) + (a.xyw * b.s)))
    }
  }
  // --- CosOfAngleSq ---
  impl CosOfAngleSq<Univec, Univec> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Univec, b: Univec) -> Self::R {
      ((a.w * b.w) * (a.w * b.w))
    }
  }
  impl CosOfAngleSq<Univec, Oddvec> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Univec, b: Oddvec) -> Self::R {
      ((a.w * b.w) * (a.w * b.w))
    }
  }
  impl CosOfAngleSq<Bivec, Univec> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Bivec, b: Univec) -> Self::R {
      (((a.xw * b.w) * (a.xw * b.w)) + ((a.yw * b.w) * (a.yw * b.w)))
    }
  }
  impl CosOfAngleSq<Bivec, Bivec> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Bivec, b: Bivec) -> Self::R {
      ((-(a.xw * -b.xw) + (a.yw * b.yw)) * (-(a.xw * -b.xw) + (a.yw * b.yw)))
    }
  }
  impl CosOfAngleSq<Bivec, Evenvec> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Bivec, b: Evenvec) -> Self::R {
      ((-(a.xw * -b.xw) + (a.yw * b.yw)) * (-(a.xw * -b.xw) + (a.yw * b.yw)))
    }
  }
  impl CosOfAngleSq<Bivec, Oddvec> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Bivec, b: Oddvec) -> Self::R {
      (((a.xw * b.w) * (a.xw * b.w)) + ((a.yw * b.w) * (a.yw * b.w)))
    }
  }
  impl CosOfAngleSq<Trivec, Univec> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Trivec, b: Univec) -> Self::R {
      ((a.xyw * b.w) * (a.xyw * b.w))
    }
  }
  impl CosOfAngleSq<Trivec, Bivec> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Trivec, b: Bivec) -> Self::R {
      (((a.xyw * b.yw) * (a.xyw * b.yw)) + ((a.xyw * -b.xw) * (a.xyw * -b.xw)))
    }
  }
  impl CosOfAngleSq<Trivec, Trivec> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Trivec, b: Trivec) -> Self::R {
      ((a.xyw * b.xyw) * (a.xyw * b.xyw))
    }
  }
  impl CosOfAngleSq<Trivec, Evenvec> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Trivec, b: Evenvec) -> Self::R {
      (((a.xyw * b.yw) * (a.xyw * b.yw)) + ((a.xyw * -b.xw) * (a.xyw * -b.xw)))
    }
  }
  impl CosOfAngleSq<Trivec, Oddvec> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Trivec, b: Oddvec) -> Self::R {
      (((a.xyw * b.xyw) * (a.xyw * b.xyw)) + ((a.xyw * b.w) * (a.xyw * b.w)))
    }
  }
  impl CosOfAngleSq<Trivec, Magnitude> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Trivec, b: Magnitude) -> Self::R {
      ((a.xyw * b.xyw) * (a.xyw * b.xyw))
    }
  }
  impl CosOfAngleSq<Evenvec, Univec> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Evenvec, b: Univec) -> Self::R {
      (((a.xw * b.w) * (a.xw * b.w)) + ((a.yw * b.w) * (a.yw * b.w)))
    }
  }
  impl CosOfAngleSq<Evenvec, Bivec> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Evenvec, b: Bivec) -> Self::R {
      ((-(a.xw * -b.xw) + (a.yw * b.yw)) * (-(a.xw * -b.xw) + (a.yw * b.yw)))
    }
  }
  impl CosOfAngleSq<Evenvec, Evenvec> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Evenvec, b: Evenvec) -> Self::R {
      ((-(a.xw * -b.xw) + (a.yw * b.yw)) * (-(a.xw * -b.xw) + (a.yw * b.yw)))
    }
  }
  impl CosOfAngleSq<Evenvec, Oddvec> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Evenvec, b: Oddvec) -> Self::R {
      (((a.xw * b.w) * (a.xw * b.w)) + ((a.yw * b.w) * (a.yw * b.w)))
    }
  }
  impl CosOfAngleSq<Oddvec, Univec> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Oddvec, b: Univec) -> Self::R {
      (((a.w * b.w) * (a.w * b.w)) + ((a.xyw * b.w) * (a.xyw * b.w)))
    }
  }
  impl CosOfAngleSq<Oddvec, Bivec> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Oddvec, b: Bivec) -> Self::R {
      (((a.xyw * b.yw) * (a.xyw * b.yw)) + ((a.xyw * -b.xw) * (a.xyw * -b.xw)))
    }
  }
  impl CosOfAngleSq<Oddvec, Trivec> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Oddvec, b: Trivec) -> Self::R {
      ((a.xyw * b.xyw) * (a.xyw * b.xyw))
    }
  }
  impl CosOfAngleSq<Oddvec, Evenvec> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Oddvec, b: Evenvec) -> Self::R {
      (((a.xyw * b.yw) * (a.xyw * b.yw)) + ((a.xyw * -b.xw) * (a.xyw * -b.xw)))
    }
  }
  impl CosOfAngleSq<Oddvec, Oddvec> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Oddvec, b: Oddvec) -> Self::R {
      ((((a.w * b.w) + (a.xyw * b.xyw)) * ((a.w * b.w) + (a.xyw * b.xyw))) + ((a.xyw * b.w) * (a.xyw * b.w)))
    }
  }
  impl CosOfAngleSq<Oddvec, Magnitude> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Oddvec, b: Magnitude) -> Self::R {
      ((a.xyw * b.xyw) * (a.xyw * b.xyw))
    }
  }
  impl CosOfAngleSq<Magnitude, Univec> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Magnitude, b: Univec) -> Self::R {
      ((a.xyw * b.w) * (a.xyw * b.w))
    }
  }
  impl CosOfAngleSq<Magnitude, Bivec> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Magnitude, b: Bivec) -> Self::R {
      (((a.xyw * b.yw) * (a.xyw * b.yw)) + ((a.xyw * -b.xw) * (a.xyw * -b.xw)))
    }
  }
  impl CosOfAngleSq<Magnitude, Trivec> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Magnitude, b: Trivec) -> Self::R {
      ((a.xyw * b.xyw) * (a.xyw * b.xyw))
    }
  }
  impl CosOfAngleSq<Magnitude, Evenvec> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Magnitude, b: Evenvec) -> Self::R {
      (((a.xyw * b.yw) * (a.xyw * b.yw)) + ((a.xyw * -b.xw) * (a.xyw * -b.xw)))
    }
  }
  impl CosOfAngleSq<Magnitude, Oddvec> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Magnitude, b: Oddvec) -> Self::R {
      (((a.xyw * b.xyw) * (a.xyw * b.xyw)) + ((a.xyw * b.w) * (a.xyw * b.w)))
    }
  }
  impl CosOfAngleSq<Magnitude, Magnitude> for () {
    type R = f64;
    fn cos_of_angle_sq(a: Magnitude, b: Magnitude) -> Self::R {
      ((a.xyw * b.xyw) * (a.xyw * b.xyw))
    }
  }
  