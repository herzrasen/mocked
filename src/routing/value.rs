use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Numeric(f64),
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        match value.parse::<f64>() {
            Ok(v) => Self::Numeric(v),
            Err(_) => Self::String(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::routing::value::Value;

    #[test]
    fn test_from_string_with_numeric() {
        let v: Value = String::from("10").into();
        assert_eq!(v, Value::Numeric(10.));
    }

    #[test]
    fn test_from_string_with_string() {
        let v: Value = String::from("hello").into();
        assert_eq!(v, Value::String(String::from("hello")));
    }
}
