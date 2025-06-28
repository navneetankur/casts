use num_traits::Num;
pub mod roundit;
pub mod trunit;
pub trait CastIt: Sized {
    fn u    (self) -> usize;
    fn u8   (self) -> u8   ;
    fn u16  (self) -> u16  ;
    fn u32  (self) -> u32  ;
    fn u64  (self) -> u64  ;
    fn usize(self) -> usize;
    fn i8   (self) -> i8   ;
    fn i16  (self) -> i16  ;
    fn i32  (self) -> i32  ;
    fn i64  (self) -> i64  ;
    fn isize(self) -> isize;
    fn f32  (self) -> f32  ;
    fn f64  (self) -> f64  ;
}

pub trait CastFrom<T: Num + Copy>: Sized {
    fn cast_from(v: T) -> Self;
}
macro_rules! castfrom {
    ($t: ident) => {
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
        impl CastFrom<u64> for $t {
            fn cast_from(v: u64) -> $t {
                u64::$t(v)
            }
        }
        impl CastFrom<usize> for $t {
            fn cast_from(v: usize) -> $t {
                usize::$t(v)
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
        impl CastFrom<i64> for $t {
            fn cast_from(v: i64) -> $t {
                v.$t()
            }
        }
        impl CastFrom<isize> for $t {
            fn cast_from(v: isize) -> $t {
                v.$t()
            }
        }
        impl CastFrom<f32> for $t {
            fn cast_from(v: f32) -> $t {
                f32::$t(v)
            }
        }
        impl CastFrom<f64> for $t {
            fn cast_from(v: f64) -> $t {
                f64::$t(v)
            }
        }
    }
}

castfrom!(u8);
castfrom!(u16);
castfrom!(u32);
castfrom!(u64);
castfrom!(usize);
castfrom!(i8);
castfrom!(i16);
castfrom!(i32);
castfrom!(i64);
castfrom!(f32);

macro_rules! castit {
    ($t: ty) => {
        impl CastIt for $t {
            #[inline]
            fn u(self) -> usize {
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
            fn u64(self) -> u64 {
                let after = self as u64;
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
            fn i64(self) -> i64 {
                let after = self as i64;
                debug_assert_eq!(self, after as Self);
                return after;
            }
            #[inline]
            fn isize(self) -> isize {
                let after = self as isize;
                debug_assert_eq!(self, after as Self);
                return after;
            }
            #[inline]
            fn f32(self) -> f32 {
                let after = self as f32;
                debug_assert_eq!(self, after as Self);
                return after;
            }
            #[inline]
            fn f64(self) -> f64 {
                let after = self as f64;
                debug_assert_eq!(self, after as Self);
                return after;
            }
        }
    };
}
castit!(u8);
castit!(u16);
castit!(u32);
castit!(u64);
castit!(usize);
castit!(u128);
castit!(i8);
castit!(i16);
castit!(i32);
castit!(i64);
castit!(isize);
castit!(f32);
castit!(f64);
