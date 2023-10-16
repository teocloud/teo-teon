use crate::error::Error;
use crate::value::Value;

impl TryFrom<Value> for String {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(s) => Ok(s),
            _ => Err(Error::new(format!("Cannot convert {} into String", value.type_hint()))),
        }
    }
}

impl TryFrom<&Value> for String {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(s) => Ok(s.to_owned()),
            _ => Err(Error::new(format!("Cannot convert {} into String", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for &'a String {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(s) => Ok(s),
            _ => Err(Error::new(format!("Cannot convert {} into &String", value.type_hint()))),
        }
    }
}

impl TryFrom<Value> for Option<String> {

    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::String(s) => Ok(Some(s)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<String>", value.type_hint()))),
        }
    }
}

impl TryFrom<&Value> for Option<String> {

    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::String(_) => value.clone().try_into(),
            _ => Err(Error::new(format!("Cannot convert {} into Option<String>", value.type_hint()))),
        }
    }
}

impl<'a> TryFrom<&'a Value> for Option<&'a String> {

    type Error = Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(None),
            Value::String(s) => Ok(Some(s)),
            _ => Err(Error::new(format!("Cannot convert {} into Option<&String>", value.type_hint()))),
        }
    }
}
