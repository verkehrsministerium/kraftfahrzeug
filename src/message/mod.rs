use std::fmt;
use std::rc::Rc;
use std::cell::Cell;
use std::collections::hash_map::HashMap;
use cursive::theme::Style;
use cursive::utils::markup::StyledString;

pub mod from_json;
pub mod view;
pub mod abbreviate;

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

#[derive(Debug, Clone, Copy)]
pub enum ExpansionState {
    Solid,
    Expanded,
    Collapsed,
}

impl Default for ExpansionState {
    fn default() -> Self {
        Self::Solid
    }
}

pub struct Value<'a> {
    pub expansion_state: Rc<Cell<ExpansionState>>,
    prefix: StyledString,
    content: StyledString,
    postfix: StyledString,
    kind: ValueKind<'a>,
    theme: &'a Theme,
}

pub enum ValueKind<'a> {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value<'a>>),
    Binary(Vec<u8>),
    Object(HashMap<String, Value<'a>>),
}

impl From<ValueKind<'static>> for Value<'static> {
    fn from(kind: ValueKind<'static>) -> Self {
        Self {
            expansion_state: Default::default(),
            prefix: Default::default(),
            content: Default::default(),
            postfix: Default::default(),
            theme: &DEFAULT_THEME,
            kind,
        }
    }
}

impl<'a> From<Number> for Value<'a> {
    fn from(number: Number) -> Self {
        ValueKind::Number(number).into()
    }
}

lazy_static::lazy_static! {
    static ref DEFAULT_THEME: Theme = Default::default();
}

#[derive(Clone, Default)]
pub struct Theme {
    pub null: Style,
    pub bool: Style,
    pub number: Style,
    pub string: Style,
    pub brace: Style,
    pub name: Style,
    pub separator: Style,
    pub abbreviation: Style,
    pub tree_control: Style,
}

impl<'a> Value<'a> {
    pub fn new<V: Into<Value<'a>>>(theme: &'a Theme, value_like: V) -> Rc<Self> {
        let mut value = value_like.into();
        value.style(theme, StyledString::default(), StyledString::default());

        Rc::new(value)
    }

    fn style(&mut self, theme: &'a Theme, prefix: StyledString, postfix: StyledString) {
        self.theme = theme;

        match &mut self.kind {
            ValueKind::Array(ref mut a) => {
                self.expansion_state.set(ExpansionState::Expanded);
                self.prefix = prefix;
                self.prefix.append_styled("[", theme.brace);

                let len = a.len();
                for (idx, value) in a.iter_mut().enumerate() {
                    if idx < len - 1 {
                        value.style(
                            theme,
                            StyledString::default(),
                            StyledString::styled(",", theme.separator),
                        );
                    } else {
                        value.style(
                            theme,
                            StyledString::default(),
                            StyledString::default(),
                        );
                    }
                }

                self.postfix = StyledString::styled("]", theme.brace);
                self.postfix.append(postfix);
            },
            ValueKind::Binary(b) => {
                self.expansion_state.set(ExpansionState::Collapsed);
                self.prefix = prefix;
                self.prefix.append_styled("<", theme.brace);

                let len = b.len();
                for (idx, byte) in b.iter().enumerate() {
                    self.content.append_plain(format!("{:X}", byte));

                    if idx < len - 1 {
                        self.content.append_styled(" ", theme.separator);
                    }
                }

                self.postfix = StyledString::styled(">", theme.brace);
                self.postfix.append(postfix);
            },
            ValueKind::Object(ref mut map) => {
                self.expansion_state.set(ExpansionState::Expanded);
                self.prefix = prefix;
                self.prefix.append_styled("{", theme.brace);

                let len = map.len();
                for (idx, (key, value)) in map.iter_mut().enumerate() {
                    let mut prefix = StyledString::styled(key, theme.name);
                    prefix.append_styled(": ", theme.separator);

                    if idx < len - 1 {
                        value.style(
                            theme,
                            prefix,
                            StyledString::styled(",", theme.separator),
                        );
                    } else {
                        value.style(
                            theme,
                            prefix,
                            StyledString::default(),
                        );
                    }
                }

                self.postfix = StyledString::styled("}", theme.brace);
                self.postfix.append(postfix);
            },
            recursive => {
                self.expansion_state.set(ExpansionState::Solid);
                self.prefix = prefix;
                self.content = match recursive {
                    ValueKind::Null => StyledString::styled("null", theme.null),
                    ValueKind::Bool(b) => StyledString::styled(format!("{}", b), theme.bool),
                    ValueKind::Number(n) => StyledString::styled(format!("{}", n), theme.number),
                    ValueKind::String(s) => StyledString::styled(format!("\"{}\"", s), theme.string),
                    _ => unreachable!(),
                };
                self.postfix = postfix;
            }
        }
    }
}
