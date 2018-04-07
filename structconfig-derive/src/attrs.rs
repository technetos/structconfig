use quote::Tokens;
use syn::{Attribute, Field, MetaNameValue};

#[derive(Debug)]
pub struct Attrs {
    name: String,
    methods: Vec<Directive>,
}

#[derive(Debug)]
struct Directive {
    name: String,
    args: Tokens,
}

impl Attrs {
    fn new(name: String) -> Attrs {
        Attrs {
            name,
            methods: vec![],
        }
    }

    fn set_method(&mut self, name: &str, arg: &str) {
        self.methods.push(Directive {
            name: name.into(),
            args: quote!(#arg),
        });
    }

    fn extract_attrs(&mut self, attrs: &[Attribute]) {
        use Lit::*;
        use Meta::*;
        use NestedMeta::*;

        let valid_tokens = attrs
            .iter()
            .filter_map(|attr| {
                let path = &attr.path;
                if quote!(#path) == quote!(structconfig) {
                    attr.interpret_meta()
                } else {
                    panic!("Unsupported syntax: Attribute must start with `structconfig`")
                }
            })
            .flat_map(|token| match token {
                List(list) => list.nested,
                tokens => panic!("Unsupported syntax: {}\nExpected List", quote!(#tokens)),
            })
            .map(|token| match token {
                Meta(meta) => meta,
                ref tokens => panic!("Unsupported syntax: {}\nExpected Meta", quote!(#tokens)),
            });

        valid_tokens.into_iter().for_each(|token| match token {
            #[cfg_attr(rustfmt, rustfmt_skip)]
            NameValue(MetaNameValue { ident, lit: Str(ref value), .. }) => {
              self.set_method(ident.as_ref(), &value.value())
            },
            _ => panic!("Unsupported"),
        });
    }

    pub fn from_struct(attrs: &[Attribute], name: String) -> Attrs {
        let mut res = Self::new(name);
        res.extract_attrs(attrs);
        res
    }

    pub fn from_field(field: &Field) -> Attrs {
        let name: String = field.ident.as_ref().unwrap().to_string();
        let mut res = Self::new(name);
        res.extract_attrs(&field.attrs);
        res
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn methods(&self) -> Tokens {
        let methods = self.methods
            .iter()
            .map(|&Directive { ref name, ref args }| {
                let name: ::syn::Ident = name.as_str().into();
                quote!(#name(#args))
            });
        quote!(#(#methods)*)
    }
}
