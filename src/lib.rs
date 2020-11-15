//! Overview & Motivation
//! =====================
//! In Rust, the standard way to handle errors is to either use a crate like [Anyhow](https://crates.io/crates/anyhow) to simply bubble up any errors to the caller (and hopefully, eventually, to some code that is in a position to handle it) via the `?` operator, or to create a complex custom error type for each module or crate. This error type usually takes the form of an enum with a variant for each kind of error that can take place from that crate or module.
//! 
//! The problem with these approaches is that the user of your functions likely doesn't know which errors, exactly, can happen from each function. With Anyhow, this problem is even worse since a function returning the Anyhow error type can return *literally any error*. This may be acceptable in some situations, but it clearly isn't ideal. When you create a custom error type, it sometimes isn't a trivial undertaking, which is likely why most crates don't have an error type for each function. Compare this system of poorly-defined ways that a function can error to how it's done in well-written Java pseudo-code:
//! 
//! ```java
//! public Object readAndParseFile(Path file) throws IOException, ParseError{
//!     String contents = Files.read(file); //!throws IOException
//!     Object parsedContents = parseString(contents); //!throws ParseError
//!     return parsedContents;
//! }
//! ```
//! In this example, the set of possible failure modes is clearly documented in the function's signature. Furthermore, it is
//! standard for Javadoc comments to describe under which scenarios each exception is thrown. You can read more about my concerns with Rust's error handling system compared to Java's [here](https://www.reddit.com/r/rust/comments/jdvtu4/javas_error_handling_system_is_better_than_that/).
//! 
//! Polyerror solves this problem by making it so trivial (literally a one-line macro call) to define an ergonomic (? works) and correct error type that it is practical to have a separate error type for each function. This way, it is always obvious to the end user in which way a function can error. See the examples section for more details and to learn how to use this crate. Another advantage of this crate is that it's very simple to use — once you read this document, you know all that there is to know about the crate, as opposed to other error libraries for Rust. Just one macro is exported that provides all that you need for robust, easily understandable (for you and your users), and correct error handling in Rust. 
//! 
//! 
//! Examples
//! =======
//! 
//! What does the macro expand to?
//! ------------------------------
//! Consider this Rust code (based on the basic_use test for this crate):
//! ```rust
//! use std::str::ParseBoolError;
//! use std::num::ParseIntError;
//!
//! # #[macro_use] extern crate polyerror;
//! # fn main(){}
//! 
//! create_error!(pub ParseThenCombineError: ParseBoolError, ParseIntError);
//! pub fn parse_then_combine(a: &str, b: &str) -> Result<String, ParseThenCombineError> {
//!     let parsed_bool: bool = a.parse()?;
//!     let parsed_int: i32 = b.parse()?;
//!     Ok(format!("{} {}", parsed_bool, parsed_int))
//! }
//! ```
//! In this toy function, two errors are possible: ParseBoolError and ParseIntError (both from the standard library). With the traditional model, one would either use something like Anyhow and obscure useful (if not vital) information from the users of your crate, forcing them to delve into your code or hope that it's documented properly (this reminds me of how lifetimes are specified in C and C++), or simply add these two error types to a global error enum. Here, instead, the `ParseThenCombineError` (you're free to choose less verbose names if that's your style) is to be used *only* for the `parse_then_combine` function. Since it's defined with a single line of code before the function, this isn't any significant tedium or productivity drain. 
//! 
//! To give a precise idea of what's going on, the above `create_error!` call expands to this source code:
//! ```ignore
//! #[derive(Debug)]
//! pub enum ParseThenCombineError {
//!     ParseBoolError(ParseBoolError),
//!     ParseIntError(ParseIntError),
//! }
//! impl ::std::fmt::Display for ParseThenCombineError {
//!     fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
//!         write!(f, "{:?}", self)
//!     }
//! }
//! impl ::std::error::Error for ParseThenCombineError {}
//! 
//! impl ::std::convert::From<ParseBoolError> for ParseThenCombineError {
//!     fn from(error: ParseBoolError) -> Self {
//!         Self::ParseBoolError(error)
//!     }
//! }
//! 
//! impl ::std::convert::From<ParseIntError> for ParseThenCombineError {
//!     fn from(error: ParseIntError) -> Self {
//!         Self::ParseIntError(error)
//!     }
//! }
//! ```
//! Notes:
//!  - The expanded Rust code that `create_error!` generates refers to the various error types exactly as you specify them. For example, if, instead, a full path (such as `std::num::ParseIntError`) was provided to the macro call, then the produced Rust code would also use that full path. Furthermore, the name of the variant would be `StdNumParseIntError`. Additionally, if there are underscores in the name, such as `actix_web::Error`, then these are appropriately transformed into an enum variant name: `ActixWebError`.
//!  - You can use any valid access specifier, including none for inherited access (usually private), pub(crate), etc.  
//!  - The created error type is documented in docs.rs as any manually-created error type to make using it easy. 
//!  - Note how `parse_then_combine`'s return type isn't a type alias. This is preferable when using this crate since each error type is used only once.
mod parser;
mod variant;
use crate::parser::ErrorSpecification;
use proc_macro::TokenStream;
use variant::Variant;

#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

/// Creates a new error type. See the [module documentation](./index.html) for more details.
#[proc_macro]
pub fn create_error(input: TokenStream) -> TokenStream {
    let error_specification = parse_macro_input!(input as ErrorSpecification);

    let visibility = &error_specification.visibility;
    let trait_name = &error_specification.name;
    let variants: Vec<Variant> = error_specification.error_types.into_iter().map(|path| Variant::from(path)).collect();

    let mut tokens = quote! {
        #[derive(::std::fmt::Debug)]
        #visibility enum #trait_name{
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