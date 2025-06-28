mod int_macros;
mod index;
mod selfops;
mod f32ops;
use int_macros::iops;
use num_traits::Num;
use std::fmt;
use std::iter::Step;
use std::ops;
use std::cmp;

use crate::cast::CastIt;

use self::int_macros::casti;
use self::int_macros::fromil;
use self::int_macros::fromir;

//Debug and Display is implemented later below.
#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Int {
    pub v: i32,
}
impl Int {
    #[must_use]
    #[inline]
    pub const fn new(v: i32) -> Self { Self { v } }
    #[must_use]
    #[inline]
    pub fn max(self, rhs: impl Into<Self>) -> Self {
        Self::new(self.v.max(rhs.into().v))
    }
    #[must_use]
    #[inline]
    pub fn min(self, rhs: impl Into<Self>) -> Self {
        Self::new(self.v.min(rhs.into().v))
    }
    #[must_use]
    #[inline]
    pub fn div_ceil(self, rhs: impl Into<Self>) -> Self 
    {
        let rhs = rhs.into();
        Int::new((self.v + rhs.v - 1i32)/rhs.v)
    }
    #[must_use]
    #[inline]
    pub fn u8(self) -> u8 {
        debug_assert!(self >= u8::MIN);
        debug_assert!(self <= u8::MAX);
        #[allow(clippy::cast_sign_loss)]
        #[allow(clippy::cast_possible_truncation)]
        return self.v as u8;
    }

}
impl fmt::Display for Int {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <i32 as fmt::Display>::fmt(&self.v, f)
    }
}
impl fmt::Debug for Int {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <i32 as fmt::Debug>::fmt(&self.v, f)
    }
}
pub trait CastInt: Sized {
    fn i(self) -> Int;
}
pub trait URange {
    fn urange(&self) -> ops::Range<usize>;
}
impl URange for ops::Range<Int> {
    fn urange(&self) -> ops::Range<usize> {
        let start = self.start;
        let end = self.end;
        return start.u()..end.u();
    }
}
impl Step for Int {
    #[inline]
    fn steps_between(start: &Self, end: &Self) -> (usize, std::option::Option<usize>) {
        <i32 as Step>::steps_between(&start.v, &end.v)
    }

    #[inline]
    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        Some(Int::new(<i32 as Step>::forward_checked(start.v, count)?))
    }

    #[inline]
    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        Some(Int::new(<i32 as Step>::backward_checked(start.v, count)?))
    }
}
iops!(i8);
iops!(i16);
iops!(i32);
iops!(u8);
iops!(u16);
iops!(u32);
iops!(usize);

//iops also adds CastInt so only add missing ones.
//CastInt
casti!(f32);
casti!(u64);

//Int::from(T)
fromil!(i8);
fromil!(i16);
fromil!(i32);
fromil!(u8);
fromil!(u16);

//T::From(Int)
fromir!(i32);
fromir!(i64);
fromir!(f64);


//fdsajglalg 
