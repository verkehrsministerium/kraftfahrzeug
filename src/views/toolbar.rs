use cursive::theme::{Color, BaseColor, ColorType, ColorStyle};
use cursive::views::{Canvas, BoxView, TextView, OnEventView, LinearLayout, Button};
use cursive::direction::{Direction, Orientation};
use cursive::utils::markup::StyledString;
use cursive::traits::View;
use cursive::align::HAlign;
use cursive::{Cursive, Printer, Vec2, Rect};
use cursive::event::{Event, Callback, EventTrigger, EventResult, MouseEvent, MouseButton, Key};

pub struct ToolbarButton {
    content: StyledString,
    callback: Callback,
}

impl ToolbarButton {
    pub fn new_with_style<S1, S2, E, C>(
        siv: &mut Cursive,
        style: &ColorStyle,
        key: S1,
        label: S2,
        event: E,
        callback: C,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        E: Into<Event>,
        C: Fn(&mut Cursive) + 'static + Clone,
    {
        let mut content = StyledString::new();
        content.append_styled(format!(" {} ", key.into()), ColorStyle {
            front: style.back,
            back: style.front,
        });
        content.append_styled(format!("{} ", label.into()), *style);
        siv.add_global_callback(event, callback.clone());

        Self {
            content,
            callback: Callback::from_fn_mut(callback),
        }
    }
}

impl View for ToolbarButton {
    fn draw(&self, printer: &Printer<'_, '_>) {
        printer.print_styled((0, 0), (&self.content).into());
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Mouse {
                event: MouseEvent::Release(MouseButton::Left),
                position,
                offset,
            } if position
                .fits_in_rect(offset, (self.content.width(), 1)) =>
            {
                EventResult::Consumed(Some(self.callback.clone()))
            }
            _ => EventResult::Ignored,
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        (self.content.width(), 1).into()
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        true
    }
}

pub fn toolbar_mockup(siv: &mut Cursive, style: ColorStyle) -> impl View {
    let mut toolbar = LinearLayout::new(Orientation::Horizontal);
    toolbar.add_child(ToolbarButton::new_with_style(siv, &style, "RET", "Inspect", Key::Enter, |_| {}));
    toolbar.add_child(ToolbarButton::new_with_style(siv, &style, "SPC", "Send", ' ', |_| {}));
    toolbar.add_child(ToolbarButton::new_with_style(siv, &style, "/", "Filter", '/', |_| {}));
    toolbar.add_child(ToolbarButton::new_with_style(siv, &style, "M", "Mode", 'm', |_| {}));
    toolbar.add_child(ToolbarButton::new_with_style(siv, &style, "Q", "Quit", 'q', Cursive::quit));
    toolbar.add_child(
        Canvas::wrap(
            BoxView::with_full_width(
                TextView::new(StyledString::styled(format!("kfz v{}", env!("CARGO_PKG_VERSION")), style))
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
