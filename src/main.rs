use cursive::theme::{Color, BaseColor, PaletteColor, ColorType, ColorStyle};
use cursive::views::*;
use cursive::direction::Orientation;
use cursive::view::*;
use cursive::Cursive;
use cursive::utils::markup::StyledString;
use cursive::event::EventTrigger;
use cursive::traits::View;
use cursive::align::*;

mod abstract_object;

use kraftfahrzeug::custom_theme_from_cursive;

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

fn main() {
    cursive::logger::init();

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

    let mut siv = Cursive::default();

    let theme = custom_theme_from_cursive(&siv);
    siv.set_theme(theme);

    // We can quit by pressing `q`
    siv.add_global_callback('q', Cursive::quit);

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
    list.set_selection(list.len())(&mut siv);
    highlight_list_item(&mut list);

    let mut toolbar = LinearLayout::new(Orientation::Horizontal);
    let style = ColorStyle {
        front: ColorType::Color(Color::Light(BaseColor::White)),
        back: ColorType::Color(Color::Dark(BaseColor::Blue)),
    };
    toolbar.add_child(toolbar_button("RET", "Inspect"));
    toolbar.add_child(toolbar_button("SPC", "Send"));
    toolbar.add_child(toolbar_button("/", "Filter"));
    toolbar.add_child(toolbar_button("M", "Mode"));
    toolbar.add_child(toolbar_button("Q", "Quit"));
    toolbar.add_child(
        Canvas::wrap(
            BoxView::with_full_width(
                TextView::new(StyledString::styled("kraftfahrzeug v0.1.0", style))
                    .h_align(HAlign::Right)
            )
        ).with_draw(move |child, printer| {
            printer.with_style(style, |printer| {
                for y in 0..printer.size.y {
                    printer.print_hline((0, y), printer.size.x, " ");
                }
            });
            child.draw(printer);
        })
    );

    let mut layout = LinearLayout::new(Orientation::Vertical);
    layout.add_child(
        Canvas::wrap(
            BoxView::with_full_width(
                TextView::new(StyledString::styled("wss://buchholz.local/", style))
                    .h_align(HAlign::Center)
            )
        ).with_draw(move |child, printer| {
            printer.with_style(style, |printer| {
                for y in 0..printer.size.y {
                    printer.print_hline((0, y), printer.size.x, " ");
                }
            });
            child.draw(printer);
        })
    );
    layout.add_child(
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
        ),
    );
    layout.add_child(toolbar);
    siv.add_fullscreen_layer(layout);

    siv.run();
}

fn toolbar_button(key: &str, label: &str) -> OnEventView<TextView> {
    let mut styled = StyledString::styled(format!(" {} ", key), ColorStyle {
        front: ColorType::Color(Color::Dark(BaseColor::Blue)),
        back: ColorType::Color(Color::Light(BaseColor::White)),
    });
    styled.append_styled(format!("{} ", label), ColorStyle {
        front: ColorType::Color(Color::Light(BaseColor::White)),
        back: ColorType::Color(Color::Dark(BaseColor::Blue)),
    });
    OnEventView::new(TextView::new(styled))
}

fn highlight_list_item<T: 'static>(list: &mut SelectView<T>) {
    let selection = list.selected_id();
    for i in 0..list.len() {
        let item: &mut StyledString = list.get_item_mut(i).unwrap().0;
        *item = StyledString::with_spans(item.source(), item.spans_raw().iter().map(|span| {
            let mut new_span = span.clone();
            let (front, back) = match selection {
                Some(idx) if idx == i => (
                    ColorType::Palette(PaletteColor::Secondary),
                    ColorType::Palette(PaletteColor::Highlight),
                ),
                _ => (
                    ColorType::Palette(PaletteColor::Primary),
                    ColorType::Palette(PaletteColor::View),
                ),
            };

            if let Some(ref mut color_style) = new_span.attr.color {
                log::info!("Changed bg color {:?}", span.attr.color);
                match color_style.front {
                    ColorType::Palette(PaletteColor::Secondary)
                        | ColorType::Palette(PaletteColor::Primary) => color_style.front = front,
                    _ => {},
                };
                color_style.back = back;
            } else {
                new_span.attr.color = Some(ColorStyle { front, back });
            }

            new_span
        }).collect());
    }
}
