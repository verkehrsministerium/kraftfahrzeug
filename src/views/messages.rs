use cursive::theme::{Color, BaseColor};
use cursive::views::{SelectView, ScrollView, BoxView, OnEventView};
use cursive::view::{SizeConstraint, ScrollStrategy};
use cursive::utils::markup::StyledString;
use cursive::event::EventTrigger;
use cursive::view::View;

use crate::utils::highlight_list_item;

#[derive(PartialEq)]
enum Direction {
    Incoming,
    Outgoing,
}

struct Row<'a> {
    timestamp: &'a str,
    direction: Direction,
    content: &'a str,
}

pub fn messages_mockup() -> impl View {
    let items = vec![
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:23.835",
            direction: Direction::Outgoing,
            content: "Outgoing plain text message",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
        Row {
            timestamp: "12:31:21.534",
            direction: Direction::Incoming,
            content: "{\"data\": \"Incoming json message\", \"type\": \"String\" }",
        },
    ];

    let mut list = SelectView::<usize>::new();
    let rows = items.iter().enumerate().map(|(idx, item)| {
        let timestamp = StyledString::styled(item.timestamp, Color::Light(BaseColor::Black));
        let direction = if item.direction == Direction::Incoming {
            StyledString::styled(">", Color::Dark(BaseColor::Blue))
        } else {
            StyledString::styled("<", Color::Dark(BaseColor::Magenta))
        };
        let content = StyledString::plain(item.content);
        let mut row = StyledString::new();
        row.append(timestamp);
        row.append(StyledString::plain(" "));
        row.append(direction);
        row.append(StyledString::plain(" "));
        row.append(content);
        (row, idx)
    });
    list.add_all(rows);
    highlight_list_item(&mut list);

    BoxView::new(
        SizeConstraint::Full,
        SizeConstraint::Full,
        ScrollView::new(
            BoxView::new(
                SizeConstraint::Full,
                SizeConstraint::Free,
                OnEventView::new(list)
                    .on_pre_event_inner(EventTrigger::any(), |list, event| {
                        let result = list.on_event(event.clone());
                        highlight_list_item(list);

                        Some(result)
                    }),
            ),
        ).scroll_strategy(ScrollStrategy::StickToBottom),
    )
}
