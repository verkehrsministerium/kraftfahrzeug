use super::{Number, Value, ValueKind};

impl<'a> From<serde_json::Value> for Value<'a> {
    fn from(serde: serde_json::Value) -> Self {
        match serde {
            serde_json::Value::Null => ValueKind::Null.into(),
            serde_json::Value::Bool(b) => ValueKind::Bool(b).into(),
            serde_json::Value::Number(n) => {
                if let Some(number) = n.as_u64() {
                    Number::Uint(number).into()
                } else if let Some(number) = n.as_i64() {
                    Number::Int(number).into()
                } else {
                    Number::Float(n.as_f64().unwrap()).into()
                }
            }
            serde_json::Value::String(s) => ValueKind::String(s).into(),
            serde_json::Value::Array(mut a) => {
                ValueKind::Array(a.drain(..).map(|v| v.into()).collect()).into()
            }
            serde_json::Value::Object(map) => ValueKind::Object(
                map.iter()
                    .map(|(key, value)| (key.clone(), value.clone().into()))
                    .collect(),
            )
            .into(),
        }
    }
}
