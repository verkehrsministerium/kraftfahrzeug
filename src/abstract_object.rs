use cursive::theme::Style;
use cursive::utils::markup::StyledString;

use std::collections::hash_map::HashMap;
use std::fmt;

#[derive(Clone)]
pub enum Number {
    Int(i64),
    Uint(u64),
    Float(f64),
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::Int(n) => write!(f, "{}", n),
            Number::Uint(n) => write!(f, "{}", n),
            Number::Float(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Binary(Vec<u8>),
    Object(HashMap<String, Value>),
}

impl From<serde_json::Value> for Value {
    fn from(serde: serde_json::Value) -> Self {
        match serde {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(b) => Value::Bool(b),
            serde_json::Value::Number(n) => {
                Value::Number(
                    if let Some(number) = n.as_u64() {
                        Number::Uint(number)
                    } else if let Some(number) = n.as_i64() {
                        Number::Int(number)
                    } else {
                        Number::Float(n.as_f64().unwrap())
                    }
                )
            },
            serde_json::Value::String(s) => Value::String(s),
            serde_json::Value::Array(mut a) => {
                Value::Array(a.drain(..).map(|v| v.into()).collect())
            },
            serde_json::Value::Object(map) => {
                Value::Object(
                    map.iter()
                        .map(|(key, value)| (key.clone(), value.clone().into()))
                        .collect()
                )
            },
        }
    }
}

#[derive(Clone)]
pub struct Theme {
    pub null: Style,
    pub bool: Style,
    pub number: Style,
    pub string: Style,
    pub brace: Style,
    pub name: Style,
    pub separator: Style,
    pub abbreviation: Style,
}

pub struct ThemedValue {
    value: Value,
    theme: Theme,
}

impl ThemedValue {
    pub fn new(value: Value, theme: Theme) -> Self {
        Self {
            value,
            theme,
        }
    }

    fn apply_theme(&self, other: &Value) -> ThemedValue {
        ThemedValue {
            value: other.clone(),
            theme: self.theme.clone(),
        }
    }

    pub fn style(&self) -> StyledString {
        self.style_indented(1)
    }

    fn style_indented(&self, level: usize) -> StyledString {
        let indent = std::iter::repeat(" ").take(level * 2).collect::<String>();
        let previous_indent = std::iter::repeat(" ").take((level - 1) * 2).collect::<String>();
        match &self.value {
            Value::Null => StyledString::styled("null", self.theme.null),
            Value::Bool(b) => StyledString::styled(format!("{}", b), self.theme.bool),
            Value::Number(n) => StyledString::styled(format!("{}", n), self.theme.number),
            Value::String(s) => StyledString::styled(format!("\"{}\"", s), self.theme.string),
            Value::Array(a) => {
                let mut styled = StyledString::styled("[\n", self.theme.brace);
                styled.append_plain(indent.clone());
                let len = a.len();
                for (idx, value) in a.iter().enumerate() {
                    styled.append(self.apply_theme(value).style_indented(level + 1));

                    if idx < len - 1 {
                        styled.append_styled(",\n", self.theme.separator);
                        styled.append_plain(indent.clone());
                    }
                }
                styled.append_plain(format!("\n{}", previous_indent));
                styled.append_styled("]", self.theme.brace);
                styled
            },
            Value::Binary(b) => {
                let mut styled = StyledString::styled("<\n", self.theme.brace);
                styled.append_plain(indent.clone());
                let len = b.len();
                for (idx, byte) in b.iter().enumerate() {
                    styled.append_plain(format!("{:X}", byte));

                    if idx < len - 1 {
                        styled.append_styled(" ", self.theme.separator);
                    }
                }
                styled.append_plain(format!("\n{}", previous_indent));
                styled.append_styled(">", self.theme.brace);
                styled
            },
            Value::Object(map) => {
                let mut styled = StyledString::styled("{\n", self.theme.brace);
                styled.append_plain(indent.clone());
                let len = map.len();
                for (idx, (key, value)) in map.iter().enumerate() {
                    styled.append_styled(key, self.theme.name);
                    styled.append_styled(": ", self.theme.separator);
                    styled.append(self.apply_theme(value).style_indented(level + 1));

                    if idx < len - 1 {
                        styled.append_styled(",\n", self.theme.separator);
                        styled.append_plain(indent.clone());
                    }
                }
                styled.append_plain(format!("\n{}", previous_indent));
                styled.append_styled("}", self.theme.brace);
                styled
            },
        }
    }
}
