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
macro_rules! cast_these {
    ($($t: ident),*) => {
        $(
        #[inline]
        fn $t(self) -> $t {
            let after = self as $t;
            debug_assert_eq!(self, after as Self);
            return after;
        }
        )*
    };
}
macro_rules! impl_castit {
    ($($t: ty),*) => {
        $(
        impl CastIt for $t {
            #[inline]
            fn u(self) -> usize {
                let after = self as usize;
                debug_assert_eq!(self, after as Self);
                return after;
            }
            cast_these!(u8, u16, u32, u64, usize);
            cast_these!(i8, i16, i32, i64, isize);
            cast_these!(f32, f64);
        }
        )*
    };
}
impl_castit!(u8, u16, u32, u64, usize);
impl_castit!(i8, i16, i32, i64, isize);
impl_castit!(f32, f64);
