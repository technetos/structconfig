extern crate failure;
extern crate yaml_rust;

#[allow(unused_imports)]
#[macro_use]
extern crate structconfig_derive;

#[allow(unused_attributes)]
pub use structconfig_derive::*;

mod parse;

pub use parse::YamlParser;

pub trait StructConfig {
    fn parse_config() -> Self;

    fn open() -> Self
    where
        Self: Sized,
    {
        Self::parse_config()
    }
}
