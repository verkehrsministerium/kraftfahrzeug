use std::rc::Rc;

use cursive::direction::Orientation;
use cursive::theme::{BaseColor, Color, Effect, PaletteColor};
use cursive::view::{View, ViewWrapper};
use cursive::views::{LinearLayout, TextView};
use cursive::Vec2;

use serde_json;

use crate::message::view::MessageView;
use crate::message::{Theme, Value};

lazy_static::lazy_static! {
    static ref MESSAGE_THEME: Theme = Theme {
        null: Color::Light(BaseColor::Magenta).into(),
        bool: Color::Light(BaseColor::Cyan).into(),
        number: Color::Light(BaseColor::Blue).into(),
        string: Color::Light(BaseColor::Green).into(),
        brace: Color::Light(BaseColor::Yellow).into(),
        name: PaletteColor::Primary.into(),
        separator: PaletteColor::Primary.into(),
        abbreviation: Color::Light(BaseColor::Red).into(),
        tree_control: Effect::Bold.into(),
    };
}

pub fn message_inspect_mockup() -> impl View {
    let data = "
    {
      \"Actors\": [
        {
          \"name\": \"Tom Cruise\",
          \"age\": 56,
          \"Born At\": \"Syracuse, NY\",
          \"Birthdate\": \"July 3, 1962\",
          \"photo\": \"https://jsonformatter.org/img/tom-cruise.jpg\",
          \"wife\": null,
          \"weight\": 67.5,
          \"hasChildren\": true,
          \"hasGreyHair\": false,
          \"children\": [
            \"Suri\",
            \"Isabella Jane\",
            \"Connor\"
          ]
        },
        {
          \"name\": \"Robert Downey Jr.\",
          \"age\": 53,
          \"Born At\": \"New York City, NY\",
          \"Birthdate\": \"April 4, 1965\",
          \"photo\": \"https://jsonformatter.org/img/Robert-Downey-Jr.jpg\",
          \"wife\": \"Susan Downey\",
          \"weight\": 77.1,
          \"hasChildren\": true,
          \"hasGreyHair\": false,
          \"children\": [
            \"Indio Falconer\",
            \"Avri Roel\",
            \"Exton Elias\"
          ]
        }
      ]
    }
";

    let val = Value::new(
        &MESSAGE_THEME,
        serde_json::from_str::<serde_json::Value>(data).unwrap(),
    );

    let mut layout = LinearLayout::new(Orientation::Vertical);
    layout.add_child(MessageSingleLineView::new(val.clone()));
    layout.add_child(MessageView::new(val));

    layout
}

pub struct MessageSingleLineView {
    message: Rc<Value<'static>>,
    view: TextView,
}

impl MessageSingleLineView {
    pub fn new(val: Rc<Value<'static>>) -> Self {
        Self {
            message: val,
            view: TextView::empty(),
        }
    }
}

impl ViewWrapper for MessageSingleLineView {
    cursive::wrap_impl!(self.view: TextView);

    fn wrap_required_size(&mut self, req: Vec2) -> Vec2 {
        self.view.set_content(self.message.abbreviate(req.x));

        Vec2 { x: req.x, y: 1 }
    }
}
