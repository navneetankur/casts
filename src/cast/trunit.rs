use more_asserts as ma;
pub trait TrunIt: Sized {
    fn tu(self) -> usize;
    fn tu8(self) ->  u8;
    fn tu16(self) -> u16;
    fn tu32(self) -> u32;
    fn ti8(self) ->  i8;
    fn ti16(self) -> i16;
    fn ti32(self) -> i32;
}
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
impl TrunIt for f32 {
    fn tu(self) -> usize {
        ma::debug_assert_le!(self, usize::MAX as f32);
        self as usize
    }

    fn tu8(self) ->  u8 {
        ma::debug_assert_le!(self, u8::MAX as f32);
        self as u8
    }

    fn tu16(self) -> u16 {
        ma::debug_assert_le!(self, u16::MAX as f32);
        self as u16
    }

    fn tu32(self) -> u32 {
        ma::debug_assert_le!(self, u32::MAX as f32);
        self as u32
    }

    fn ti8(self) ->  i8 {
        ma::debug_assert_le!(self, i8::MAX as f32);
        ma::debug_assert_ge!(self, i8::MIN as f32);
        self as i8
    }

    fn ti16(self) -> i16 {
        ma::debug_assert_le!(self, i16::MAX as f32);
        ma::debug_assert_ge!(self, i16::MIN as f32);
        self as i16
    }

    fn ti32(self) -> i32 {
        ma::debug_assert_le!(self, i32::MAX as f32);
        ma::debug_assert_ge!(self, i32::MIN as f32);
        self as i32
    }
}
