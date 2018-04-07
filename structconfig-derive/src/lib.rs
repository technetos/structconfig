extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

mod attrs;

use attrs::Attrs;
use proc_macro::TokenStream;
use syn::{*, punctuated::Punctuated, token::Comma};

// Generate the `StructConfig` impl
#[proc_macro_derive(StructConfig)]
pub fn structconfig(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let gen = impl_structconfig(&input);
    gen.into()
}

fn generate_impl(
    name: &Ident,
    fields: &Punctuated<Field, Comma>,
    attrs: &[Attribute],
) -> quote::Tokens {
    Attrs::debug_handle(attrs);

    quote! {
      impl ::structconfig::StructConfig for #name {
        fn parse_config() -> Self {
          Self
        }
      }
    }
}

fn impl_structconfig(input: &DeriveInput) -> quote::Tokens {
    use syn::Data::Struct;
    use syn::Fields::Named;

    #[cfg_attr(rustfmt, rustfmt_skip)]
    // Extract the fields of the struct defined by the user
    let inner_impl = if let Struct(DataStruct { fields: Named(ref fields), .. }) = input.data {
        generate_impl(&input.ident, &fields.named, &input.attrs)
    } else {
        panic!("Only structs are supported by StructConfig")
    };

    quote!(#inner_impl)
}
