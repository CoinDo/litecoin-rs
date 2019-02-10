extern crate proc_macro;

mod de;
mod ser;

use crate::de::impl_deserializable;
use crate::ser::impl_serializable;
use proc_macro::TokenStream;

#[proc_macro_derive(Serializable)]
pub fn serializable(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_serializable(&ast);
    gen.parse().unwrap()
}

#[proc_macro_derive(Deserializable)]
pub fn deserializable(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_deserializable(&ast);
    gen.parse().unwrap()
}
