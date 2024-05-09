use std::error::Error;
use std::{fs, io};
use std::path::PathBuf;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Body {
    String(String),
    File(File),
}

impl Body {
    pub fn empty() -> Self {
        Body::String(String::new())
    }
}

impl TryInto<String> for Body {
    type Error = io::Error;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Body::String(value) => Ok(value),
            Body::File(file) => {
                fs::read_to_string(file.file)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct File {
    pub file: PathBuf,
}
