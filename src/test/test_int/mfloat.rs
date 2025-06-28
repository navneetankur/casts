use crate::{float::{CastFloat, Float}, int::CastInt};

#[test]
fn add(){
    assert_eq!(8.f() + 9.i(),  17.i());
    assert_eq!(8.f().i() + 9, 17.i());
}
#[test]
fn sub(){
    assert_eq!(8.i() - 9.f(), (-1).i());
    assert_eq!(8.f() - 9.i(),  (-1).i());
    assert_eq!(9.f() - 8.i(), 1.i());
    assert_eq!(9.i() - 8.f(), 1.i());
}
#[test]
fn mul(){
    assert_eq!(8.f() * 9.i(),  72.i());
    assert_eq!(8.f().i() * 9, 72.i());
}
#[test]
fn div(){
    assert_eq!(8.f()     / 9.f().i() , Float::new(8f32/9f32));
    assert_eq!(8.f().i() / 9.f()     , Float::new(8./9.));
    assert_eq!(9.f()     / 8.f().i() , Float::new(9./8.));
    assert_eq!(9.f().i() / 8.f()     , Float::new(9./8.));
    assert_eq!(9.f()     / 9.f().i() , Float::new(9./9.));
    assert_eq!(9.f().i() / 9.f()     , Float::new(9./9.));
}
#[test]
fn rem(){
    assert_eq!(8.f()     % 9.3.f() , 8.i());
    assert_eq!(8.f().i() % 9.f()     , 8.i());
    assert_eq!(9.f()     % 8.7.f() , 9.f() % 8.7.f());
    assert_eq!(9.f().i() % 8.f()     , 1.i());
    assert_eq!(9.f()     % 9.f().i() , 0.i());
    assert_eq!(9.f().i() % 9.f()     , 0.i());
}
#[test]
fn add_assign(){
    let mut eight = 8.f();
    eight += 9.i();
    assert_eq!(eight, 17);
}
#[test]
fn sub_assign(){
    let mut eight = 8.f();
    eight -= 9.i();
    assert_eq!(eight, -1);
}
#[test]
fn mul_assign(){
    let mut eight = 8.f();
    eight *= 9.i();
    assert_eq!(eight, 72);
}
#[test]
fn div_assign(){
    let mut eight = 8.f();
    eight /= 7.i();
    assert_eq!(eight, 8.f()/7.f());
}
#[test]
fn rem_assign(){
    let mut eight = 8.f();
    eight %= 7.i();
    assert_eq!(eight, 1);
}
#[test]
fn greator(){
    assert!(8.f() > 7.i());
    assert!(7.i() < 8.f());
    assert!(8.f() == 8.i());
}
