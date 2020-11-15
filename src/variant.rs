use syn::Path;
use proc_macro2::{Ident, Span};
use quote::{ToTokens, TokenStreamExt};
use inflector::cases::classcase::to_class_case;

pub struct Variant{
    variant_name: String,
    error_type: Path
}

impl From<Path> for Variant{
    fn from(path: Path) -> Self {
        Variant{
            variant_name: recapitalize_error_path(&path),
            error_type: path
        }
    }
}

impl Variant{
    pub fn build_from_impl(&self, enum_name: &Ident, tokens: &mut proc_macro2::TokenStream){
        let name = Ident::new(&self.variant_name, Span::call_site());
        let error_type = &self.error_type;
        tokens.append_all(quote! {
            impl ::std::convert::From<#error_type> for #enum_name{
                fn from(error: #error_type) -> Self{
                    Self::#name(error)
                }
            }
        });
    }
}

impl ToTokens for Variant{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = Ident::new(&self.variant_name, Span::call_site());
        let error_type = &self.error_type;
        tokens.append_all(quote! {
            #name(#error_type)
        });
    }
}

fn recapitalize_error_path(path: &Path) -> String{
    let mut words = Vec::new();
    for segment in &path.segments{
        let name_string = segment.ident.to_string();
        for word in name_string.split("_"){
            words.push(String::from(word));
        }
    }

    let mut name = String::new();
    for word in words{
        name.push_str(&to_class_case(&word));
    }

    name
}