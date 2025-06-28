macro_rules! fops {
    ($t: ty) => {
        impl ops::Add<$t> for Float {
            type Output = Float;

            #[allow(clippy::float_cmp)]
            #[inline]
            fn add(self, rhs: $t) -> Self::Output {
                let after = rhs as f32;
                debug_assert_eq!(after as $t, rhs);
                Float::new(ops::Add::add(self.v, after))
            }
        }
        impl ops::Sub<$t> for Float {
            type Output = Float;

            #[allow(clippy::float_cmp)]
            #[inline]
            fn sub(self, rhs: $t) -> Self::Output {
                let after = rhs as f32;
                debug_assert_eq!(after as $t, rhs);
                Float::new(ops::Sub::sub(self.v, after))
            }
        }
        impl ops::Mul<$t> for Float {
            type Output = Float;

            #[allow(clippy::float_cmp)]
            #[inline]
            fn mul(self, rhs: $t) -> Self::Output {
                let after = rhs as f32;
                debug_assert_eq!(after as $t, rhs);
                Float::new(ops::Mul::mul(self.v, after))
            }
        }
        impl ops::Div<$t> for Float {
            type Output = Float;

            #[allow(clippy::float_cmp)]
            #[inline]
            fn div(self, rhs: $t) -> Self::Output {
                let after = rhs as f32;
                debug_assert_eq!(after as $t, rhs);
                Float::new(ops::Div::div(self.v, after))
            }
        }
        impl ops::Rem<$t> for Float {
            type Output = Float;

            #[allow(clippy::float_cmp)]
            #[inline]
            fn rem(self, rhs: $t) -> Self::Output {
                let after = rhs as f32;
                debug_assert_eq!(after as $t, rhs);
                Float::new(ops::Rem::rem(self.v, after))
            }
        }
        impl ops::Add<Float> for $t {
            type Output = Float;

            #[allow(clippy::float_cmp)]
            #[inline]
            fn add(self, rhs: Float) -> Self::Output {
                let after = self as f32;
                debug_assert_eq!(after as $t, self);
                Float::new(ops::Add::add(after, rhs.v))
            }
        }
        impl ops::Sub<Float> for $t {
            type Output = Float;

            #[allow(clippy::float_cmp)]
            #[inline]
            fn sub(self, rhs: Float) -> Self::Output {
                let after = self as f32;
                debug_assert_eq!(after as $t, self);
                Float::new(ops::Sub::sub(after, rhs.v))
            }
        }
        impl ops::Mul<Float> for $t {
            type Output = Float;

            #[allow(clippy::float_cmp)]
            #[inline]
            fn mul(self, rhs: Float) -> Self::Output {
                let after = self as f32;
                debug_assert_eq!(after as $t, self);
                Float::new(ops::Mul::mul(after, rhs.v))
            }
        }
        impl ops::Div<Float> for $t {
            type Output = Float;

            #[allow(clippy::float_cmp)]
            #[inline]
            fn div(self, rhs: Float) -> Self::Output {
                let after = self as f32;
                debug_assert_eq!(after as $t, self);
                Float::new(ops::Div::div(after, rhs.v))
            }
        }
        impl ops::Rem<Float> for $t {
            type Output = Float;

            #[allow(clippy::float_cmp)]
            #[inline]
            fn rem(self, rhs: Float) -> Self::Output {
                let after = self as f32;
                debug_assert_eq!(after as $t, self);
                Float::new(ops::Rem::rem(after, rhs.v))
            }
        }
        impl ops::AddAssign<$t> for Float {
            #[allow(clippy::float_cmp)]
            #[inline]
            fn add_assign(&mut self, rhs: $t) {
                let after = rhs as f32;
                debug_assert_eq!(after as $t, rhs);
                ops::AddAssign::add_assign(&mut self.v, after)
            }
        }
        impl ops::SubAssign<$t> for Float {
            #[allow(clippy::float_cmp)]
            #[inline]
            fn sub_assign(&mut self, rhs: $t) {
                let after = rhs as f32;
                debug_assert_eq!(after as $t, rhs);
                ops::SubAssign::sub_assign(&mut self.v, after)
            }
        }
        impl ops::MulAssign<$t> for Float {
            #[allow(clippy::float_cmp)]
            #[inline]
            fn mul_assign(&mut self, rhs: $t) {
                let after = rhs as f32;
                debug_assert_eq!(after as $t, rhs);
                ops::MulAssign::mul_assign(&mut self.v, after)
            }
        }
        impl ops::DivAssign<$t> for Float {
            #[allow(clippy::float_cmp)]
            #[inline]
            fn div_assign(&mut self, rhs: $t) {
                let after = rhs as f32;
                debug_assert_eq!(after as $t, rhs);
                ops::DivAssign::div_assign(&mut self.v, after)
            }
        }
        impl ops::RemAssign<$t> for Float {
            #[allow(clippy::float_cmp)]
            #[inline]
            fn rem_assign(&mut self, rhs: $t) {
                let after = rhs as f32;
                debug_assert_eq!(after as $t, rhs);
                ops::RemAssign::rem_assign(&mut self.v, after)
            }
        }
        impl PartialOrd<$t> for Float {
            #[allow(clippy::float_cmp)]
            #[inline]
            fn partial_cmp(&self, other: &$t) -> Option<cmp::Ordering> {
                let after = *other as f32;
                debug_assert_eq!(after as $t, *other);
                self.v.partial_cmp(&(after))
            }
        }
        impl PartialEq<$t> for Float {
            #[allow(clippy::float_cmp)]
            #[inline]
            fn eq(&self, other: &$t) -> bool {
                let after = *other as f32;
                debug_assert_eq!(after as $t, *other);
                self.v.eq(&(after))
            }
        }
        impl PartialOrd<Float> for $t {
            #[allow(clippy::float_cmp)]
            #[inline]
            fn partial_cmp(&self, other: &Float) -> Option<cmp::Ordering> {
                let after = other.v as $t;
                debug_assert_eq!(after as f32, other.v);
                self.partial_cmp(&after)
            }
        }
        impl PartialEq<Float> for $t {
            #[inline]
            fn eq(&self, other: &Float) -> bool {
                let after = *self as f32;
                debug_assert_eq!(after as $t, *self);
                other.v.eq(&after)
            }
        }
        impl CastFloat for $t {
            #[allow(clippy::float_cmp)]
            #[inline]
            fn f(self) -> Float {
                let after = self as f32;
                debug_assert_eq!(after as $t, self);
                Float::new(after)
            }
        }
    };
}
macro_rules! fromfl {
    ($t: ty) => {
        impl From<$t> for Float {
            #[inline]
            fn from(value: $t) -> Self {
                Float::new(value as f32)
            }
        }
    };
}
macro_rules! fromfr {
    ($t: ty) => {
        impl From<Float> for $t {
            #[inline]
            fn from(value: Float) -> Self {
                value.v as $t
            }
        }
    };
}
macro_rules! castf {
    ($t: ty) => {
        impl CastFloat for $t {
            #[inline]
            fn f(self) -> Float {
                Float::new(self as f32)
            }
        }
    };
}

pub(super) use fops;
pub(super) use fromfr;
pub(super) use fromfl;
pub(super) use castf;
