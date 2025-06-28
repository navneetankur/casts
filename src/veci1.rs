use std::ops::{Index, IndexMut};

pub struct VecI1<T>(Vec<T>);
impl<T> VecI1<T> {
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }
    #[must_use] #[inline]
    pub fn new() -> Self { Self(Vec::new()) }
    #[must_use] #[inline]
    pub fn with_capacity(capacity: usize) -> Self { Self(Vec::with_capacity(capacity)) }

    #[inline] #[must_use]
    pub fn last_index(&self) -> usize {
        self.0.len()
    }
    #[inline] #[must_use]
    pub fn next_index(&self) -> usize {
        self.0.len() + 1
    }
    /// return index of new pushed data.
    #[inline]
    pub fn push(&mut self, value: T) -> usize {
        self.0.push(value);
        return self.last_index();
    }
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit()
    }
    #[inline] #[must_use]
    pub fn len2(&self) -> usize {
        self.0.len()
    }
    #[inline] #[must_use]
    pub fn is_empty(self) -> bool {
        self.0.is_empty()
    }
    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
    }
    #[inline]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.0.iter_mut()
    }
    #[inline] #[must_use]
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
    #[inline] #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.0.as_mut_slice()
    }

}
impl<T> Default for VecI1<T> {
    fn default() -> Self {
        Self(Vec::default())
    }
}

impl<T> IntoIterator for VecI1<T> {
    type Item = <Vec<T> as IntoIterator>::Item;

    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<'a, T> IntoIterator for &'a VecI1<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a, T> IntoIterator for &'a mut VecI1<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> Index<usize> for VecI1<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index - 1]
    }
}
impl<T> IndexMut<usize> for VecI1<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index - 1]
    }
}
impl<T> Index<u16> for VecI1<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: u16) -> &Self::Output {
        &self.0[index as usize - 1]
    }
}
impl<T> IndexMut<u16> for VecI1<T> {
    #[inline]
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.0[index as usize - 1]
    }
}
impl<T> Index<u32> for VecI1<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: u32) -> &Self::Output {
        &self.0[index as usize - 1]
    }
}
impl<T> IndexMut<u32> for VecI1<T> {
    #[inline]
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        &mut self.0[index as usize - 1]
    }
}
