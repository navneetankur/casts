use super::CastIt;
pub trait RoundIt: Sized {
    fn ru(self) -> usize;
    fn ru8(self) ->  u8;
    fn ru16(self) -> u16;
    fn ru32(self) -> u32;
    fn ri8(self) ->  i8;
    fn ri16(self) -> i16;
    fn ri32(self) -> i32;
}
impl RoundIt for f32 {
    fn ru(self) -> usize {
        self.round().u()
    }

    fn ru8(self) ->  u8 {
        self.round().u8()
    }

    fn ru16(self) -> u16 {
        self.round().u16()
    }

    fn ru32(self) -> u32 {
        self.round().u32()
    }

    fn ri8(self) ->  i8 {
        self.round().i8()
    }

    fn ri16(self) -> i16 {
        self.round().i16()
    }

    fn ri32(self) -> i32 {
        self.round().i32()
    }
}
