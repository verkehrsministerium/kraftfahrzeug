use cursive::theme::{Color, BaseColor, ColorType, ColorStyle};
use cursive::views::{Canvas, BoxView, TextView, OnEventView, LinearLayout};
use cursive::direction::Orientation;
use cursive::utils::markup::StyledString;
use cursive::traits::View;
use cursive::align::HAlign;

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

pub fn toolbar_mockup(style: ColorStyle) -> impl View {
    let mut toolbar = LinearLayout::new(Orientation::Horizontal);
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

    toolbar
}
