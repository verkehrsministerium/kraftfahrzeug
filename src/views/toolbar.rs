use cursive::theme::{Color, BaseColor, ColorType, ColorStyle};
use cursive::views::{Canvas, BoxView, TextView, OnEventView, LinearLayout};
use cursive::direction::Orientation;
use cursive::utils::markup::StyledString;
use cursive::traits::View;
use cursive::align::HAlign;
use cursive::Cursive;
use cursive::event::{Event, MouseEvent, MouseButton, Key};

fn toolbar_button<F, E: Into<Event>>(
    siv: &mut Cursive,
    key: &str,
    label: &str,
    event: E,
    cb: F,
) -> OnEventView<TextView>
where
    F: Fn(&mut Cursive) + Clone + 'static,
{
    log::info!("Creating toolbar button [{}|{}]", key, label);
    let mut styled = StyledString::styled(format!(" {} ", key), ColorStyle {
        front: ColorType::Color(Color::Dark(BaseColor::Blue)),
        back: ColorType::Color(Color::Light(BaseColor::White)),
    });
    styled.append_styled(format!("{} ", label), ColorStyle {
        front: ColorType::Color(Color::Light(BaseColor::White)),
        back: ColorType::Color(Color::Dark(BaseColor::Blue)),
    });
    siv.add_global_callback(event, cb.clone());
    OnEventView::new(TextView::new(styled))
        .on_pre_event(|ev: &Event| {
            log::info!("{:?}", ev);
            if let Event::Mouse {
                offset: _,
                position: _,
                event,
            } = ev {
                if let MouseEvent::Release(MouseButton::Left) = event {
                    return true;
                }
            }

            false
        }, move |siv| {
            log::info!("callback");
            cb(siv);
        })
}

pub fn toolbar_mockup(siv: &mut Cursive, style: ColorStyle) -> impl View {
    let mut toolbar = LinearLayout::new(Orientation::Horizontal);
    toolbar.add_child(toolbar_button(siv, "RET", "Inspect", Key::Enter, |_| {}));
    toolbar.add_child(toolbar_button(siv, "SPC", "Send", ' ', |_| {}));
    toolbar.add_child(toolbar_button(siv, "/", "Filter", '/', |_| {}));
    toolbar.add_child(toolbar_button(siv, "M", "Mode", 'm', |_| {}));
    toolbar.add_child(toolbar_button(siv, "Q", "Quit", 'q', Cursive::quit));
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

    toolbar
}
