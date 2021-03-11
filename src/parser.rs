use syn::parse::{Parse, ParseStream};
use syn::{Ident, Path, Token, Visibility};
use std::fmt::{Debug, Formatter};

pub struct ErrorSpecification {
    pub visibility: Visibility,
    pub name: Ident,
    pub error_types: Vec<Path>,
}

impl Parse for ErrorSpecification {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let visibility: Visibility = input.parse()?;
        let name: Ident = input.parse()?;
        let _ = input.parse::<Token![:]>()?;
        let punctuated = input.parse_terminated::<Path, Token![,]>(Path::parse)?;
        let error_types: Vec<Path> = punctuated.into_iter().collect();

        Ok(ErrorSpecification {
            visibility,
            name,
            error_types,
        })
    }
}

impl Debug for ErrorSpecification {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ErrorSpecification {{ name: {}, error_types: {:?} }}",
            self.name,
            self.error_types
                .iter()
                .map(|path| {
                    path.segments
                        .iter()
                        .map(|segment| &segment.ident)
                        .collect::<Vec<&Ident>>()
                })
                .collect::<Vec<Vec<&Ident>>>()
        )
    }
}

#[test]
fn test_parse_of_error_specification() {
    let parsed: ErrorSpecification =
        syn::parse_str("NewErrorTypeName: crate1::Error1, crate2::some_module::Error2")
            .expect("Parse failed");
    assert_eq!(format!("{:?}", parsed), String::from("ErrorSpecification { name: NewErrorTypeName, error_types: [[Ident(crate1), Ident(Error1)], [Ident(crate2), Ident(some_module), Ident(Error2)]] }"));
}
