extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

mod attrs;

use attrs::{from_field, from_struct};
use proc_macro::TokenStream;
use syn::{punctuated::Punctuated, token::Comma, *};

// Generate the `StructConfig` impl
#[proc_macro_derive(StructConfig)]
pub fn structconfig(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let gen = impl_structconfig(&input);
    gen.into()
}

fn gen_ctor(fields: &Punctuated<Field, Comma>) -> quote::Tokens {
    let fields = fields.iter().map(|field| {
        let field_attrs = from_field(field);
        let method = field_attrs.method();
        let field_name = field.ident.as_ref().unwrap();

        quote!(#field_name: parsed.#method)
    });

    quote! {{
      #( #fields ),*
    }}
}

fn gen_impl(name: &Ident, fields: &Punctuated<Field, Comma>, attrs: &[Attribute]) -> quote::Tokens {
    let struct_attrs = from_struct(attrs);
    let field_block = gen_ctor(fields);
    let file_name = struct_attrs.file_name();

    // Generate the implementation of the StructConfig trait
    quote! {
      impl ::structconfig::StructConfig for #name {
        fn parse_config() -> Self {
          let parsed = ::structconfig::YamlParser::parse(#file_name);

          #name #field_block
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
        gen_impl(&input.ident, &fields.named, &input.attrs)
    } else {
        panic!("Only structs are supported by StructConfig")
    };

    quote!(#inner_impl)
}
