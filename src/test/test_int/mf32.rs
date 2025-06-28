use crate::int::CastInt;

#[test]
fn add(){
    assert_eq!(8f32 + 9.i(),  17.i());
    assert_eq!(8f32.i() + 9, 17.i());
}
#[test]
fn sub(){
    assert_eq!(8.i() - 9f32, (-1).i());
    assert_eq!(8f32 - 9.i(),  (-1).i());
    assert_eq!(9f32 - 8.i(), 1.i());
    assert_eq!(9.i() - 8f32, 1.i());
}
#[test]
fn mul(){
    assert_eq!(8f32 * 9.i(),  72.i());
    assert_eq!(8f32.i() * 9, 72.i());
}
#[test]
fn div(){
    assert_eq!(8f32     / 9f32.i() , 8f32/9.);
    assert_eq!(8f32.i() / 9f32     , 8f32/9.);
    assert_eq!(9f32     / 8f32.i() , 9f32/8.);
    assert_eq!(9f32.i() / 8f32     , 9f32/8.);
    assert_eq!(9f32     / 9f32.i() , 9f32/9.);
    assert_eq!(9f32.i() / 9f32     , 9f32/9.);
}
#[test]
fn rem(){
    assert_eq!(8f32     % 9f32.i() , 8.i());
    assert_eq!(8f32.i() % 9f32     , 8.i());
    assert_eq!(9f32     % 8f32.i() , 1.i());
    assert_eq!(9f32.i() % 8f32     , 1.i());
    assert_eq!(9f32     % 9f32.i() , 0.i());
    assert_eq!(9f32.i() % 9f32     , 0.i());
}
#[test]
fn add_assign(){
    let mut eight = 8f32;
    eight += 9.i();
    assert_eq!(eight, 17.);

}
#[test]
fn sub_assign(){
    let mut eight = 8f32;
    eight -= 9.i();
    assert_eq!(eight, -1.);
}
#[test]
fn mul_assign(){
    let mut eight = 8f32;
    eight *= 9.i();
    assert_eq!(eight, 72.);

}
#[test]
fn div_assign(){
    let mut eight = 8f32;
    eight /= 7.i();
    assert_eq!(eight, 8./7.);
}
#[test]
fn rem_assign(){
    let mut eight = 8f32;
    eight %= 7.i();
    assert_eq!(eight, 1.);
}
#[test]
fn greator(){
    assert!(8f32 > 7.i());
    assert!(7.i() < 8f32);
    assert!(8f32 == 8.i());
}
