use cursive::theme::Style;
use cursive::utils::markup::StyledString;

use std::collections::hash_map::HashMap;
use std::fmt;

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

pub enum Value {
    Null(Formatting),
    Bool(Formatting, bool),
    Number(Formatting, Number),
    String(Formatting, String),
    Array(Formatting, Vec<Value>),
    Binary(Formatting, Vec<u8>),
    Object(Formatting, HashMap<String, Value>),
}

pub struct Formatting {
    pub expanded: bool,
    prefix: StyledString,
    content: StyledString,
    postfix: StyledString,
}

impl Default for Formatting {
    fn default() -> Self {
        Self {
            expanded: true,
            prefix: StyledString::default(),
            content: StyledString::default(),
            postfix: StyledString::default(),
        }
    }
}

impl From<serde_json::Value> for Value {
    fn from(serde: serde_json::Value) -> Self {
        match serde {
            serde_json::Value::Null => Value::null(),
            serde_json::Value::Bool(b) => Value::bool(b),
            serde_json::Value::Number(n) => {
                if let Some(number) = n.as_u64() {
                    Value::number_u64(number)
                } else if let Some(number) = n.as_i64() {
                    Value::number_i64(number)
                } else {
                    Value::number_f64(n.as_f64().unwrap())
                }
            },
            serde_json::Value::String(s) => Value::string(s),
            serde_json::Value::Array(mut a) => {
                Value::array(a.drain(..).map(|v| v.into()).collect())
            },
            serde_json::Value::Object(map) => {
                Value::object(
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

pub fn repeat_str(s: &str, n: usize) -> String {
    std::iter::repeat(s).take(n).collect::<String>()
}

impl Value {
    pub fn null() -> Self {
        Value::Null(Formatting::default())
    }

    pub fn bool(val: bool) -> Self {
        Value::Bool(Formatting::default(), val)
    }

    pub fn number_i64(val: i64) -> Self {
        Value::Number(Formatting::default(), Number::Int(val))
    }

    pub fn number_u64(val: u64) -> Self {
        Value::Number(Formatting::default(), Number::Uint(val))
    }

    pub fn number_f64(val: f64) -> Self {
        Value::Number(Formatting::default(), Number::Float(val))
    }

    pub fn string<T: Into<String>>(val: T) -> Self {
        Value::String(Formatting::default(), val.into())
    }

    pub fn array(val: Vec<Value>) -> Self {
        Value::Array(Formatting::default(), val)
    }

    pub fn object(val: HashMap<String, Value>) -> Self {
        Value::Object(Formatting::default(), val)
    }

    pub fn binary(val: Vec<u8>) -> Self {
        Value::Binary(Formatting::default(), val)
    }

    pub fn indent(&self, theme: &Theme) -> StyledString {
        self.indent_inner(theme, 0)
    }

    fn indent_inner(&self, theme: &Theme, level: usize) -> StyledString {
        let indent = StyledString::plain(repeat_str(" ", level * 2));
        match self {
            Value::Null(f) |
            Value::Bool(f, _) |
            Value::Number(f, _) |
            Value::String(f, _) |
            Value::Binary(f, _) => {
                let mut styled = indent;
                styled.append(f.prefix.clone());

                if f.expanded {
                    styled.append(f.content.clone());
                } else {
                    styled.append_styled("…", theme.abbreviation);
                }
                styled.append(f.postfix.clone());

                styled
            },
            Value::Array(f, a) => {
                let mut styled = indent.clone();
                styled.append(f.prefix.clone());

                if f.expanded {
                    styled.append_plain("\n");
                    for value in a.iter() {
                        styled.append(value.indent_inner(theme, level + 1));
                        styled.append_plain("\n");
                    }
                    styled.append(indent);
                } else {
                    styled.append_styled(" … ", theme.abbreviation);
                }
                styled.append(f.postfix.clone());

                styled
            },
            Value::Object(f, map) => {
                let mut styled = indent.clone();
                styled.append(f.prefix.clone());

                if f.expanded {
                    styled.append_plain("\n");
                    for value in map.values() {
                        styled.append(value.indent_inner(theme, level + 1));
                        styled.append_plain("\n");
                    }
                    styled.append(indent);
                } else {
                    styled.append_styled(" … ", theme.abbreviation);
                }
                styled.append(f.postfix.clone());

                styled
            },
        }
    }

    pub fn style(&mut self, theme: &Theme) {
        self.style_inner(theme, StyledString::default(), StyledString::default());
    }

    fn style_inner(&mut self, theme: &Theme, prefix: StyledString, postfix: StyledString) {
        match self {
            Value::Null(f) => {
                f.prefix = prefix;
                f.content = StyledString::styled("null", theme.null);
                f.postfix = postfix;
            },
            Value::Bool(f, b) => {
                f.prefix = prefix;
                f.content = StyledString::styled(format!("{}", b), theme.bool);
                f.postfix = postfix;
            },
            Value::Number(f, n) => {
                f.prefix = prefix;
                f.content = StyledString::styled(format!("{}", n), theme.number);
                f.postfix = postfix;
            },
            Value::String(f, s) => {
                f.prefix = prefix;
                f.content = StyledString::styled(format!("\"{}\"", s), theme.string);
                f.postfix = postfix;
            },
            Value::Array(f, a) => {
                f.prefix = prefix;
                f.prefix.append_styled("[", theme.brace);

                let len = a.len();
                for (idx, value) in a.iter_mut().enumerate() {
                    if idx < len - 1 {
                        value.style_inner(
                            theme,
                            StyledString::default(),
                            StyledString::styled(",", theme.separator),
                        );
                    } else {
                        value.style_inner(
                            theme,
                            StyledString::default(),
                            StyledString::default(),
                        );
                    }
                }

                f.postfix = StyledString::styled("]", theme.brace);
                f.postfix.append(postfix);
            },
            Value::Binary(f, b) => {
                f.prefix = prefix;
                f.prefix.append_styled("<", theme.brace);

                let len = b.len();
                for (idx, byte) in b.iter().enumerate() {
                    f.content.append_plain(format!("{:X}", byte));

                    if idx < len - 1 {
                        f.content.append_styled(" ", theme.separator);
                    }
                }

                f.postfix = StyledString::styled(">", theme.brace);
                f.postfix.append(postfix);
            },
            Value::Object(f, map) => {
                f.prefix = prefix;
                f.prefix.append_styled("{", theme.brace);

                let len = map.len();
                for (idx, (key, value)) in map.iter_mut().enumerate() {
                    let mut prefix = StyledString::styled(key, theme.name);
                    prefix.append_styled(": ", theme.separator);

                    if idx < len - 1 {
                        value.style_inner(
                            theme,
                            prefix,
                            StyledString::styled(",", theme.separator),
                        );
                    } else {
                        value.style_inner(
                            theme,
                            prefix,
                            StyledString::default(),
                        );
                    }
                }

                f.postfix = StyledString::styled("}", theme.brace);
                f.postfix.append(postfix);
            },
        }
    }
}
