use cursive::views::{LinearLayout, EditView, Button, Dialog, DummyView, StackView};
use cursive::view::{View, Boxable, Identifiable};

use crate::utils::{self, PrimaryView};

fn submit(content: &str) {
    log::info!("connecting to {}", content);
}

pub fn connect_mockup() -> impl View {
    let mut layout = LinearLayout::vertical();
    let id = uuid::Uuid::new_v4().to_string();
    let id2 = id.clone();
    let edit = EditView::new()
        .filler(" ")
        .style(utils::secondary())
        .on_submit(|_siv, content| submit(content))
        .with_id(id)
        .full_width();

    layout.add_child(edit);
    layout.add_child(DummyView);
    layout.add_child(Button::new("Connect", move |siv| {
        siv.call_on_id(&id2, |edit: &mut EditView| {
            submit(&edit.get_content());
        });
    }).with_primary_view());

    let dialog = Dialog::around(layout)
        .padding((1, 1, 0, 0))
        .title("Websocket Address")
        .max_width(80);

    StackView::new().layer(dialog).full_screen()
}
