use std::{cmp, ops};

use crate::{cast::CastIt, int::Int};

use super::Float;


impl ops::Add<Int> for Float {
    type Output = Float;

    #[inline]
    fn add(self, rhs: Int) -> Self::Output {
        Float::new(self.v + rhs.f32())
    }
}
impl ops::Sub<Int> for Float {
    type Output = Float;

    #[inline]
    fn sub(self, rhs: Int) -> Self::Output {
        Float::new(self.v - rhs.v.f32())
    }
}
impl ops::Mul<Int> for Float {
    type Output = Float;

    #[inline]
    fn mul(self, rhs: Int) -> Self::Output {
        Float::new(self.v * rhs.v.f32())
    }
}
impl ops::Div<Int> for Float {
    type Output = Float;

    #[inline]
    fn div(self, rhs: Int) -> Self::Output {
        Float::new(self.v / rhs.v.f32())
    }
}
impl ops::Rem<Int> for Float {
    type Output = Float;

    #[inline]
    fn rem(self, rhs: Int) -> Self::Output {
        Float::new(self.v % rhs.v.f32())
    }
}

impl ops::Add<Float> for Int {
    type Output = Float;

    #[inline]
    fn add(self, rhs: Float) -> Self::Output {
        Float::new(self.v.f32() + rhs.v)
    }
}
impl ops::Sub<Float> for Int {
    type Output = Float;

    #[inline]
    fn sub(self, rhs: Float) -> Self::Output {
        Float::new(self.v.f32() - rhs.v)
    }
}
impl ops::Mul<Float> for Int {
    type Output = Float;

    #[inline]
    fn mul(self, rhs: Float) -> Self::Output {
        Float::new(self.v.f32() * rhs.v)
    }
}
impl ops::Div<Float> for Int {
    type Output = Float;

    #[inline]
    fn div(self, rhs: Float) -> Self::Output {
        Float::new(self.v.f32() / rhs.v)
    }
}
impl ops::Rem<Float> for Int {
    type Output = Float;

    #[inline]
    fn rem(self, rhs: Float) -> Self::Output {
        Float::new(self.v.f32() % rhs.v)
    }
}
impl ops::AddAssign<Int> for Float {
    #[inline]
    fn add_assign(&mut self, rhs: Int) {
        self.v += rhs.v.f32()
    }
}
impl ops::SubAssign<Int> for Float {
    #[inline]
    fn sub_assign(&mut self, rhs: Int) {
        self.v -= rhs.v.f32()
    }
}
impl ops::MulAssign<Int> for Float {
    #[inline]
    fn mul_assign(&mut self, rhs: Int) {
        self.v *= rhs.v.f32()
    }
}
impl ops::DivAssign<Int> for Float {
    #[inline]
    fn div_assign(&mut self, rhs: Int) {
        self.v /= rhs.v.f32()
    }
}
impl ops::RemAssign<Int> for Float {
    #[inline]
    fn rem_assign(&mut self, rhs: Int) {
        self.v %= rhs.v.f32()
    }
}
impl PartialOrd<Int> for Float {
    #[inline]
    fn partial_cmp(&self, other: &Int) -> Option<cmp::Ordering> {
        self.v.partial_cmp(&(other.v.f32()))
    }
}
impl PartialEq<Int> for Float {
    #[inline]
    fn eq(&self, other: &Int) -> bool {
        self.v.eq(&(other.v.f32()))
    }
}
impl PartialOrd<Float> for Int {
    #[inline]
    fn partial_cmp(&self, other: &Float) -> Option<cmp::Ordering> {
        (self.v.f32()).partial_cmp(&other.v)
    }
}
impl PartialEq<Float> for Int {
    #[inline]
    fn eq(&self, other: &Float) -> bool {
        (self.v.f32()).eq(&other.v)
    }
}
