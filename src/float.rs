mod float_macros;
mod selfops;
mod intops;
mod f32ops;
use float_macros::fops;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops;
use std::cmp;

use crate::int::Int;

use self::float_macros::castf;
use self::float_macros::fromfl;
use self::float_macros::fromfr;

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Float {
    pub v: f32,
}
impl Float {
    #[must_use]
    #[inline]
    pub const fn new(v: f32) -> Self { Self { v } }
    #[must_use]
    pub fn ti(self) -> Int {
        #[allow(clippy::cast_possible_truncation)]
        Int::new(self.v as i32)
    }
    #[must_use]
    #[inline]
    pub fn max(self, rhs: impl CastFloat) -> Self {
        Self::new(self.v.max(rhs.f().v))
    }
    #[must_use]
    #[inline]
    pub fn min(self, rhs: impl CastFloat) -> Self {
        Self::new(self.v.min(rhs.f().v))
    }
    #[must_use]
    #[inline]
    pub fn ceil(self) -> Self {
        self.v.ceil().f()
    }
    #[must_use]
    #[inline]
    pub fn floor(self) -> Self {
        self.v.floor().f()
    }
    #[must_use]
    #[inline]
    pub fn copysign(self, sign: Float) -> Self {
        self.v.copysign(sign.v).f()
    }
    #[must_use]
    #[inline]
    pub fn abs(self) -> Self {
        self.v.abs().f()
    }
    #[must_use]
    #[inline]
    pub fn round(self) -> Self {
        self.v.round().f()
    }
}
impl fmt::Display for Float {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <f32 as fmt::Display>::fmt(&self.v, f)
    }
}
impl fmt::Debug for Float {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <f32 as fmt::Display>::fmt(&self.v, f)
    }
}
pub trait CastFloat: Sized {
    fn f(self) -> Float;
}


fops!(i8);
fops!(i16);
fops!(i32);
fops!(u8);
fops!(u16);
fops!(u32);
fops!(usize);
fops!(f32);

//CastFloat, fops also adds this to only missing ones.
castf!(i64);
castf!(u64);
castf!(f64);
castf!(isize);

//Float::From(T)
fromfl!(i8);
fromfl!(i16);
fromfl!(u8);
fromfl!(u16);
fromfl!(f32);

//T::From(Float)
fromfr!(f32);
