use std::ops;
use super::Int;
impl<T> ops::Index<Int> for Vec<T> {
    type Output = T;
    #[inline]
    fn index(&self, index: Int) -> &Self::Output {
        self.index(index.v as usize)
    }
}
impl<T> ops::Index<Int> for [T] {
    type Output = T;
    #[inline]
    fn index(&self, index: Int) -> &Self::Output {
        self.index(index.v as usize)
    }
}
