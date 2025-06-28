use crate::float::CastFloat;


#[test]
fn add(){
    assert_eq!(8i8 + 9.f(),  17.f());
    assert_eq!(8i8.f() + 9, 17.f());
}
#[test]
fn sub(){
    assert_eq!(8.f() - 9i8, (-1).f());
    assert_eq!(8i8 - 9.f(),  (-1).f());
    assert_eq!(9i8 - 8.f(), 1.f());
    assert_eq!(9.f() - 8i8, 1.f());
}
#[test]
fn mul(){
    assert_eq!(8i8 * 9.f(),  72.f());
    assert_eq!(8i8.f() * 9, 72.f());
}
#[test]
fn div(){
    assert_eq!(8i8     / 9i8.f() , 8f32/9.);
    assert_eq!(8i8.f() / 9i8     , 8f32/9.);
}
#[test]
fn rem(){
    assert_eq!(8i8     % 9i8.f() , 8.f());
    assert_eq!(8i8.f() % 9i8     , 8.f());
    assert_eq!(9i8     % 8i8.f() , 1.f());
    assert_eq!(9i8.f() % 8i8     , 1.f());
    assert_eq!(9i8     % 9i8.f() , 0.f());
    assert_eq!(9i8.f() % 9i8     , 0.f());
}
#[test]
fn add_assign(){
    let mut eight = 8.f();
    eight += 9i8;
    assert_eq!(eight, 17);
}
#[test]
fn sub_assign(){
    let mut eight = 8.f();
    eight -= 9i8;
    assert_eq!(eight, -1);
}
#[test]
fn mul_assign(){
    let mut eight = 8.f();
    eight *= 9i8;
    assert_eq!(eight, 72);
}
#[test]
fn div_assign(){
    let mut eight = 8.f();
    eight /= 9i8;
    assert_eq!(eight, 8f32/9.);
}
#[test]
fn rem_assign(){
    let mut eight = 8.f();
    eight %= 9i8;
    assert_eq!(eight, 8);
}
#[test]
fn greator(){
    assert!(8i8 > 7.f());
    assert!(7.f() < 8i8);
    assert!(8i8 == 8.f());
}
