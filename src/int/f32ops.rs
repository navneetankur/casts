use std::{cmp, ops};
use crate::{cast::CastIt, float::Float};
use super::Int;

impl ops::Add<f32> for Int {
    type Output = Float;
    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Float::new(self.v.f32() + rhs)
    }
}
impl ops::Sub<f32> for Int {
    type Output = Float;
    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Float::new(self.v.f32() - rhs)
    }
}
impl ops::Mul<f32> for Int {
    type Output = Float;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Float::new(self.v.f32() * rhs)
    }
}
impl ops::Div<f32> for Int {
    type Output = Float;
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Float::new(self.v.f32() / rhs)
    }
}
impl ops::Rem<f32> for Int {
    type Output = Float;
    #[inline]
    fn rem(self, rhs: f32) -> Self::Output {
        Float::new(self.v.f32() % rhs)
    }
}
impl ops::Add<Int> for f32 {
    type Output = Float;

    #[inline]
    fn add(self, rhs: Int) -> Self::Output {
        Float::new(self + rhs.v.f32())
    }
}
impl ops::Sub<Int> for f32 {
    type Output = Float;

    #[inline]
    fn sub(self, rhs: Int) -> Self::Output {
        Float::new(self - rhs.v.f32())
    }
}
impl ops::Mul<Int> for f32 {
    type Output = Float;

    #[inline]
    fn mul(self, rhs: Int) -> Self::Output {
        Float::new(self * rhs.v.f32())
    }
}
impl ops::Div<Int> for f32 {
    type Output = Float;

    #[inline]
    fn div(self, rhs: Int) -> Self::Output {
        Float::new(self / rhs.v.f32())
    }
}
impl ops::Rem<Int> for f32 {
    type Output = Float;

    #[inline]
    fn rem(self, rhs: Int) -> Self::Output {
        Float::new(self % rhs.v.f32())
    }
}
impl ops::AddAssign<Int> for f32 {
    #[inline]
    fn add_assign(&mut self, rhs: Int) {
        *self += rhs.v.f32()
    }
}
impl ops::SubAssign<Int> for f32 {
    #[inline]
    fn sub_assign(&mut self, rhs: Int) {
        *self -= rhs.v.f32()
    }
}
impl ops::MulAssign<Int> for f32 {
    #[inline]
    fn mul_assign(&mut self, rhs: Int) {
        *self *= rhs.v.f32()
    }
}
impl ops::DivAssign<Int> for f32 {
    #[inline]
    fn div_assign(&mut self, rhs: Int) {
        *self /= rhs.v.f32()
    }
}
impl ops::RemAssign<Int> for f32 {
    #[inline]
    fn rem_assign(&mut self, rhs: Int) {
        *self %= rhs.v.f32()
    }
}
impl PartialOrd<f32> for Int {
    #[inline]
    fn partial_cmp(&self, other: &f32) -> Option<cmp::Ordering> {
        (self.v.f32()).partial_cmp(other)
    }
}
impl PartialEq<f32> for Int {
    #[inline]
    fn eq(&self, other: &f32) -> bool {
        (self.v.f32()).eq(other)
    }
}
impl PartialOrd<Int> for f32 {
    #[inline]
    fn partial_cmp(&self, other: &Int) -> Option<cmp::Ordering> {
        self.partial_cmp(&(other.v.f32()))
    }
}
impl PartialEq<Int> for f32 {
    #[inline]
    fn eq(&self, other: &Int) -> bool {
        self.eq(&(other.v.f32()))
    }
}
