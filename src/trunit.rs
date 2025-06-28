use more_asserts as ma;
pub trait TrunIt: Sized {
    fn tu(self) -> usize;
    fn tu8(self) ->  u8;
    fn tu16(self) -> u16;
    fn tu32(self) -> u32;
    fn tu64(self) -> u64;
    fn tusize(self) -> usize;
    fn ti8(self) ->  i8;
    fn ti16(self) -> i16;
    fn ti32(self) -> i32;
    fn ti64(self) -> i64;
    fn tisize(self) -> isize;
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
impl TrunIt for f32 {
    fn tu(self) -> usize {
        ma::debug_assert_le!(self, usize::MAX as Self);
        ma::debug_assert_le!(usize::MIN as Self, self);
        self as usize
    }
    fn tu8(self) -> u8 {
        ma::debug_assert_le!(self, u8::MAX as Self);
        ma::debug_assert_le!(u8::MIN as Self, self);
        self as u8
    }
    fn tu16(self) -> u16 {
        ma::debug_assert_le!(self, u16::MAX as Self);
        ma::debug_assert_le!(u16::MIN as Self, self);
        self as u16
    }
    fn tu32(self) -> u32 {
        ma::debug_assert_le!(self, u32::MAX as Self);
        ma::debug_assert_le!(u32::MIN as Self, self);
        self as u32
    }
    fn tu64(self) -> u64 {
        ma::debug_assert_le!(self, u64::MAX as Self);
        ma::debug_assert_le!(u64::MIN as Self, self);
        self as u64
    }
    fn tusize(self) -> usize {
        ma::debug_assert_le!(self, usize::MAX as Self);
        ma::debug_assert_le!(usize::MIN as Self, self);
        self as usize
    }
    fn ti8(self) -> i8 {
        ma::debug_assert_le!(self, i8::MAX as Self);
        ma::debug_assert_le!(i8::MIN as Self, self);
        self as i8
    }
    fn ti16(self) -> i16 {
        ma::debug_assert_le!(self, i16::MAX as Self);
        ma::debug_assert_le!(i16::MIN as Self, self);
        self as i16
    }
    fn ti32(self) -> i32 {
        ma::debug_assert_le!(self, i32::MAX as Self);
        ma::debug_assert_le!(i32::MIN as Self, self);
        self as i32
    }
    fn ti64(self) -> i64 {
        ma::debug_assert_le!(self, i64::MAX as Self);
        ma::debug_assert_le!(i64::MIN as Self, self);
        self as i64
    }
    fn tisize(self) -> isize {
        ma::debug_assert_le!(self, isize::MAX as Self);
        ma::debug_assert_le!(isize::MIN as Self, self);
        self as isize
    }
}
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
impl TrunIt for f64 {
    fn tu(self) -> usize {
        ma::debug_assert_le!(self, usize::MAX as Self);
        ma::debug_assert_le!(usize::MIN as Self, self);
        self as usize
    }
    fn tu8(self) -> u8 {
        ma::debug_assert_le!(self, u8::MAX as Self);
        ma::debug_assert_le!(u8::MIN as Self, self);
        self as u8
    }
    fn tu16(self) -> u16 {
        ma::debug_assert_le!(self, u16::MAX as Self);
        ma::debug_assert_le!(u16::MIN as Self, self);
        self as u16
    }
    fn tu32(self) -> u32 {
        ma::debug_assert_le!(self, u32::MAX as Self);
        ma::debug_assert_le!(u32::MIN as Self, self);
        self as u32
    }
    fn tu64(self) -> u64 {
        ma::debug_assert_le!(self, u64::MAX as Self);
        ma::debug_assert_le!(u64::MIN as Self, self);
        self as u64
    }
    fn tusize(self) -> usize {
        ma::debug_assert_le!(self, usize::MAX as Self);
        ma::debug_assert_le!(usize::MIN as Self, self);
        self as usize
    }
    fn ti8(self) -> i8 {
        ma::debug_assert_le!(self, i8::MAX as Self);
        ma::debug_assert_le!(i8::MIN as Self, self);
        self as i8
    }
    fn ti16(self) -> i16 {
        ma::debug_assert_le!(self, i16::MAX as Self);
        ma::debug_assert_le!(i16::MIN as Self, self);
        self as i16
    }
    fn ti32(self) -> i32 {
        ma::debug_assert_le!(self, i32::MAX as Self);
        ma::debug_assert_le!(i32::MIN as Self, self);
        self as i32
    }
    fn ti64(self) -> i64 {
        ma::debug_assert_le!(self, i64::MAX as Self);
        ma::debug_assert_le!(i64::MIN as Self, self);
        self as i64
    }
    fn tisize(self) -> isize {
        ma::debug_assert_le!(self, isize::MAX as Self);
        ma::debug_assert_le!(isize::MIN as Self, self);
        self as isize
    }
}
