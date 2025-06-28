use std::{cmp, ops};
use super::Float;

impl ops::AddAssign<Float> for f64 {
    #[inline]
    fn add_assign(&mut self, rhs: Float) {
        *self += rhs.v as f64
    }
}
impl ops::SubAssign<Float> for f64 {
    #[inline]
    fn sub_assign(&mut self, rhs: Float) {
        *self -= rhs.v as f64
    }
}
impl ops::MulAssign<Float> for f64 {
    #[inline]
    fn mul_assign(&mut self, rhs: Float) {
        *self *= rhs.v as f64
    }
}
impl ops::DivAssign<Float> for f64 {
    #[inline]
    fn div_assign(&mut self, rhs: Float) {
        *self /= rhs.v as f64
    }
}
impl ops::RemAssign<Float> for f64 {
    #[inline]
    fn rem_assign(&mut self, rhs: Float) {
        *self %= rhs.v as f64
    }
}

impl ops::Add<f64> for Float {
    type Output = f64;

    #[inline]
    fn add(self, rhs: f64) -> Self::Output {
        self.v as f64 + rhs
    }
}
impl ops::Sub<f64> for Float {
    type Output = f64;

    #[inline]
    fn sub(self, rhs: f64) -> Self::Output {
        self.v as f64 - rhs
    }
}
impl ops::Mul<f64> for Float {
    type Output = f64;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        self.v as f64 * rhs
    }
}
impl ops::Div<f64> for Float {
    type Output = f64;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        self.v as f64 / rhs
    }
}
impl ops::Rem<f64> for Float {
    type Output = f64;

    #[inline]
    fn rem(self, rhs: f64) -> Self::Output {
        self.v as f64 % rhs
    }
}
impl ops::Add<Float> for f64 {
    type Output = f64;

    #[inline]
    fn add(self, rhs: Float) -> Self::Output {
        self + rhs.v as f64
    }
}
impl ops::Sub<Float> for f64 {
    type Output = f64;

    #[inline]
    fn sub(self, rhs: Float) -> Self::Output {
        self - rhs.v as f64
    }
}
impl ops::Mul<Float> for f64 {
    type Output = f64;

    #[inline]
    fn mul(self, rhs: Float) -> Self::Output {
        self * rhs.v as f64
    }
}
impl ops::Div<Float> for f64 {
    type Output = f64;

    #[inline]
    fn div(self, rhs: Float) -> Self::Output {
        self / rhs.v as f64
    }
}
impl ops::Rem<Float> for f64 {
    type Output = f64;

    #[inline]
    fn rem(self, rhs: Float) -> Self::Output {
        self % rhs.v as f64
    }
}
impl PartialOrd<f64> for Float {
    #[inline]
    fn partial_cmp(&self, other: &f64) -> Option<cmp::Ordering> {
        (self.v as f64).partial_cmp(other)
    }
}
impl PartialEq<f64> for Float {
    #[inline]
    fn eq(&self, other: &f64) -> bool {
        (self.v as f64).eq(other)
    }
}
impl PartialOrd<Float> for f64 {
    #[inline]
    fn partial_cmp(&self, other: &Float) -> Option<cmp::Ordering> {
        self.partial_cmp(&(other.v as f64))
    }
}
impl PartialEq<Float> for f64 {
    #[inline]
    fn eq(&self, other: &Float) -> bool {
        self.eq(&(other.v as f64))
    }
}
