use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Integer(i64),
    Numeric(f64),
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        if let Ok(v) = value.parse::<i64>() {
            Self::Integer(v)
        } else if let Ok(v) = value.parse::<f64>() {
            Self::Numeric(v)
        } else {
            Self::String(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::routing::value::Value;

    #[test]
    fn test_from_string_with_integer() {
        let v: Value = String::from("42").into();
        assert_eq!(v, Value::Integer(42));
    }


    #[test]
    fn test_from_string_with_numeric() {
        let v: Value = String::from("10.5").into();
        assert_eq!(v, Value::Numeric(10.5));
    }

    #[test]
    fn test_from_string_with_string() {
        let v: Value = String::from("hello").into();
        assert_eq!(v, Value::String(String::from("hello")));
    }
}
