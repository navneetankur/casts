use std::ops;
use num_traits::Num;

use crate::{cast::CastIt, float::{CastFloat, Float}};
use super::{CastInt, Int};
impl CastInt for Int {
    #[inline]
    fn i(self) -> Int {
        self
    }
}
impl CastFloat for Int {
    #[inline]
    fn f(self) -> Float {
        Float::new(self.v as f32)
    }
}
impl CastIt for Int {
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
        self.v.f32()
    }
}
impl ops::Neg for Int {
    type Output = Int;

    #[inline]
    fn neg(self) -> Self::Output {
        Int::new(-self.v)
    }
}
impl ops::Add for Int {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            v: self.v + rhs.v
        }
    }
}
impl ops::Sub for Int {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            v: self.v - rhs.v
        }
    }
}
impl ops::Mul for Int {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            v: self.v * rhs.v
        }
    }
}
impl ops::Div for Int {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            v: self.v / rhs.v
        }
    }
}
impl ops::Rem for Int {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            v: self.v % rhs.v
        }
    }
}
impl ops::AddAssign for Int{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.v += rhs.v
    }
}
impl ops::SubAssign for Int{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.v -= rhs.v
    }
}
impl ops::MulAssign for Int{
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.v *= rhs.v
    }
}
impl ops::DivAssign for Int{
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.v /= rhs.v
    }
}
impl ops::RemAssign for Int{
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        self.v %= rhs.v
    }
}
impl Num for Int {
    type FromStrRadixErr = <i32 as Num>::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let v = <i32 as Num>::from_str_radix(str, radix)?;
        Ok(Int::new(v))
    }
}
impl num_traits::Zero for Int {
    fn zero() -> Self {
        Int::new(0)
    }

    fn is_zero(&self) -> bool {
        self.v == 0
    }
}
impl num_traits::One for Int {
    fn one() -> Self {
        Int::new(1)
    }
}
