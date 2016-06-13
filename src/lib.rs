extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

use serde_json::Value as JsonValue;
use serde_json::Error as JsonError;
use serde_yaml::Value as YamlValue;
use serde_yaml::Error as YamlError;
use std::num::ParseFloatError;
use std::collections::BTreeMap;
use serde::de::Deserialize;

pub enum Error {
    ParseFloat(ParseFloatError),
    InvalidValue,
    Json(JsonError),
    Yaml(YamlError)
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<YamlError> for Error {
    fn from(error: YamlError) -> Error {
        Error::Yaml(error)
    }
}

impl From<JsonError> for Error {
    fn from(error: JsonError) -> Error {
        Error::Json(error)
    }
}
impl From<ParseFloatError> for Error {
    fn from(error: ParseFloatError) -> Error {
        Error::ParseFloat(error)
    }
}

pub fn from_str<T>(s: &str) -> Result<T>
    where T: Deserialize {
        if s.starts_with("{") || s.starts_with("[") {
            let value = try!(serde_json::from_str(s));
            Ok(value)
        } else {
            let value = try!(serde_yaml::from_str(s));
            Ok(value)
        }

}

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
        },
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
        },
        &YamlValue::Hash(ref value) => {
            let mut json_value = BTreeMap::new();
            for (k, v) in value {
                match k {
                    &YamlValue::String(ref key) => {
                        let json = try!(to_json(&v));
                        json_value.insert(key.clone(), json);
                    },
                    _ => return Err(Error::InvalidValue)
                }
            }
            Ok(JsonValue::Object(json_value))
        },
        &YamlValue::Alias(_) => Err(Error::InvalidValue), // not supported yet
        &YamlValue::Null => Ok(JsonValue::Null),
        &YamlValue::BadValue => Err(Error::InvalidValue)

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
