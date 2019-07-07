use cursive::theme::Style;
use cursive::utils::markup::StyledString;
use cursive::views::{SelectView, OnEventView};
use cursive::event::EventTrigger;
use cursive::view::{View, Selector, Identifiable};
use cursive::Cursive;

use std::collections::hash_map::HashMap;
use std::fmt;
use std::rc::Rc;
use std::cell::Cell;

use crate::highlight_list_item;

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

pub struct Value {
    pub expansion_state: Rc<Cell<ExpansionState>>,
    prefix: StyledString,
    content: StyledString,
    postfix: StyledString,
    kind: ValueKind,
}

pub enum ValueKind {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Binary(Vec<u8>),
    Object(HashMap<String, Value>),
}

impl From<ValueKind> for Value {
    fn from(kind: ValueKind) -> Self {
        Self {
            expansion_state: Default::default(),
            prefix: Default::default(),
            content: Default::default(),
            postfix: Default::default(),
            kind,
        }
    }
}

impl From<Number> for Value {
    fn from(number: Number) -> Self {
        ValueKind::Number(number).into()
    }
}

impl From<serde_json::Value> for Value {
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
            },
            serde_json::Value::String(s) => ValueKind::String(s).into(),
            serde_json::Value::Array(mut a) => {
                ValueKind::Array(a.drain(..).map(|v| v.into()).collect()).into()
            },
            serde_json::Value::Object(map) => {
                ValueKind::Object(
                    map.iter()
                        .map(|(key, value)| (key.clone(), value.clone().into()))
                        .collect()
                ).into()
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
    pub tree_control: Style,
}

pub fn repeat_str(s: &str, n: usize) -> String {
    std::iter::repeat(s).take(n).collect::<String>()
}

impl Value {
    pub fn abbreviate(&self, available: usize, theme: &Theme) -> StyledString {
        self.abbreviate_inner(available, theme).1
    }

    fn abbreviate_inner(&self, available: usize, theme: &Theme) -> (bool, StyledString) {
        match &self.kind {
            ValueKind::Null | ValueKind::Bool(_) | ValueKind::Number(_) |
            ValueKind::String(_) | ValueKind::Binary(_) => {
                let mut styled = self.prefix.clone();
                styled.append(self.content.clone());
                styled.append(self.postfix.clone());

                if available < styled.width() {
                    (true, StyledString::styled("…", theme.abbreviation))
                } else {
                    (false, styled)
                }
            },
            recursive => {
                let iter: Box<dyn std::iter::Iterator<Item = &Value>> = match recursive {
                    ValueKind::Array(a) => Box::new(a.iter()),
                    ValueKind::Object(map) => Box::new(map.values()),
                    _ => unreachable!(),
                };
                let mut styled = self.prefix.clone();

                styled.append_plain(" ");
                let mut width = styled.width() + self.postfix.width() + 2;

                if available < width + 2 {
                    (true, StyledString::styled("…", theme.abbreviation))
                } else {
                    for value in iter {
                        // extra +1 because of the whitespace after this value
                        let (abbr, fmt) = value.abbreviate_inner(
                            available.checked_sub(width + 1).unwrap_or(0),
                            theme,
                        );
                        styled.append(fmt);
                        styled.append_plain(" ");

                        if abbr {
                            break;
                        }

                        width = styled.width() + self.postfix.width();
                    }
                    styled.append(self.postfix.clone());

                    (false, styled)
                }
            }
        }
    }

    pub fn indent(self, theme: Theme) -> impl View {
        let id = uuid::Uuid::new_v4().to_string();
        let id_cb = id.clone();
        let mut list = SelectView::new();
        list.add_all(self.indent_inner(&theme, 0));
        highlight_list_item(&mut list);
        list.set_on_submit(move |siv: &mut Cursive, expansion_state: &Rc<Cell<ExpansionState>>| {
            match expansion_state.get() {
                ExpansionState::Collapsed => expansion_state.set(ExpansionState::Expanded),
                ExpansionState::Expanded => expansion_state.set(ExpansionState::Collapsed),
                ExpansionState::Solid => {},
            }

            siv.call_on(
                &Selector::Id(&id_cb),
                |view: &mut OnEventView<SelectView<Rc<Cell<ExpansionState>>>>| {
                    let list = view.get_inner_mut();
                    let selected = list.selected_id();
                    list.clear();
                    list.add_all(self.indent_inner(&theme, 0));
                    if let Some(idx) = selected {
                        list.set_selection(idx);
                    }
                    highlight_list_item(list);
                },
            );
        });

        OnEventView::new(list)
            .on_pre_event_inner(EventTrigger::any(), |list, event| {
                let result = list.on_event(event.clone());
                highlight_list_item(list);

                Some(result)
            })
            .with_id(id)
    }

    fn indent_inner<'a>(&'a self, theme: &Theme, level: usize) ->
        Vec<(StyledString, Rc<Cell<ExpansionState>>)>
    {
        let indent = StyledString::plain(repeat_str(" ", level * 2));
        match &self.kind {
            ValueKind::Null | ValueKind::Bool(_) | ValueKind::Number(_) |
            ValueKind::String(_) | ValueKind::Binary(_) => {
                let mut styled = indent;
                styled.append_styled(" ", theme.tree_control);
                styled.append_plain(" ");
                styled.append(self.prefix.clone());
                styled.append(self.content.clone());
                styled.append(self.postfix.clone());

                vec![(styled, self.expansion_state.clone())]
            },
            recursive => {
                let iter: Box<dyn std::iter::Iterator<Item = &Value>> = match recursive {
                    ValueKind::Array(a) => Box::new(a.iter()),
                    ValueKind::Object(map) => Box::new(map.values()),
                    _ => unreachable!(),
                };
                let mut result = Vec::new();

                if let ExpansionState::Expanded = self.expansion_state.get() {
                    let mut prefix = indent.clone();
                    prefix.append_styled("-", theme.tree_control);
                    prefix.append_plain(" ");
                    prefix.append(self.prefix.clone());
                    result.push((prefix, self.expansion_state.clone()));

                    for value in iter {
                        result.extend(value.indent_inner(theme, level + 1));
                    }

                    let mut postfix = indent.clone();
                    postfix.append_styled(" ", theme.tree_control);
                    postfix.append_plain(" ");
                    postfix.append(self.postfix.clone());
                    result.push((postfix, self.expansion_state.clone()));
                } else {
                    let mut abbr = indent.clone();
                    abbr.append_styled("+", theme.tree_control);
                    abbr.append_plain(" ");
                    abbr.append(self.prefix.clone());
                    abbr.append_styled(" … ", theme.abbreviation);
                    abbr.append(self.postfix.clone());
                    result.push((abbr, self.expansion_state.clone()));
                }

                result
            },
        }
    }

    pub fn style(&mut self, theme: &Theme) {
        self.style_inner(theme, StyledString::default(), StyledString::default());
    }

    fn style_inner(&mut self, theme: &Theme, prefix: StyledString, postfix: StyledString) {
        match &mut self.kind {
            ValueKind::Array(ref mut a) => {
                self.expansion_state.set(ExpansionState::Expanded);
                self.prefix = prefix;
                self.prefix.append_styled("[", theme.brace);

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
