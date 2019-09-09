use cursive::direction::Orientation;
use cursive::views::LinearLayout;
use cursive::view::Identifiable;
use cursive::Cursive;
use cursive_multiplex::Mux;
use cursive_flexi_logger_view::{FlexiLoggerView, cursive_flexi_logger};
use cursive_tabs::TabView;
use cursive_async_view::AsyncView;
use flexi_logger::{Logger, LogTarget};

mod message;
mod utils;
mod views;

use crate::utils::kfz_theme;

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("kraftfahrzeug")
        .expect("failed to initialize XDG base directories!");
    let config_path = xdg_dirs
        .place_config_file("logspec.toml")
        .expect("cannot create configuration directory!");
    let mut siv = Cursive::default();
    Logger::with_env_or_str("info,kraftfahrzeug=debug")
        .log_target(LogTarget::FileAndWriter(cursive_flexi_logger(&siv)))
        .directory("logs")
        .suppress_timestamp()
        .format(flexi_logger::colored_with_thread)
        .start_with_specfile(config_path)
        .expect("failed to initialize logger!");

    let theme = kfz_theme();
    siv.set_theme(theme);

    let tabs = TabView::new()
        .with_tab("connect", views::connect_mockup(|siv, url| {
            log::info!("Connecting to {}", url);

            siv.call_on_id("root-tabs", |tabs: &mut TabView<&'static str>| {
                let mut mux = Mux::new();

                let messages_id = mux
                    .add_right_of(views::messages_mockup(), mux.root().build().unwrap())
                    .expect("failed to add messages");
                let _message_inspect_id = mux
                    .add_right_of(views::message_inspect_mockup(), messages_id)
                    .expect("failed to add message-inspect");
                let _debug_id = mux
                    .add_below(FlexiLoggerView::scrollable(), messages_id)
                    .expect("failed to add debug-view");

                tabs.add_tab("socket", mux);
            });
        }))
        .with_id("root-tabs");

    let mut layout = LinearLayout::new(Orientation::Vertical);
    layout.add_child(views::titlebar_mockup(&mut siv));
    layout.add_child(tabs);
    layout.add_child(views::toolbar_mockup(&mut siv));

    siv.add_fullscreen_layer(layout);
    siv.run();
}
