use std::{fs, io};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Body {
    String(String),
    Include(Include),
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
            Body::Include(include) => fs::read_to_string(include.include),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Include {
    pub include: PathBuf,
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Write;

    use crate::routing::body::{Body, Include};

    #[test]
    fn test_empty_body_is_formatted_correctly() {
        let b = Body::empty();
        let res: Result<String, io::Error> = b.try_into();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "")
    }

    #[test]
    fn test_include_is_read_correctly() {
        let tmp_file = tempfile::NamedTempFile::new().unwrap();
        tmp_file.as_file().write("test-data".as_bytes()).unwrap();
        let body = Body::Include(Include {
            include: tmp_file.path().to_path_buf(),
        });
        let res: Result<String, io::Error> = body.try_into();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "test-data");
    }
}
