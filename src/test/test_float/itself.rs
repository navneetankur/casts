use crate::float::CastFloat;

#[test]
fn add(){
    assert_eq!(8.f() + 9.f(),  17.f());
    assert_eq!(8.f().f() + 9, 17.f());
}
#[test]
fn sub(){
    assert_eq!(8.f() - 9.f(), (-1).f());
    assert_eq!(8.f() - 9.f(),  (-1).f());
    assert_eq!(9.f() - 8.f(), 1.f());
    assert_eq!(9.f() - 8.f(), 1.f());
}
#[test]
fn mul(){
    assert_eq!(8.f() * 9.f(),  72.f());
    assert_eq!(8.f().f() * 9, 72.f());
}
#[test]
fn div(){
    assert_eq!(8.f() / 9.f() , 8f32/9.);
    assert_eq!(8.f() / 9.f() , 8f32/9.);
    assert_eq!(9.f() / 8.f() , 9f32/8.);
    assert_eq!(9.f() / 8.f() , 9f32/8.);
    assert_eq!(9.f() / 9.f() , 9f32/9.);
    assert_eq!(9.f() / 9.f() , 9f32/9.);
}
#[test]
fn rem(){
    assert_eq!(8.f()     % 9.f().f() , 8.f());
    assert_eq!(8.f().f() % 9.f()     , 8.f());
    assert_eq!(9.f()     % 8.f().f() , 1.f());
    assert_eq!(9.f().f() % 8.f()     , 1.f());
    assert_eq!(9.f()     % 9.f().f() , 0.f());
    assert_eq!(9.f().f() % 9.f()     , 0.f());
}
#[test]
fn add_assign(){
    let mut eight = 8.f();
    eight += 9.f();
    assert_eq!(eight, 17.);
    let mut eight = 8.f();
    eight += 9.f();
    assert_eq!(eight, 17.);

}
#[test]
fn sub_assign(){
    let mut eight = 8.f();
    eight -= 9.f();
    assert_eq!(eight, -1.);
    let mut eight = 8.f();
    eight -= 9.f();
    assert_eq!(eight, -1.);
}
#[test]
fn mul_assign(){
    let mut eight = 8.f();
    eight *= 9.f();
    assert_eq!(eight, 72.);
    let mut eight = 8.f();
    eight *= 9.f();
    assert_eq!(eight, 72.);

}
#[test]
fn div_assign(){
    let mut eight = 8.f();
    eight /= 7.f();
    assert_eq!(eight, 8f32/7.);
}
#[test]
fn rem_assign(){
    let mut eight = 8.f();
    eight %= 7.f();
    assert_eq!(eight, 1.);
    let mut eight = 8.f();
    eight %= 7.f();
    assert_eq!(eight, 1.);
}
#[test]
fn greator(){
    assert!(8.f() > 7.f());
    assert!(7.f() < 8.f());
    assert!(8.f() == 8.f());
}
