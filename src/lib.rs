mod parser;
mod variant;
use crate::parser::ErrorSpecification;
use proc_macro::TokenStream;
use variant::Variant;

#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

#[proc_macro]
pub fn create_error(input: TokenStream) -> TokenStream {
    let error_specification = parse_macro_input!(input as ErrorSpecification);

    let trait_name = &error_specification.name;
    let variants: Vec<Variant> = error_specification.error_types.into_iter().map(|path| Variant::from(path)).collect();

    let mut tokens = quote! {
        #[derive(::std::fmt::Debug)]
        pub enum #trait_name{
            #(#variants),*
        }

        impl ::std::fmt::Display for #trait_name{
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result{
                write!(f, "{:?}", self)
            }
        }

        impl ::std::error::Error for #trait_name{}
    };

    for variant in variants{
        variant.build_from_impl(&error_specification.name, &mut tokens);
    }

    tokens.into()
}