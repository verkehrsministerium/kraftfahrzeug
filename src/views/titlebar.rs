use cursive::align::HAlign;
use cursive::traits::View;
use cursive::utils::markup::StyledString;
use cursive::views::{BoxView, Canvas, TextView};
use cursive::Cursive;

use crate::utils;

pub fn titlebar_mockup(siv: &mut Cursive) -> impl View {
    let style = utils::kfz_primary(siv);
    Canvas::wrap(BoxView::with_full_width(
        TextView::new(StyledString::styled("wss://buchholz.local/", style)).h_align(HAlign::Center),
    ))
    .with_draw(move |child, printer| {
        printer.with_style(style, |printer| {
            for y in 0..printer.size.y {
                printer.print_hline((0, y), printer.size.x, " ");
            }
        });
        child.draw(printer);
    })
}
