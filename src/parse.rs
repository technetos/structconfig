use failure::{Error, ResultExt};
use std::{fs, io::Read, path::Path};
use yaml_rust::{Yaml, YamlLoader};

pub fn from_file(file_name: &Path) -> Result<String, Error> {
    let mut file = fs::File::open(&file_name)
        .context({ format!("failed to open file: `{}`", &file_name.display()) })?;

    let mut content = String::new();
    file.read_to_string(&mut content)
        .context({ format!("failed to read contents of: `{}`", file_name.display()) })?;

    Ok(content)
}

pub struct YamlParser {
    data: Vec<Yaml>,
}

impl YamlParser {
    pub fn key(&self, name: &str) -> Yaml {
        self.data[0][name].clone()
    }

    pub fn parse(name: &str) -> YamlParser {
        YamlParser {
            data: as_yaml(Path::new(name)),
        }
    }
}

fn as_yaml(file_name: &Path) -> Vec<Yaml> {
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
