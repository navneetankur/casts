use crate::int::CastInt;

#[test]
fn add(){
    assert_eq!(8i8 + 9.i(),  17.i());
    assert_eq!(8i8.i() + 9, 17.i());
}
#[test]
fn sub(){
    assert_eq!(8.i() - 9i8, (-1).i());
    assert_eq!(8i8 - 9.i(),  (-1).i());
    assert_eq!(9i8 - 8.i(), 1.i());
    assert_eq!(9.i() - 8i8, 1.i());
}
#[test]
fn mul(){
    assert_eq!(8i8 * 9.i(),  72.i());
    assert_eq!(8i8.i() * 9, 72.i());
}
#[test]
fn div(){
    assert_eq!(8i8     / 9i8.i() , 0.i());
    assert_eq!(8i8.i() / 9i8     , 0.i());
    assert_eq!(9i8     / 8i8.i() , 1.i());
    assert_eq!(9i8.i() / 8i8     , 1.i());
    assert_eq!(9i8     / 9i8.i() , 1.i());
    assert_eq!(9i8.i() / 9i8     , 1.i());
}
#[test]
fn rem(){
    assert_eq!(8i8     % 9i8.i() , 8.i());
    assert_eq!(8i8.i() % 9i8     , 8.i());
    assert_eq!(9i8     % 8i8.i() , 1.i());
    assert_eq!(9i8.i() % 8i8     , 1.i());
    assert_eq!(9i8     % 9i8.i() , 0.i());
    assert_eq!(9i8.i() % 9i8     , 0.i());
}
#[test]
fn add_assign(){
    let mut eight = 8i8;
    eight += 9.i();
    assert_eq!(eight, 17);

    let mut eight = 8.i();
    eight += 9i8;
    assert_eq!(eight, 17);
}
#[test]
fn sub_assign(){
    let mut eight = 8i8;
    eight -= 9.i();
    assert_eq!(eight, -1);

    let mut eight = 8.i();
    eight -= 9i8;
    assert_eq!(eight, -1);
}
#[test]
fn mul_assign(){
    let mut eight = 8i8;
    eight *= 9.i();
    assert_eq!(eight, 72);

    let mut eight = 8.i();
    eight *= 9i8;
    assert_eq!(eight, 72);
}
#[test]
fn div_assign(){
    let mut eight = 8i8;
    eight /= 7.i();
    assert_eq!(eight, 1);

    let mut eight = 8.i();
    eight /= 9i8;
    assert_eq!(eight, 0);
}
#[test]
fn rem_assign(){
    let mut eight = 8i8;
    eight %= 7.i();
    assert_eq!(eight, 1);

    let mut eight = 8.i();
    eight %= 9i8;
    assert_eq!(eight, 8);
}
#[test]
fn greator(){
    assert!(8i8 > 7.i());
    assert!(7.i() < 8i8);
    assert!(8i8 == 8.i());
}
