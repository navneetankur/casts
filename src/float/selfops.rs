use std::ops;

use num_traits::Num;

use crate::{cast::{trunit::TrunIt, CastIt}, int::{CastInt, Int}};

use super::{CastFloat, Float};


impl CastFloat for Float {
    #[inline]
    fn f(self) -> Float {
        self
    }
}
impl CastInt for Float {
    #[inline]
    fn i(self) -> Int {
        Int::new(self.v.i32())
    }
}
impl CastIt for Float {
    #[inline]
    fn u(self) -> usize {
        self.v.u()
    }
    #[inline]
    fn usize(self) -> usize {
        self.v.u()
    }

    fn u8(self) ->  u8 {
        self.v.u8()
    }

    fn u16(self) -> u16 {
        self.v.u16()
    }

    fn u32(self) -> u32 {
        self.v.u32()
    }

    fn i8(self) ->  i8 {
        self.v.i8()
    }

    fn i16(self) -> i16 {
        self.v.i16()
    }

    fn i32(self) -> i32 {
        self.v.i32()
    }

    fn f32(self) -> f32 {
        self.v
    }
}
impl TrunIt for Float {
    fn tu(self) -> usize {
        self.v.tu()
    }

    fn tu8(self) ->  u8 {
        self.v.tu8()
    }

    fn tu16(self) -> u16 {
        self.v.tu16()
    }

    fn tu32(self) -> u32 {
        self.v.tu32()
    }

    fn ti8(self) ->  i8 {
        self.v.ti8()
    }

    fn ti16(self) -> i16 {
        self.v.ti16()
    }

    fn ti32(self) -> i32 {
        self.v.ti32()
    }
}
impl ops::Neg for Float {
    type Output = Float;

    #[inline]
    fn neg(self) -> Self::Output {
        Float::new(-self.v)
    }
}
impl ops::Add for Float {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            v: self.v + rhs.v
        }
    }
}
impl ops::Sub for Float {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            v: self.v - rhs.v
        }
    }
}
impl ops::Mul for Float {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            v: self.v * rhs.v
        }
    }
}
impl ops::Div for Float {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            v: self.v / rhs.v
        }
    }
}
impl ops::Rem for Float {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            v: self.v % rhs.v
        }
    }
}
impl ops::AddAssign for Float{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.v += rhs.v
    }
}
impl ops::SubAssign for Float{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.v -= rhs.v
    }
}
impl ops::MulAssign for Float{
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.v *= rhs.v
    }
}
impl ops::DivAssign for Float{
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.v /= rhs.v
    }
}
impl ops::RemAssign for Float{
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        self.v %= rhs.v
    }
}

impl Num for Float {
    type FromStrRadixErr = <f32 as Num>::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let v = <f32 as Num>::from_str_radix(str, radix)?;
        Ok(Float::new(v))
    }
}
impl num_traits::Zero for Float {
    fn zero() -> Self {
        Float::new(0.)
    }

    fn is_zero(&self) -> bool {
        self.v == 0.
    }
}
impl num_traits::One for Float {
    fn one() -> Self {
        Float::new(1.)
    }
}
