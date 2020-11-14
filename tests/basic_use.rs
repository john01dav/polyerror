#[macro_use]
extern crate polyerror;

create_error!(ErrorName: crate1::Error, crate2::some_mod::Error2);

#[test]
fn test_4() {
    //assert_eq!(answer(), 6);
}
