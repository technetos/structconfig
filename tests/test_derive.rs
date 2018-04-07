#![feature(custom_attribute)]

#[macro_use]
extern crate structconfig;

use structconfig::StructConfig;

#[derive(StructConfig)]
#[structconfig(filename = "test.yml", filetype = "yaml")]
struct Tester {
    #[structconfig(key = "cc")]
    compiler: String,

    #[structconfig(key = "ld")]
    linker: String,
}

#[test]
fn tester_derive() {

  let test = Tester::open();
}
