use quote::Tokens;
use syn::{Attribute, Field, MetaNameValue};

pub trait Attrs {
    fn new(name: String) -> Self;
    fn set_method(&mut self, name: &str, arg: &str);
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
            NameValue(MetaNameValue { ident, lit: Str(ref value), .. }) =>
              self.set_method(ident.as_ref(), &value.value()),
            _ => panic!("Unsupported"),
        });
    }
}

#[derive(Debug)]
pub struct StructAttrs {
    name: String,
    file_name: String,
    file_type: String,
}

#[derive(Debug)]
pub struct FieldAttrs {
    name: String,
    methods: Vec<Method>,
}

#[derive(Debug)]
struct Method {
    name: String,
    args: Tokens,
}

impl Attrs for StructAttrs {
    fn new(name: String) -> StructAttrs {
        StructAttrs {
            name,
            file_type: "".to_owned(),
            file_name: "".to_owned(),
        }
    }

    fn set_method(&mut self, name: &str, arg: &str) {
        match (name, arg) {
            ("filename", arg) => self.file_name = arg.to_owned(),
            ("filetype", arg) => self.file_type = arg.to_owned(),
            _ => (),
        }
    }
}

impl StructAttrs {
    pub fn filename(&self) -> &str {
        &self.file_name
    }

    pub fn filetype(&self) -> &str {
        &self.file_type
    }
}

impl Attrs for FieldAttrs {
    fn new(name: String) -> FieldAttrs {
        FieldAttrs {
            name,
            methods: vec![],
        }
    }

    fn set_method(&mut self, name: &str, arg: &str) {
        self.methods.push(Method {
            name: name.into(),
            args: quote!(#arg),
        });
    }
}

impl FieldAttrs {
    pub fn methods(&self) -> Tokens {
        let methods = self.methods.iter().map(|&Method { ref name, ref args }| {
            let name: ::syn::Ident = name.as_str().into();
            quote!(#name(#args))
        });
        quote!(#(#methods)*)
    }
}

pub fn from_struct(attrs: &[Attribute], name: String) -> StructAttrs {
    let mut res = StructAttrs::new(name);
    res.extract_attrs(attrs);
    res
}

pub fn from_field(field: &Field) -> FieldAttrs {
    let name: String = field.ident.as_ref().unwrap().to_string();
    let mut res = FieldAttrs::new(name);
    res.extract_attrs(&field.attrs);
    res
}
