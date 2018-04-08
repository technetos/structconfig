use quote::Tokens;
use syn::{Attribute, Field, Lit::Str, Meta, Meta::*, MetaList, MetaNameValue, NestedMeta::*};

fn walk_attrs(attrs: &[Attribute], callback: &mut FnMut(Meta)) {
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

    valid_tokens.into_iter().for_each(|token| callback(token));
}

#[derive(Debug)]
struct Parser(Tokens);

impl Parser {
    pub fn from_string(string: &str) -> Parser {
        let method: ::syn::Ident = string.into();

        match string {
            "as_bool" => Parser(quote!(#method().unwrap())),
            "as_i64" => Parser(quote!(#method().unwrap())),
            "as_str" => Parser(quote!(#method().unwrap().to_owned())),
            "as_hash" => Parser(quote! {
              #method()
              .unwrap()
              .iter()
              .map(|(k, v)| (k.as_str(), v.as_str()))
              .filter(|&(k, v)| k.is_some() && v.is_some())
              .map(|(k, v)| (k.unwrap().to_owned(), v.unwrap().to_owned()))
              .collect::<HashMap<String, String>>()
            }),
            "as_vec" => Parser(quote! {
              #method()
              .unwrap()
              .iter()
              .filter_map(|yaml| yaml.as_str())
              .map(|string| string.to_string())
              .collect::<Vec<String>>()
            }),
            "as_f64" => Parser(quote!(#method().unwrap())),
            _ => Parser(quote!(#method().unwrap().to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct Method {
    name: String,
    arg: Tokens,
}

#[derive(Debug)]
pub struct StructAttr {
    file_name: String,
    file_type: String,
}

impl StructAttr {
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn file_type(&self) -> &str {
        &self.file_type
    }
}

#[derive(Debug)]
pub struct FieldAttr {
    parser: Parser,
    method: Method,
}

impl FieldAttr {
    pub fn method(&self) -> Tokens {
        let Method { ref name, ref arg } = self.method;
        let name: ::syn::Ident = name.as_str().into();
        let parse = &self.parser.0;
        quote!(#name(#arg).#parse)
    }
}

#[derive(Debug)]
struct AttrBuilder {
    name: Option<String>,
    file_name: Option<String>,
    file_type: Option<String>,
    method: Option<Method>,
    parser: Option<Parser>,
}

impl AttrBuilder {
    pub fn new() -> AttrBuilder {
        AttrBuilder {
            name: None,
            file_name: None,
            file_type: None,
            method: None,
            parser: None,
        }
    }

    pub fn name(&mut self, string: String) {
        self.name = Some(string);
    }

    pub fn file_name(&mut self, string: String) {
        self.file_name = Some(string);
    }

    pub fn file_type(&mut self, string: String) {
        self.file_type = Some(string);
    }

    pub fn method(&mut self, method: Method) {
        self.method = Some(method);
    }

    pub fn parser(&mut self, string: String) {
        self.parser = Some(Parser::from_string(&string));
    }

    pub fn build_struct_attrs(self) -> StructAttr {
        StructAttr {
            file_name: self.file_name.unwrap(),
            file_type: self.file_type.unwrap(),
        }
    }

    pub fn build_field_attrs(mut self) -> FieldAttr {
        if let None = self.parser {
            self.parser = Some(Parser::from_string("as_str"));
        }

        FieldAttr {
            method: self.method.unwrap(),
            parser: self.parser.unwrap(),
        }
    }
}

fn struct_attrs(attrs: &[Attribute], builder: &mut AttrBuilder) {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    walk_attrs(attrs, &mut |meta| {
        if let NameValue(MetaNameValue { ident, lit: Str(ref value), ..  }) = meta {
            match ident.as_ref() {
                "filename" => builder.file_name(value.value().to_string()),
                "filetype" => builder.file_type(value.value().to_string()),
                _ => (),
            }
        } else {
            panic!("Unsupported")
        }
    });
}

fn field_attrs(field: &Field, builder: &mut AttrBuilder) {
    builder.name(field.ident.as_ref().unwrap().to_string());

    walk_attrs(&field.attrs, &mut |meta| match meta {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        NameValue(MetaNameValue { ident, lit: Str(ref value), ..  }) => {
            builder.method(Method {
                name: ident.as_ref().to_string(),
                arg: quote!(#value),
            });
        }
        #[cfg_attr(rustfmt, rustfmt_skip)]
        List(MetaList { ident, ref nested, ..  }) if ident == "read" => {
            if nested.len() != 1 {
                panic!("the read method only accepts 1 parameter");
            }

            if let Meta(Word(ref ident)) = nested[0] {
                builder.parser(ident.as_ref().to_string());
            }
        }
        _ => panic!("Unsupported"),
    });
}

pub fn from_struct(attrs: &[Attribute]) -> StructAttr {
    let mut builder = AttrBuilder::new();
    struct_attrs(attrs, &mut builder);
    builder.build_struct_attrs()
}

pub fn from_field(field: &Field) -> FieldAttr {
    let mut builder = AttrBuilder::new();
    field_attrs(field, &mut builder);
    builder.build_field_attrs()
}
