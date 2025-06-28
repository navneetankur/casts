macro_rules! iops {
    ($t: ty) => {
        impl ops::Add<$t> for Int {
            type Output = Int;

            #[inline]
            fn add(self, rhs: $t) -> Self::Output {
                Int::new(self.v + rhs.i32())
            }
        }
        impl ops::Sub<$t> for Int {
            type Output = Int;

            #[inline]
            fn sub(self, rhs: $t) -> Self::Output {
                Int::new(self.v - rhs.i32())
            }
        }
        impl ops::Mul<$t> for Int {
            type Output = Int;

            #[inline]
            fn mul(self, rhs: $t) -> Self::Output {
                Int::new(self.v * rhs.i32())
            }
        }
        impl ops::Div<$t> for Int {
            type Output = Int;

            #[inline]
            fn div(self, rhs: $t) -> Self::Output {
                Int::new(self.v / rhs.i32())
            }
        }
        impl ops::Rem<$t> for Int {
            type Output = Int;

            #[inline]
            fn rem(self, rhs: $t) -> Self::Output {
                Int::new(self.v % rhs.i32())
            }
        }
        impl ops::Add<Int> for $t {
            type Output = Int;

            #[inline]
            fn add(self, rhs: Int) -> Self::Output {
                Int::new(self.i32() + rhs.v)
            }
        }
        impl ops::Sub<Int> for $t {
            type Output = Int;

            #[inline]
            fn sub(self, rhs: Int) -> Self::Output {
                Int::new(self.i32() - rhs.v)
            }
        }
        impl ops::Mul<Int> for $t {
            type Output = Int;

            #[inline]
            fn mul(self, rhs: Int) -> Self::Output {
                Int::new(self.i32() * rhs.v)
            }
        }
        impl ops::Div<Int> for $t {
            type Output = Int;

            #[inline]
            fn div(self, rhs: Int) -> Self::Output {
                Int::new(self.i32() / rhs.v)
            }
        }
        impl ops::Rem<Int> for $t {
            type Output = Int;

            #[inline]
            fn rem(self, rhs: Int) -> Self::Output {
                Int::new(self.i32() % rhs.v)
            }
        }
        impl ops::AddAssign<$t> for Int {
            #[inline]
            fn add_assign(&mut self, rhs: $t) {
                self.v += rhs.i32()
            }
        }
        impl ops::SubAssign<$t> for Int {
            #[inline]
            fn sub_assign(&mut self, rhs: $t) {
                self.v -= rhs.i32()
            }
        }
        impl ops::MulAssign<$t> for Int {
            #[inline]
            fn mul_assign(&mut self, rhs: $t) {
                self.v *= rhs.i32()
            }
        }
        impl ops::DivAssign<$t> for Int {
            #[inline]
            fn div_assign(&mut self, rhs: $t) {
                self.v /= rhs.i32()
            }
        }
        impl ops::RemAssign<$t> for Int {
            #[inline]
            fn rem_assign(&mut self, rhs: $t) {
                self.v %= rhs.i32()
            }
        }
        impl ops::AddAssign<Int> for $t {
            #[inline]
            fn add_assign(&mut self, rhs: Int) {
                let after = rhs.v as $t;
                debug_assert_eq!(after as i32, rhs.v);
                *self += after
            }
        }
        impl ops::SubAssign<Int> for $t {
            #[inline]
            fn sub_assign(&mut self, rhs: Int) {
                let after = rhs.v as $t;
                debug_assert_eq!(after as i32, rhs.v);
                *self -= after
            }
        }
        impl ops::MulAssign<Int> for $t {
            #[inline]
            fn mul_assign(&mut self, rhs: Int) {
                let after = rhs.v as $t;
                debug_assert_eq!(after as i32, rhs.v);
                *self *= after
            }
        }
        impl ops::DivAssign<Int> for $t {
            #[inline]
            fn div_assign(&mut self, rhs: Int) {
                let after = rhs.v as $t;
                debug_assert_eq!(after as i32, rhs.v);
                *self /= after
            }
        }
        impl ops::RemAssign<Int> for $t {
            #[inline]
            fn rem_assign(&mut self, rhs: Int) {
                let after = rhs.v as $t;
                debug_assert_eq!(after as i32, rhs.v);
                *self %= after
            }
        }
        impl PartialOrd<$t> for Int {
            #[inline]
            fn partial_cmp(&self, other: &$t) -> Option<cmp::Ordering> {
                self.v.partial_cmp(&(other.i32()))
            }
        }
        impl PartialEq<$t> for Int {
            #[inline]
            fn eq(&self, other: &$t) -> bool {
                self.v.eq(&(other.i32()))
            }
        }
        impl PartialOrd<Int> for $t {
            #[inline]
            fn partial_cmp(&self, other: &Int) -> Option<cmp::Ordering> {
                let after = other.v as $t;
                debug_assert_eq!(after as i32, other.v);
                self.partial_cmp(&after)
            }
        }
        impl PartialEq<Int> for $t {
            #[inline]
            fn eq(&self, other: &Int) -> bool {
                let after = *self as i32;
                debug_assert_eq!(after as $t, *self);
                other.v.eq(&after)
            }
        }
        impl CastInt for $t {
            #[inline]
            fn i(self) -> Int {
                Int::new(self.i32())
            }
        }
    };
}
macro_rules! fromil {
    ($t: ty) => {
        impl From<$t> for Int {
            #[inline]
            fn from(value: $t) -> Self {
                Int::new(i32::from(value))
            }
        }
    };
}
macro_rules! fromir {
    ($t: ty) => {
        impl From<Int> for $t {
            #[inline]
            fn from(value: Int) -> Self {
                <$t>::from(value.v)
            }
        }
    };
}
macro_rules! casti {
    ($t: ty) => {
        impl CastInt for $t {
            #[inline]
            fn i(self) -> Int {
                Int::new(self.i32())
            }
        }
    };
}


pub(super) use iops;
pub(super) use fromil;
pub(super) use fromir;
pub(super) use casti;
