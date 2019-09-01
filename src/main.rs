use cursive::direction::Orientation;
use cursive::event::{Event, Key};
use cursive::views::{LinearLayout, ScrollView, StackView, LayerPosition};
use cursive::view::Identifiable;
use cursive::Cursive;
use cursive_multiplex::Mux;

mod message;
mod utils;
mod views;

use crate::utils::kfz_theme;
use crate::views::DebugView;

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("kraftfahrzeug")
        .expect("failed to initialize XDG base directories!");
    let config_path = xdg_dirs
        .place_config_file("logspec.toml")
        .expect("cannot create configuration directory!");
    flexi_logger::Logger::with_env_or_str("info,kraftfahrzeug=debug")
        .log_target(flexi_logger::LogTarget::FileAndWriter(
            views::cursive_log_writer(),
        ))
        .directory("logs")
        .suppress_timestamp()
        .format(flexi_logger::colored_with_thread)
        .start_with_specfile(config_path)
        .expect("failed to initialize logger!");

    let mut siv = Cursive::default();

    let theme = kfz_theme();
    siv.set_theme(theme);

    let mut mux = Mux::new()
        .with_move_focus_up(Event::Alt(Key::Up))
        .with_move_focus_right(Event::Alt(Key::Right))
        .with_move_focus_down(Event::Alt(Key::Down))
        .with_move_focus_left(Event::Alt(Key::Left));

    let messages_id = mux
        .add_right_of(views::messages_mockup(), mux.root().build().unwrap())
        .expect("failed to add messages");
    let _message_inspect_id = mux
        .add_right_of(views::message_inspect_mockup(), messages_id)
        .expect("failed to add message-inspect");
    let _debug_id = mux
        .add_below(
            ScrollView::new(DebugView::new())
                .scroll_x(true)
                .scroll_y(true)
                .show_scrollbars(true),
            messages_id,
        )
        .expect("failed to add debug-view");

    let stack = StackView::new()
        .fullscreen_layer(mux)
        .fullscreen_layer(views::connect_mockup(|siv, url| {
            log::info!("Connecting to {}", url);

            siv.call_on_id("stack", |stack: &mut StackView| {
                stack.move_to_front(LayerPosition::FromFront(1));
            });
        }))
        .with_id("stack");

    let mut layout = LinearLayout::new(Orientation::Vertical);
    layout.add_child(views::titlebar_mockup(&mut siv));
    layout.add_child(stack);
    layout.add_child(views::toolbar_mockup(&mut siv));

    siv.add_fullscreen_layer(layout);
    siv.run();
}
