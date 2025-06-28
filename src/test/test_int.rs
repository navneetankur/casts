use crate::int::CastInt;
mod mi32;
mod mi16;
mod mi8;
mod mu32;
mod mu16;
mod mu8;
mod mfloat;
mod mf32;
mod itself;
#[test]
fn index_vec() {
    #[allow(clippy::useless_vec)]
    let v = vec![1,2,3];
    assert_eq!(v[1.i()], 2);
}
#[test]
fn index_arr() {
    let v = [1,2,3];
    assert_eq!(v[1.i()], 2);
}
#[test]
fn index_slice() {
    let v = &[1,2,3];
    assert_eq!(v[1.i()], 2);
}
