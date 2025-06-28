use crate::float::CastFloat;


#[test]
fn add(){
    assert_eq!(8i32 + 9.f(),  17.f());
    assert_eq!(8i32.f() + 9, 17.f());
}
#[test]
fn sub(){
    assert_eq!(8.f() - 9i32, (-1).f());
    assert_eq!(8i32 - 9.f(),  (-1).f());
    assert_eq!(9i32 - 8.f(), 1.f());
    assert_eq!(9.f() - 8i32, 1.f());
}
#[test]
fn mul(){
    assert_eq!(8i32 * 9.f(),  72.f());
    assert_eq!(8i32.f() * 9, 72.f());
}
#[test]
fn div(){
    assert_eq!(8i32     / 9i32.f() , 8f32/9.);
    assert_eq!(8i32.f() / 9i32     , 8f32/9.);
    assert_eq!(9i32     / 8i32.f() , 9f32/8.);
    assert_eq!(9i32.f() / 8i32     , 9f32/8.);
    assert_eq!(9i32     / 9i32.f() , 9f32/9.);
    assert_eq!(9i32.f() / 9i32     , 9f32/9.);
}
#[test]
fn rem(){
    assert_eq!(8i32     % 9i32.f() , 8.f());
    assert_eq!(8i32.f() % 9i32     , 8.f());
    assert_eq!(9i32     % 8i32.f() , 1.f());
    assert_eq!(9i32.f() % 8i32     , 1.f());
    assert_eq!(9i32     % 9i32.f() , 0.f());
    assert_eq!(9i32.f() % 9i32     , 0.f());
}
#[test]
fn add_assign(){
    let mut eight = 8.f();
    eight += 9i32;
    assert_eq!(eight, 17);
}
#[test]
fn sub_assign(){
    let mut eight = 8.f();
    eight -= 9i32;
    assert_eq!(eight, -1);
}
#[test]
fn mul_assign(){
    let mut eight = 8.f();
    eight *= 9i32;
    assert_eq!(eight, 72);
}
#[test]
fn div_assign(){
    let mut eight = 8.f();
    eight /= 9i32;
    assert_eq!(eight, 8f32/9.);
}
#[test]
fn rem_assign(){
    let mut eight = 8.f();
    eight %= 9i32;
    assert_eq!(eight, 8);
}
#[test]
fn greator(){
    assert!(8i32 > 7.f());
    assert!(7.f() < 8i32);
    assert!(8i32 == 8.f());
}
