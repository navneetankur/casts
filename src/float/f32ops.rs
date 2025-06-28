use std::ops;
use super::Float;

// rest is done by macro
impl ops::AddAssign<Float> for f32 {
    #[inline]
    fn add_assign(&mut self, rhs: Float) {
        *self += rhs.v
    }
}
impl ops::SubAssign<Float> for f32 {
    #[inline]
    fn sub_assign(&mut self, rhs: Float) {
        *self -= rhs.v
    }
}
impl ops::MulAssign<Float> for f32 {
    #[inline]
    fn mul_assign(&mut self, rhs: Float) {
        *self *= rhs.v
    }
}
impl ops::DivAssign<Float> for f32 {
    #[inline]
    fn div_assign(&mut self, rhs: Float) {
        *self /= rhs.v
    }
}
impl ops::RemAssign<Float> for f32 {
    #[inline]
    fn rem_assign(&mut self, rhs: Float) {
        *self %= rhs.v
    }
}
