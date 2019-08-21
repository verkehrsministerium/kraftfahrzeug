use cursive::theme::{Color, BaseColor, ColorType, ColorStyle};
use cursive::views::{LinearLayout, DebugView, ScrollView};
use cursive::direction::Orientation;
use cursive::Cursive;
use cursive_multiplex::Mux;

mod message;
mod utils;
mod views;

use crate::utils::custom_theme_from_cursive;

fn main() {
    cursive::logger::init();

    let mut siv = Cursive::default();

    let theme = custom_theme_from_cursive(&siv);
    siv.set_theme(theme);

    // We can quit by pressing `q`
    siv.add_global_callback('q', Cursive::quit);

    let (mut mux, messages_id) = Mux::new(views::messages_mockup());
    let _message_inspect_id = mux.add_right_of(views::message_inspect_mockup(), messages_id)
        .expect("failed to add message-inspect");
    let _debug_id = mux.add_below(ScrollView::new(DebugView::new()), messages_id)
        .expect("failed to add debug-view");

    let style = ColorStyle {
        front: ColorType::Color(Color::Light(BaseColor::White)),
        back: ColorType::Color(Color::Dark(BaseColor::Blue)),
    };
    let mut layout = LinearLayout::new(Orientation::Vertical);
    layout.add_child(views::titlebar_mockup(style.clone()));
    layout.add_child(mux);
    layout.add_child(views::toolbar_mockup(style.clone()));

    siv.add_fullscreen_layer(layout);
    siv.run();
}
