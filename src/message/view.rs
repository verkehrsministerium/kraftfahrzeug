use std::rc::Rc;
use std::cell::Cell;
use cursive::{Cursive, Printer, Rect, vec::Vec2};
use cursive::direction::Direction;
use cursive::event::{EventTrigger, Event, EventResult, AnyCb};
use cursive::view::{View, Identifiable, Selector};
use cursive::views::{IdView, OnEventView, SelectView};
use cursive::utils::markup::StyledString;

use super::{Value, ValueKind, ExpansionState};
use crate::utils;

type InnerView = IdView<OnEventView<SelectView<Rc<Cell<ExpansionState>>>>>;

pub struct MessageView {
    view: InnerView,
}

impl MessageView {
    // TODO: remove the static lifetime requirement
    // we can bound the lifetime of Value to the lifetime of the MessageView,
    // as the callback will not get called after MessageView has died.
    // However, this requires unsafe code, as `set_on_submit` requires the callback
    // to own all used data (leaving only 'static references or values). We should
    // keep the unsafe code restricted to the Value<'a> accesses...
    pub fn new(message: Rc<Value<'static>>) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let id_cb = id.clone();
        let mut list = SelectView::new();
        list.add_all(Self::indent(&message, 0));
        utils::highlight_list_item(&mut list);
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
                    list.add_all(Self::indent(&message, 0));
                    if let Some(idx) = selected {
                        list.set_selection(idx);
                    }
                    utils::highlight_list_item(list);
                },
            );
        });

        let view = OnEventView::new(list)
            .on_pre_event_inner(EventTrigger::any(), |list, event| {
                let result = list.on_event(event.clone());
                utils::highlight_list_item(list);

                Some(result)
            })
            .with_id(id);

        Self {
            view,
       }
    }

    fn indent(message: &Value, level: usize) ->
        Vec<(StyledString, Rc<Cell<ExpansionState>>)>
    {
        let indent = StyledString::plain(utils::repeat_str(" ", level * 2));
        match &message.kind {
            ValueKind::Null | ValueKind::Bool(_) | ValueKind::Number(_) |
            ValueKind::String(_) | ValueKind::Binary(_) => {
                let mut styled = indent;
                styled.append_styled(" ", message.theme.tree_control);
                styled.append_plain(" ");
                styled.append(message.prefix.clone());
                styled.append(message.content.clone());
                styled.append(message.postfix.clone());

                vec![(styled, message.expansion_state.clone())]
            },
            recursive => {
                let iter: Box<dyn std::iter::Iterator<Item = &Value>> = match recursive {
                    ValueKind::Array(a) => Box::new(a.iter()),
                    ValueKind::Object(map) => Box::new(map.values()),
                    _ => unreachable!(),
                };
                let mut result = Vec::new();

                if let ExpansionState::Expanded = message.expansion_state.get() {
                    let mut prefix = indent.clone();
                    prefix.append_styled("-", message.theme.tree_control);
                    prefix.append_plain(" ");
                    prefix.append(message.prefix.clone());
                    result.push((prefix, message.expansion_state.clone()));

                    for value in iter {
                        result.extend(Self::indent(value, level + 1));
                    }

                    let mut postfix = indent.clone();
                    postfix.append_styled(" ", message.theme.tree_control);
                    postfix.append_plain(" ");
                    postfix.append(message.postfix.clone());
                    result.push((postfix, message.expansion_state.clone()));
                } else {
                    let mut abbr = indent.clone();
                    abbr.append_styled("+", message.theme.tree_control);
                    abbr.append_plain(" ");
                    abbr.append(message.prefix.clone());
                    abbr.append_styled(" … ", message.theme.abbreviation);
                    abbr.append(message.postfix.clone());
                    result.push((abbr, message.expansion_state.clone()));
                }

                result
            },
        }
    }
}

impl View for MessageView {
    fn draw(&self, printer: &Printer) {
        self.view.draw(printer);
    }

    fn layout(&mut self, v: Vec2) {
        self.view.layout(v);
    }

    fn needs_relayout(&self) -> bool {
        self.view.needs_relayout()
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        self.view.required_size(constraint)
    }

    fn on_event(&mut self, ev: Event) -> EventResult {
        self.view.on_event(ev)
    }

    fn call_on_any<'a>(&mut self, sel: &Selector<'_>, cb: AnyCb<'a>) {
        self.view.call_on_any(sel, cb);
    }

    fn focus_view(&mut self, sel: &Selector<'_>) -> Result<(), ()> {
        self.view.focus_view(sel)
    }

    fn take_focus(&mut self, source: Direction) -> bool {
        self.view.take_focus(source)
    }

    fn important_area(&self, view_size: Vec2) -> Rect {
        self.view.important_area(view_size)
    }
}
