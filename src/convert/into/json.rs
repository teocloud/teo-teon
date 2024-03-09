use serde_json::{Value as JsonValue, Number as JsonNumber, Map as JsonMap, json};
use chrono::SecondsFormat;
use teo_result::Error;
use crate::value::Value;

impl TryFrom<&Value> for JsonValue {

    type Error = Error;

    fn try_from(value: &Value) -> Result<JsonValue, Self::Error> {
        Ok(match value {
            Value::Null => JsonValue::Null,
            Value::ObjectId(val) => JsonValue::String(val.to_hex()),
            Value::Bool(val) => JsonValue::Bool(*val),
            Value::Int(val) => JsonValue::Number(JsonNumber::from(*val)),
            Value::Int64(val) => JsonValue::Number(JsonNumber::from(*val)),
            Value::Float32(val) => JsonValue::Number(JsonNumber::from_f64(*val as f64).unwrap()),
            Value::Float(val) => JsonValue::Number(JsonNumber::from_f64(*val).unwrap()),
            Value::Decimal(val) => json!({"$decimal": val.normalized().to_string() }),
            Value::String(val) => JsonValue::String(val.clone()),
            Value::Date(val) => json!({"$date": val.format("%Y-%m-%d").to_string()}),
            Value::DateTime(val) => json!({"$datetime": val.to_rfc3339_opts(SecondsFormat::Millis, true)}),
            Value::Array(val) => {
                let mut vec = vec![];
                for v in val {
                    vec.push(v.try_into()?);
                }
                JsonValue::Array(vec)
            },
            Value::Dictionary(val) => {
                let mut map = JsonMap::new();
                for (k, v) in val {
                    map.insert(k.clone(), v.try_into()?);
                }
                JsonValue::Object(map)
            }
            Value::EnumVariant(val) => {
                JsonValue::String(val.value.clone())
            }
            _ => {
                Err(Error::new(format!("Cannot convert {} into json", value.type_hint())))?
            }
        })
    }
}

impl TryFrom<Value> for JsonValue {

    type Error = Error;

    fn try_from(value: Value) -> Result<JsonValue, Self::Error> {
        (&value).try_into()
    }
}
