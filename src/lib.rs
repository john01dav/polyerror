mod parser;

extern crate proc_macro;
use crate::parser::ErrorSpecification;
use proc_macro::TokenStream;

#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

#[proc_macro]
pub fn create_error(input: TokenStream) -> TokenStream {
    let _error_specification = parse_macro_input!(input as ErrorSpecification);
    let tokens = quote! {
        fn answer() -> u8{
            6
        }
    };

    tokens.into()
}
