use cursive::theme::Style;
use cursive::utils::markup::StyledString;
use cursive::views::{SelectView, OnEventView, IdView};
use cursive::event::EventTrigger;
use cursive::view::View;

use std::collections::hash_map::HashMap;
use std::fmt;
use std::rc::Rc;

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

pub enum Value {
    Null(Formatting),
    Bool(Formatting, bool),
    Number(Formatting, Number),
    String(Formatting, String),
    Array(Formatting, Vec<Value>),
    Binary(Formatting, Vec<u8>),
    Object(Formatting, HashMap<String, Value>),
}

#[derive(Debug)]
pub enum ExpansionState {
    Expanded,
    Collapsed,
}

pub struct Formatting {
    pub expansion_state: Rc<Option<ExpansionState>>,
    prefix: StyledString,
    content: StyledString,
    postfix: StyledString,
}

impl Default for Formatting {
    fn default() -> Self {
        Self {
            expansion_state: Rc::new(None),
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
    pub tree_control: Style,
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

    pub fn abbreviate(&self, available: usize, theme: &Theme) -> StyledString {
        self.abbreviate_inner(available, theme).1
    }

    fn abbreviate_inner(&self, available: usize, theme: &Theme) -> (bool, StyledString) {
        match self {
            Value::Null(f) |
            Value::Bool(f, _) |
            Value::Number(f, _) |
            Value::String(f, _) |
            Value::Binary(f, _) => {
                let mut styled = f.prefix.clone();
                styled.append(f.content.clone());
                styled.append(f.postfix.clone());

                if available < styled.width() {
                    (true, StyledString::styled("…", theme.abbreviation))
                } else {
                    (false, styled)
                }
            },
            Value::Array(f, a) => {
                let mut styled = f.prefix.clone();

                styled.append_plain(" ");
                let mut width = styled.width() + f.postfix.width() + 2;

                if available < width + 2 {
                    (true, StyledString::styled("…", theme.abbreviation))
                } else {
                    for value in a.iter() {
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

                        width = styled.width() + f.postfix.width();
                    }
                    styled.append(f.postfix.clone());

                    (false, styled)
                }
            },
            Value::Object(f, map) => {
                let mut styled = f.prefix.clone();

                styled.append_plain(" ");
                let mut width = styled.width() + f.postfix.width() + 2;

                if available < width {
                    (true, StyledString::styled("…", theme.abbreviation))
                } else {
                    for value in map.values() {
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

                        width = styled.width() + f.postfix.width();
                    }
                    styled.append(f.postfix.clone());

                    (false, styled)
                }
            },
        }
    }

    pub fn indent(self, theme: Theme) -> IdView<OnEventView<SelectView<Rc<Option<ExpansionState>>>>> {
        let mut list = SelectView::new();
        list.add_all(self.indent_inner(&theme, 0));
        highlight_list_item(&mut list);
        list.set_on_submit(move |siv, expansion_state_ref| {
            // oh no, he's hacking again...
            let expansion_state = unsafe { make_mut(expansion_state_ref) };
            if let Some(ref state_ref) = *expansion_state {
                eprintln!("alalalalalala");
                let state = unsafe { make_mut(state_ref) };
                match state {
                    ExpansionState::Collapsed => *state = ExpansionState::Expanded,
                    ExpansionState::Expanded => *state = ExpansionState::Collapsed,
                }
            }

            if let Some(ref mut list) = siv.find_id::<OnEventView<SelectView<Rc<Option<ExpansionState>>>>>("💩") {
                let selected = list.get_inner().selected_id();
                list.get_inner_mut().clear();
                list.get_inner_mut().add_all(self.indent_inner(&theme, 0));
                if let Some(idx) = selected {
                    list.get_inner_mut().set_selection(idx);
                }
                highlight_list_item(list.get_inner_mut());
            }
        });

        IdView::new("💩", OnEventView::new(list)
            .on_pre_event_inner(EventTrigger::any(), |list, event| {
                let result = list.on_event(event.clone());
                highlight_list_item(list);

                Some(result)
            }))
    }

    fn indent_inner<'a>(&'a self, theme: &Theme, level: usize) ->
        Vec<(StyledString, Rc<Option<ExpansionState>>)>
    {
        let indent = StyledString::plain(repeat_str(" ", level * 2));
        match self {
            Value::Null(f) |
            Value::Bool(f, _) |
            Value::Number(f, _) |
            Value::String(f, _) |
            Value::Binary(f, _) => {
                let mut styled = indent;
                styled.append_styled(" ", theme.tree_control);
                styled.append_plain(" ");
                styled.append(f.prefix.clone());
                styled.append(f.content.clone());
                styled.append(f.postfix.clone());

                vec![(styled, f.expansion_state.clone())]
            },
            Value::Array(f, a) => {
                let mut result = Vec::new();

                if let Some(ExpansionState::Expanded) = *f.expansion_state {
                    let mut prefix = indent.clone();
                    prefix.append_styled("-", theme.tree_control);
                    prefix.append_plain(" ");
                    prefix.append(f.prefix.clone());
                    result.push((prefix, f.expansion_state.clone()));

                    for value in a.iter() {
                        result.extend(value.indent_inner(theme, level + 1));
                    }

                    let mut postfix = indent.clone();
                    postfix.append_styled(" ", theme.tree_control);
                    postfix.append_plain(" ");
                    postfix.append(f.postfix.clone());
                    result.push((postfix, f.expansion_state.clone()));
                } else {
                    let mut abbr = indent.clone();
                    abbr.append_styled("+", theme.tree_control);
                    abbr.append_plain(" ");
                    abbr.append(f.prefix.clone());
                    abbr.append_styled(" … ", theme.abbreviation);
                    abbr.append(f.postfix.clone());
                    result.push((abbr, f.expansion_state.clone()));
                }

                result
            },
            Value::Object(f, map) => {
                let mut result = Vec::new();

                if let Some(ExpansionState::Expanded) = *f.expansion_state {
                    let mut prefix = indent.clone();
                    prefix.append_styled("-", theme.tree_control);
                    prefix.append_plain(" ");
                    prefix.append(f.prefix.clone());
                    result.push((prefix, f.expansion_state.clone()));

                    for value in map.values() {
                        result.extend(value.indent_inner(theme, level + 1));
                    }

                    let mut postfix = indent.clone();
                    postfix.append_styled(" ", theme.tree_control);
                    postfix.append_plain(" ");
                    postfix.append(f.postfix.clone());
                    result.push((postfix, f.expansion_state.clone()));
                } else {
                    let mut abbr = indent.clone();
                    abbr.append_styled("+", theme.tree_control);
                    abbr.append_plain(" ");
                    abbr.append(f.prefix.clone());
                    abbr.append_styled(" … ", theme.abbreviation);
                    abbr.append(f.postfix.clone());
                    result.push((abbr, f.expansion_state.clone()));
                }

                result
            },
        }
    }

    pub fn style(&mut self, theme: &Theme) {
        self.style_inner(theme, StyledString::default(), StyledString::default());
    }

    fn style_inner(&mut self, theme: &Theme, prefix: StyledString, postfix: StyledString) {
        match self {
            Value::Null(f) => {
                *Rc::get_mut(&mut f.expansion_state).unwrap() = None;
                f.prefix = prefix;
                f.content = StyledString::styled("null", theme.null);
                f.postfix = postfix;
            },
            Value::Bool(f, b) => {
                *Rc::get_mut(&mut f.expansion_state).unwrap() = None;
                f.prefix = prefix;
                f.content = StyledString::styled(format!("{}", b), theme.bool);
                f.postfix = postfix;
            },
            Value::Number(f, n) => {
                *Rc::get_mut(&mut f.expansion_state).unwrap() = None;
                f.prefix = prefix;
                f.content = StyledString::styled(format!("{}", n), theme.number);
                f.postfix = postfix;
            },
            Value::String(f, s) => {
                *Rc::get_mut(&mut f.expansion_state).unwrap() = None;
                f.prefix = prefix;
                f.content = StyledString::styled(format!("\"{}\"", s), theme.string);
                f.postfix = postfix;
            },
            Value::Array(f, a) => {
                *Rc::get_mut(&mut f.expansion_state).unwrap() = Some(ExpansionState::Expanded);
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
                *Rc::get_mut(&mut f.expansion_state).unwrap() = Some(ExpansionState::Collapsed);
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
                *Rc::get_mut(&mut f.expansion_state).unwrap() = Some(ExpansionState::Expanded);
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

// I'm hiding down here, nobody will see me 😈
unsafe fn make_mut<T>(reference: &T) -> &mut T {
    let const_ptr = reference as *const T;
    let mut_ptr = const_ptr as *mut T;
    &mut *mut_ptr
}
