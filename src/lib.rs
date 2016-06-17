//! Jamal provides a bi-directional interface for transformations between json and yaml documents.
extern crate serde_json;
extern crate serde_yaml;

use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use std::error::Error as StdError;
use std::num::ParseFloatError;
use std::collections::BTreeMap;

/// represents potential errors that can happen during the transformation process
#[derive(Debug)]
pub enum Error {
    /// Occurs when an attempt to parse a `serde_yaml::Real` as an `f64` fails
    ParseFloat(ParseFloatError),
    /// Occurs when a transformation is requested for an invalid value
    InvalidValue,
    /// Occurs when an unsupported transformation is requested
    UnsupportedValue(YamlValue)
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ParseFloat(ref e) => e.description(),
            Error::InvalidValue => "invalid value",
            Error::UnsupportedValue(_) => "supported value",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::ParseFloat(ref e) => Some(e),
            _ => None,
        }
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


/// A result type with a fixed type for `jamal::Errors`
pub type Result<T> = std::result::Result<T, Error>;

impl From<ParseFloatError> for Error {
    fn from(error: ParseFloatError) -> Error {
        Error::ParseFloat(error)
    }
}

/// converts a `serde_json::Value` into a `serde_yaml::Value`
pub fn to_yaml(json: &JsonValue) -> Result<YamlValue> {
    match json {
        &JsonValue::Null => Ok(YamlValue::Null),
        &JsonValue::Bool(ref value) => Ok(YamlValue::Boolean(value.clone())),
        &JsonValue::I64(ref value) => Ok(YamlValue::Integer(value.clone())),
        &JsonValue::U64(ref value) => Ok(YamlValue::Integer(value.clone() as i64)),
        &JsonValue::F64(ref value) => Ok(YamlValue::Real(value.to_string())),
        &JsonValue::String(ref value) => Ok(YamlValue::String(value.clone())),
        &JsonValue::Array(ref values) => {
            let mut yaml_values = vec![];
            for value in values {
                yaml_values.push(try!(to_yaml(&value)));
            }
            Ok(YamlValue::Array(yaml_values))
        }
        &JsonValue::Object(ref value) => {
            let mut yaml_value = BTreeMap::new();
            for (k, v) in value {
                let yaml = try!(to_yaml(&v));
                yaml_value.insert(YamlValue::String(k.clone()), yaml);
            }
            Ok(YamlValue::Hash(yaml_value))
        }
    }
}

/// converts a `serde_yaml::Value` into a `serde_json::Value`
pub fn to_json(yaml: &YamlValue) -> Result<JsonValue> {
    match yaml {
        &YamlValue::Real(ref value) => Ok(JsonValue::F64(try!(value.parse::<f64>()))),
        &YamlValue::Integer(ref value) => Ok(JsonValue::I64(value.clone())),
        &YamlValue::String(ref value) => Ok(JsonValue::String(value.clone())),
        &YamlValue::Boolean(ref value) => Ok(JsonValue::Bool(value.clone())),
        &YamlValue::Array(ref values) => {
            let mut json_values = vec![];
            for value in values {
                let json = try!(to_json(&value));
                json_values.push(json);
            }
            Ok(JsonValue::Array(json_values))
        }
        &YamlValue::Hash(ref value) => {
            let mut json_value = BTreeMap::new();
            for (k, v) in value {
                match k {
                    &YamlValue::String(ref key) => {
                        let json = try!(to_json(&v));
                        json_value.insert(key.clone(), json);
                    }
                    unsupported => return Err(Error::UnsupportedValue(unsupported.clone())),
                }
            }
            Ok(JsonValue::Object(json_value))
        }
        alias @ &YamlValue::Alias(_) => Err(Error::UnsupportedValue(alias.clone())),
        &YamlValue::Null => Ok(JsonValue::Null),
        &YamlValue::BadValue => Err(Error::InvalidValue),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use super::serde_json;
    use super::serde_yaml;
    use super::{to_json, to_yaml};

    #[test]
    fn json_to_yaml() {
        let input = serde_json::to_value(&vec!["foo"]);
        let output = serde_yaml::Value::Array(vec![serde_yaml::Value::String("foo".to_owned())]);
        assert_eq!(to_yaml(&input).unwrap(), output);
    }

    #[test]
    fn yaml_to_json() {
        let input = serde_yaml::to_value(&vec!["foo"]);
        let output = serde_json::Value::Array(vec![serde_json::Value::String("foo".to_owned())]);
        assert_eq!(to_json(&input).unwrap(), output);
    }
}
