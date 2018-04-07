use quote::Tokens;
use syn::{Attribute, MetaNameValue};

#[derive(Debug)]
pub struct Attrs {
    file_name: String,
    file_type: String,
    directives: Vec<Directive>,
}

#[derive(Debug)]
struct Directive {
    name: String,
    args: Tokens,
}

impl Attrs {
    fn new() -> Attrs {
        Attrs {
            file_name: "".to_owned(),
            file_type: "".to_owned(),
            directives: vec![],
        }
    }

    // Grab the filename and filetype from the attribute on
    // the user defined struct
    fn set_file_metadata(&mut self, name: &str, arg: &str) {
        match (name, arg) {
            ("filename", fname) => self.file_name = fname.into(),
            ("filetype", ftype) => self.file_type = ftype.into(),
            _ => (),
        };
    }

    fn push_attrs(&mut self, attrs: &[Attribute]) {
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
              self.set_file_metadata(ident.as_ref(), &value.value())
            },
            _ => panic!("Unsupported"),
        });
    }

    pub fn debug_handle(attrs: &[Attribute]) {
        let mut res = Self::new();
        res.push_attrs(attrs);
    }

    //  pub fn from_struct(attrs: &[Attribute], name: String) -> Attrs {
    //    let mut res = Self::new(name);
    //    res.push_attrs(attrs);
    //  }
}
