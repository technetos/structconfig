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

fn gen_ctor(name: &Ident, attrs: &[Attribute], fields: &Punctuated<Field, Comma>) -> quote::Tokens {
    let fields = fields.iter().map(|field| {
        let field_attrs = Attrs::from_field(field);
        let method = field_attrs.methods();
        let field_name = field.ident.as_ref().unwrap();

        quote!(#field_name: parsed.#method)
    });
    quote! {{
      #( #fields ),*
    }}
}

fn gen_impl(name: &Ident, fields: &Punctuated<Field, Comma>, attrs: &[Attribute]) -> quote::Tokens {
    let struct_attrs = Attrs::from_struct(attrs, name.to_string());
    let field_block = gen_ctor(name, attrs, fields);

    let file_name = struct_attrs.methods();

    quote! {
      impl ::structconfig::StructConfig for #name {
        fn parse_config() -> Self {
            let parsed = ::structconfig::Parsed::#file_name;

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
