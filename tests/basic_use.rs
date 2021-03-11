use error_test::{parse_then_combine, ParseThenCombineError};
use polyerror::create_error;

mod error_test {
    //you don't need to have a separate module like this, but it is added to the test to test that the `pub` specification works
    use std::num::ParseIntError;
    use std::str::ParseBoolError;

    use polyerror::create_error;

    create_error!(pub ParseThenCombineError: ParseBoolError, ParseIntError);
    pub fn parse_then_combine(a: &str, b: &str) -> Result<String, ParseThenCombineError> {
        let parsed_bool: bool = a.parse()?;
        let parsed_int: i32 = b.parse()?;
        Ok(format!("{} {}", parsed_bool, parsed_int))
    }
}

#[test]
fn test_succeeds() {
    assert_eq!(
        parse_then_combine("true", "52").expect("Result didn't return Ok"),
        String::from("true 52")
    );
}

#[test]
fn test_malformed_bool() {
    let result = parse_then_combine("This string isn't a boolean.", "52");
    match &result {
        Ok(_) => panic!("Result was okay: {:?}", result),
        Err(ParseThenCombineError::ParseBoolError(_)) => {} // Okay
        Err(ParseThenCombineError::ParseIntError(_)) => panic!(
            "Result's error type is incorrect, expected bool: {:?}",
            result
        ),
    }
}

#[test]
fn test_malformed_int() {
    let result = parse_then_combine("false", "THis string isn't an integer.");
    match &result {
        Ok(_) => panic!("Result was okay: {:?}", result),
        Err(ParseThenCombineError::ParseBoolError(_)) => panic!(
            "Result's error type is incorrect, expected integer: {:?}",
            result
        ),
        Err(ParseThenCombineError::ParseIntError(_)) => {} // Expected Okay
    }
}

#[test]
fn test_private_type_compiles() {
    use std::num::ParseIntError;
    use std::str::ParseBoolError;

    use polyerror::create_error;

    create_error!(PrivateError: ParseBoolError, ParseIntError);
    fn parse_then_combine(a: &str, b: &str) -> Result<String, PrivateError> {
        let parsed_bool: bool = a.parse()?;
        let parsed_int: i32 = b.parse()?;
        Ok(format!("{} {}", parsed_bool, parsed_int))
    }

    let result = parse_then_combine("false", "THis string isn't an integer.");
    match &result {
        Ok(_) => panic!("Result was okay: {:?}", result),
        Err(PrivateError::ParseBoolError(_)) => panic!(
            "Result's error type is incorrect, expected integer: {:?}",
            result
        ),
        Err(PrivateError::ParseIntError(_)) => {} // Expected Okay
    }

    let result = parse_then_combine("This string isn't a boolean.", "52");
    match &result {
        Ok(_) => panic!("Result was okay: {:?}", result),
        Err(PrivateError::ParseBoolError(_)) => {} // Okay
        Err(PrivateError::ParseIntError(_)) => panic!(
            "Result's error type is incorrect, expected bool: {:?}",
            result
        ),
    }
}

#[test]
fn test_single_error_type_compiles() {
    #[derive(Debug)]
    struct MyError(i32);
    create_error!(OneVariantError: MyError);
    fn parse_one_variant(a: &str) -> Result<String, OneVariantError> {
        let parsed_bool: bool = a.parse().map_err(|_| MyError(5))?;
        Ok(format!("{}", parsed_bool))
    }

    match parse_one_variant("true") {
        Ok(s) => assert_eq!("true", &s),
        Err(_) => panic!("Should be Ok"),
    }

    match parse_one_variant("sdfsdf") {
        Ok(_) => panic!("Should fail"),
        Err(OneVariantError::MyError(MyError(v))) => assert_eq!(v, 5),
    }
}

#[test]
fn test_pub_crate_type_compiles() {
    mod inner {
        use polyerror::create_error;

        #[derive(Debug)]
        pub(crate) struct MyError1;
        #[derive(Debug)]
        pub(crate) struct MyError2;

        create_error!(pub(crate) PubCrate: MyError1, MyError2);
    }
    // `use` should compile
    #[allow(unused_imports)]
    use inner::PubCrate;
    #[warn(unused_imports)]
    () // #[warn(unused_imports)] lead to compile error without this
}

#[test]
fn test_names_valid() {
    pub(crate) mod aa {
        pub(crate) mod bb {
            pub(crate) mod cc {
                #[derive(Debug)]
                pub(crate) struct ABC;
            }
        }
    }

    create_error!(MyError: std::str::ParseBoolError, aa::bb::cc::ABC);

    // We should change casing if it is not CamelCalse?
    let val = MyError::AaBbCcAbc(aa::bb::cc::ABC {});
    match val {
        MyError::AaBbCcAbc(aa::bb::cc::ABC) => {}
        MyError::StdStrParseBoolError(_) => {}
    }
}
