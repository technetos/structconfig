extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{token::Comma, punctuated::Punctuated, *};

// Generate the `StructConfig` impl
#[proc_macro_derive(StructConfig)]
pub fn structconfig(input: TokenStream) -> TokenStream {
  let input: DeriveInput = syn::parse(input).unwrap();
  let gen = impl_structconfig(&input);
  gen.into()
}

fn impl_structconfig_for_struct(name: &Ident, fields: &Punctuated<Field, Comma>) -> quote::Tokens {
  quote! {
    impl ::structconfig::StructConfig for #name {
    }
  }
}

fn impl_structconfig(input: &DeriveInput) -> quote::Tokens {
  use syn::Data::Struct;
  use syn::Fields::Named;

  // Grab the name of the struct defined by the user
  let struct_name = &input.ident;

  // Extract the fields of the struct defined by the user
  let inner_impl = if let Struct(DataStruct { fields: Named(ref fields), .. }) = input.data {
    impl_structconfig_for_struct(&struct_name, &fields.named)
  } else {
    panic!("Only structs are supported by StructConfig")
  };

  quote!(#inner_impl)
}
