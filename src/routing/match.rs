use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum Match {
    String(String),
    Numeric(f64),
}

impl From<String> for Match {
    fn from(value: String) -> Self {
        match value.parse::<f64>() {
            Ok(v) => Match::Numeric(v),
            Err(_) => Match::String(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::routing::r#match::Match;

    #[test]
    fn test_from_string_with_numeric() {
        let m: Match = String::from("10").into();
        assert_eq!(m, Match::Numeric(10.));
    }

    #[test]
    fn test_from_string_with_string() {
        let m: Match = String::from("hello").into();
        assert_eq!(m, Match::String(String::from("hello")));
    }
}
