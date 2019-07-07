extern crate kraftfahrzeug;

use cursive::Cursive;
use cursive::direction::Orientation;
use cursive::views::{TextView, LinearLayout};
use cursive::theme::{Color, BaseColor, Effect, PaletteColor};

use serde_json;

use kraftfahrzeug::message::{Theme, Value};
use kraftfahrzeug::message::view::MessageView;

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

fn main() {
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

    let mut siv = Cursive::default();
    let theme = kraftfahrzeug::utils::custom_theme_from_cursive(&siv);
    siv.set_theme(theme);

    // We can quit by pressing `q`
    siv.add_global_callback('q', Cursive::quit);

    let val = Value::new(&MESSAGE_THEME, serde_json::from_str::<serde_json::Value>(data).unwrap());

    let mut layout = LinearLayout::new(Orientation::Vertical);
    layout.add_child(TextView::new(val.abbreviate(80)));
    layout.add_child(MessageView::new(val));
    siv.add_fullscreen_layer(layout);

    siv.run();
}
