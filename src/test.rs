mod test_int;
mod test_float;
#[must_use]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[test]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}
