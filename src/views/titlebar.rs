use cursive::views::{Canvas, BoxView, TextView};
use cursive::traits::View;
use cursive::utils::markup::StyledString;
use cursive::align::HAlign;
use cursive::theme::ColorStyle;

pub fn titlebar_mockup(style: ColorStyle) -> impl View {
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
}
