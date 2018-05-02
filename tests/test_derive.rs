#![feature(custom_attribute)]

#[macro_use]
extern crate structconfig;

use std::collections::HashMap;
use structconfig::StructConfig;

#[derive(StructConfig, Debug)]
#[structconfig(filename = "test.yml", filetype = "yaml")]
struct Tester {
    #[structconfig(key = "cc")]
    pub compiler: String,

    #[structconfig(key = "ld")]
    pub linker: String,

    #[structconfig(key = "build", read(as_hash))]
    pub build: HashMap<String, String>,

    #[structconfig(key = "buildNo", read(as_i64))]
    pub build_number: i64,

    #[structconfig(key = "version", read(as_f64))]
    pub version: f64,

    #[structconfig(key = "debug", read(as_bool))]
    pub debug: bool,

    #[structconfig(key = "authors", read(as_vec))]
    pub authors: Vec<String>,
}

#[test]
fn tester_derive() {
    let test = Tester::open();
    assert_eq!(test.authors, &["technetos"]);
}
