use std::collections::HashMap;
use std::fmt::{Debug, Display};
use crate::error::Error;
use crate::value::Value;

impl<T> TryInto<Vec<T>> for Value where T: TryFrom<Value>, T::Error: Display {

    type Error = Error;

    fn try_into(self) -> Result<Vec<T>, Self::Error> {
        match self {
            Value::Array(vec) => {
                let mut result: Vec<T> = Vec::new();
                for v in vec {
                    result.push(v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(result)
            }
            _ => Err(Error::new(format!("Cannot convert {} into Array", self.type_hint()))),
        }
    }
}

impl<'a, T> TryInto<Vec<T>> for &'a Value where T: TryFrom<&'a Value>, T::Error: Display {

    type Error = Error;

    fn try_into(self) -> Result<Vec<T>, Self::Error> {
        match self {
            Value::Array(vec) => {
                let mut result: Vec<T> = Vec::new();
                for v in vec {
                    result.push(v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(result)
            }
            _ => Err(Error::new(format!("Cannot convert {} into Array", self.type_hint()))),
        }
    }
}

impl<T> TryInto<Option<Vec<T>>> for Value where T: TryFrom<Value>, T::Error: Display {

    type Error = Error;

    fn try_into(self) -> Result<Option<Vec<T>>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::Array(vec) => {
                let mut result: Vec<T> = Vec::new();
                for v in vec {
                    result.push(v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(Some(result))
            }
            _ => Err(Error::new(format!("Cannot convert {} into Option<Array>", self.type_hint()))),
        }
    }
}

impl<'a, T> TryInto<Option<Vec<T>>> for &'a Value where T: TryFrom<&'a Value>, T::Error: Display {

    type Error = Error;

    fn try_into(self) -> Result<Option<Vec<T>>, Self::Error> {
        match self {
            Value::Null => Ok(None),
            Value::Array(vec) => {
                let mut result: Vec<T> = Vec::new();
                for v in vec {
                    result.push(v.try_into().map_err(|e: T::Error| Error::new(format!("{}", e)))?);
                }
                Ok(Some(result))
            }
            _ => Err(Error::new(format!("Cannot convert {} into Array", self.type_hint()))),
        }
    }
}