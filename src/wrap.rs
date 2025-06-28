pub trait WrappedOption {
    fn some(self) -> Option<Self> where Self: core::marker::Sized;
}
pub trait WrappedResult<E> {
    fn ok(self) -> Result<Self, E> where Self: std::marker::Sized;
}
impl<T> WrappedOption for T {
    fn some(self) -> Option<Self> where Self: core::marker::Sized { Some(self) }
}
impl<T, E> WrappedResult<E> for T {
    fn ok(self) -> Result<Self, E> where Self: std::marker::Sized {
        Ok(self)
    }
}
