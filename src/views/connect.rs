use cursive::view::{Boxable, Identifiable, View};
use cursive::views::{Button, DummyView, EditView, LinearLayout, PaddedView, Panel};
use cursive::Cursive;
use cursive_aligned_view::Alignable;

use crate::utils::{self, PrimaryView};

pub fn connect_mockup<F>(submit: F) -> impl View
where
    F: Fn(&mut Cursive, &str) + Copy + Send + Sync + 'static,
{
    let id = uuid::Uuid::new_v4().to_string();
    let id2 = id.clone();

    let edit = EditView::new()
        .filler(" ")
        .style(utils::secondary())
        .on_submit(move |siv, content| submit(siv, content))
        .with_id(id)
        .full_width();
    let connect_btn = Button::new("Connect", move |siv| {
        let sink = siv.cb_sink().clone();
        siv.call_on_id(&id2, |edit: &mut EditView| {
            let content: String = (*edit.get_content()).clone();
            sink.send(Box::new(move |siv| submit(siv, &content)))
                .expect("cursive sink failed");
        });
    })
    .with_primary_view();

    let content = PaddedView::new(
        (1, 1, 0, 0),
        LinearLayout::vertical()
            .child(edit)
            .child(DummyView)
            .child(connect_btn),
    );

    Panel::new(content)
        .title("Websocket Address")
        .max_width(80)
        .align_center()
}
