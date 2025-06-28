pub trait CastFrom<T: Copy>: Sized {
    fn cast_from(v: T) -> Self;
}
macro_rules! cast_from_these {
    ($t: ident <= $($f: ident),*) => {
        $(
        impl CastFrom<$f> for $t {
            #[inline]
            fn cast_from(v: $f) -> $t {
                use crate::CastIt;
                <$f as CastIt>::$t(v)
            }
        }
        )*
    };
}
macro_rules! castfrom_impl {
    ($($t: ident),*) => {
        $(
        cast_from_these!($t <= u8, u16, u32, u64, usize);
        cast_from_these!($t <= i8, i16, i32, i64, isize);
        cast_from_these!($t <= f32, f64);
        )*
    }
}

castfrom_impl!(u8, u16, u32, u64, usize);
castfrom_impl!(i8, i16, i32, i64, isize);
castfrom_impl!(f32, f64);
