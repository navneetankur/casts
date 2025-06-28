#![allow(clippy::needless_return)]
#![feature(step_trait)]
pub mod cast;
pub mod wrap;
#[cfg(test)]
mod test {
    use crate::cast::CastIt;

    fn test_casts() {
        let three = 3;
        assert_eq!(three.u8(), three as u8);
        assert_eq!(three.u16(), three as u16);
        assert_eq!(three.u32(), three as u32);
        assert_eq!(three.u64(), three as u64);
    }
}
