use cursive::theme::{Color, BaseColor, ColorType, ColorStyle};
use cursive::views::{LinearLayout, ScrollView};
use cursive::direction::Orientation;
use cursive::event::{Event, Key};
use cursive::Cursive;
use cursive_multiplex::Mux;

mod message;
mod utils;
mod views;

use crate::utils::custom_theme_from_cursive;
use crate::views::DebugView;

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("kraftfahrzeug")
        .expect("failed to initialize XDG base directories!");
    let config_path = xdg_dirs.place_config_file("logspec.toml")
        .expect("cannot create configuration directory!");
    flexi_logger::Logger::with_env_or_str("info,kraftfahrzeug=debug")
        .log_target(flexi_logger::LogTarget::FileAndWriter(
            views::cursive_log_writer(flexi_logger::with_thread)
        ))
        .directory("logs")
        .suppress_timestamp()
        .format(flexi_logger::colored_with_thread)
        .start_with_specfile(config_path)
        .expect("failed to initialize logger!");

    let mut siv = Cursive::default();

    let theme = custom_theme_from_cursive(&siv);
    siv.set_theme(theme);

    // We can quit by pressing `q`
    siv.add_global_callback('q', Cursive::quit);

    let (mut mux, messages_id) = Mux::new(views::messages_mockup());
    mux.set_move_focus_up(Event::Alt(Key::Up));
    mux.set_move_focus_right(Event::Alt(Key::Right));
    mux.set_move_focus_down(Event::Alt(Key::Down));
    mux.set_move_focus_left(Event::Alt(Key::Left));

    let _message_inspect_id = mux.add_right_of(views::message_inspect_mockup(), messages_id)
        .expect("failed to add message-inspect");
    let _debug_id = mux.add_below(
        ScrollView::new(DebugView::new())
            .scroll_x(true)
            .scroll_y(true),
        messages_id,
    ).expect("failed to add debug-view");

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
