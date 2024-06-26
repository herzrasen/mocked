use std::{fs, io};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Body {
    Bytes(Vec<u8>),
    String(String),
    Include(Include),
}

impl Body {
    pub fn empty() -> Self {
        Body::Bytes(Vec::new())
    }
}

impl TryInto<Vec<u8>> for Body {
    type Error = io::Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        match self {
            Body::Bytes(value) => Ok(value),
            Body::String(value) => Ok(value.as_bytes().to_vec()),
            Body::Include(include) => fs::read(include.include),
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
        let res: Result<Vec<u8>, io::Error> = b.try_into();
        assert!(res.is_ok());
        let expected: Vec<u8> = Vec::new();
        assert_eq!(res.unwrap(), expected)
    }

    #[test]
    fn test_include_is_read_correctly() {
        let tmp_file = tempfile::NamedTempFile::new().unwrap();
        tmp_file.as_file().write("test-data".as_bytes()).unwrap();
        let body = Body::Include(Include {
            include: tmp_file.path().to_path_buf(),
        });
        let res: Result<Vec<u8>, io::Error> = body.try_into();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "test-data".as_bytes().to_vec());
    }
}
