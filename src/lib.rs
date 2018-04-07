extern crate failure;
extern crate yaml_rust;

#[allow(unused_imports)]
#[macro_use]
extern crate structconfig_derive;

pub use structconfig_derive::*;

use std::{fs, io::Read, path::Path};

use failure::{Error, ResultExt};

use yaml_rust::{Yaml, YamlLoader};

fn from_file(file_name: &Path) -> Result<String, Error> {
    let mut file = fs::File::open(&file_name)
        .context({ format!("failed to open file: `{}`", &file_name.display()) })?;

    let mut content = String::new();
    file.read_to_string(&mut content)
        .context({ format!("failed to read contents of: `{}`", file_name.display()) })?;

    Ok(content)
}

pub fn as_yaml(file_name: &Path) -> Vec<Yaml> {
    let result = || -> Result<Vec<Yaml>, Error> {
        let content = from_file(file_name)?;
        let parsed_content = YamlLoader::load_from_str(&content)
            .compat()
            .with_context(|e| format!("parsing error: `{}`", e))?;

        Ok(parsed_content)
    };

    let data = result().unwrap_or_else(|e| {
        println!("{}", e);
        panic!();
    });

    data
}

pub struct Parsed {
    data: Vec<Yaml>,
}

impl Parsed {
    pub fn key(&self, name: &str) -> String {
        self.data[0][name].as_str().unwrap().to_owned()
    }

    pub fn filename(name: &str) -> Parsed {
        Parsed {
            data: as_yaml(Path::new(name)),
        }
    }
}

pub trait StructConfig {
    fn parse_config() -> Self;

    fn open() -> Self
    where
        Self: Sized,
    {
        Self::parse_config()
    }
}
