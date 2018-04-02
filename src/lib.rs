extern crate failure;
extern crate yaml_rust as _yaml_rust;

use std::{fs, io::Read, path::Path};

use failure::{Error, ResultExt};

pub fn from_file(file_name: &Path) -> Result<String, Error> {
    let mut file = fs::File::open(&file_name)
        .context({ format!("failed to open file: `{}`", &file_name.display()) })?;

    let mut content = String::new();
    file.read_to_string(&mut content)
        .context({ format!("failed to read contents of: `{}`", file_name.display()) })?;

    Ok(content)
}

// Re-export of yaml_rust
pub mod yaml_rust {
    pub use _yaml_rust::*;
}

pub trait StructConfig {
    fn from_yaml() -> Self;
}

#[cfg(test)]
mod tests {}
