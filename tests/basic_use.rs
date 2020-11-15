use std::str::ParseBoolError;
use std::num::ParseIntError;

#[macro_use]
extern crate polyerror;

create_error!(ParseThenCombineError: ParseBoolError, ParseIntError);
fn parse_then_combine(a: &str, b: &str) -> Result<String, ParseThenCombineError>{
    let parsed_bool: bool = a.parse()?;
    let parsed_int: i32 = b.parse()?;
    Ok(format!("{} {}", parsed_bool, parsed_int))
}

#[test]
fn test_succeeds() {
    assert_eq!(parse_then_combine("true", "52").expect("Result didn't return Ok"), String::from("true 52"));
}

#[test]
fn test_malformed_bool(){
    let result = parse_then_combine("This string isn't a boolean.", "52");
    match &result{
        Ok(_) => panic!("Result was okay: {:?}", result),
        Err(err) => if let ParseThenCombineError::ParseBoolError(_) = err{
            //okay
        }else{
            panic!("Result's error type is incorrect, expected bool: {:?}", result);
        }
    }
}

#[test]
fn test_malformed_int(){
    let result = parse_then_combine("false", "THis string isn't an integer.");
    match &result{
        Ok(_) => panic!("Result was okay: {:?}", result),
        Err(err) => if let ParseThenCombineError::ParseIntError(_) = err{
            //okay
        }else{
            panic!("Result's error type is incorrect, expected integer: {:?}", result);
        }
    }
}