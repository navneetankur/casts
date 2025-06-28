use num_traits::Num;

pub mod roundit;
pub mod trunit;
pub trait CastIt: Sized {
    fn u(self) -> usize;
    fn usize(self) -> usize;
    fn u8(self) ->  u8;
    fn u16(self) -> u16;
    fn u32(self) -> u32;
    fn i8(self) ->  i8;
    fn i16(self) -> i16;
    fn i32(self) -> i32;
    fn f32(self) -> f32;
}

pub trait CastFrom<T: Num + Copy>: Sized {
    fn cast_from(v: T) -> Self;
}
macro_rules! castfrom {
    ($t: ident) => {
        impl CastFrom<usize> for $t {
            fn cast_from(v: usize) -> $t {
                usize::$t(v)
            }
        }
        impl CastFrom<u8> for $t {
            fn cast_from(v: u8) -> $t {
                u8::$t(v)
            }
        }
        impl CastFrom<u16> for $t {
            fn cast_from(v: u16) -> $t {
                u16::$t(v)
            }
        }
        impl CastFrom<u32> for $t {
            fn cast_from(v: u32) -> $t {
                u32::$t(v)
            }
        }
        impl CastFrom<i8> for $t {
            fn cast_from(v: i8) -> $t {
                i8::$t(v)
            }
        }
        impl CastFrom<i16> for $t {
            fn cast_from(v: i16) -> $t {
                i16::$t(v)
            }
        }
        impl CastFrom<i32> for $t {
            fn cast_from(v: i32) -> $t {
                v.$t()
            }
        }
        impl CastFrom<f32> for $t {
            fn cast_from(v: f32) -> $t {
                f32::$t(v)
            }
        }
    }
}

castfrom!(f32);
castfrom!(u8);
castfrom!(u16);
castfrom!(u32);
castfrom!(usize);
castfrom!(i8);
castfrom!(i16);
castfrom!(i32);

macro_rules! castu {
    ($t: ty) => {
        impl CastIt for $t {
            #[inline]
            fn u(self) -> usize {
                let after = self as usize;
                debug_assert_eq!(self, after as Self);
                return after;
            }
            #[inline]
            fn usize(self) -> usize {
                let after = self as usize;
                debug_assert_eq!(self, after as Self);
                return after;
            }
            #[inline]
            fn u8(self) -> u8 {
                let after = self as u8;
                debug_assert_eq!(self, after as Self);
                return after;
            }
            #[inline]
            fn u16(self) -> u16 {
                let after = self as u16;
                debug_assert_eq!(self, after as Self);
                return after;
            }
            #[inline]
            fn u32(self) -> u32 {
                let after = self as u32;
                debug_assert_eq!(self, after as Self);
                return after;
            }
            #[inline]
            fn i8(self) -> i8 {
                let after = self as i8;
                debug_assert_eq!(self, after as Self);
                return after;
            }
            #[inline]
            fn i16(self) -> i16 {
                let after = self as i16;
                debug_assert_eq!(self, after as Self);
                return after;
            }
            #[inline]
            fn i32(self) -> i32 {
                let after = self as i32;
                debug_assert_eq!(self, after as Self);
                return after;
            }
            #[inline]
            fn f32(self) -> f32 {
                self as f32
            }
        }
    };
}
castu!(u8);
castu!(u16);
castu!(u32);
castu!(u64);
castu!(usize);
castu!(u128);
castu!(i8);
castu!(i16);
castu!(i32);
castu!(i64);
castu!(isize);
castu!(f32);
castu!(f64);
