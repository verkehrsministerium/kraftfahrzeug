extern crate kraftfahrzeug;

use cursive::Cursive;
use cursive::views::TextView;
use cursive::theme::{Color, BaseColor};

use serde_json;

use kraftfahrzeug::abstract_object::*;

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

    let mut val: Value = serde_json::from_str::<serde_json::Value>(data).unwrap().into();

    let mut siv = Cursive::default();
    let theme = kraftfahrzeug::custom_theme_from_cursive(&siv);
    siv.set_theme(theme);

    // We can quit by pressing `q`
    siv.add_global_callback('q', Cursive::quit);

    let theme = Theme {
        null: Color::Light(BaseColor::Magenta).into(),
        bool: Color::Light(BaseColor::Cyan).into(),
        number: Color::Light(BaseColor::Blue).into(),
        string: Color::Light(BaseColor::Green).into(),
        brace: Color::Light(BaseColor::Yellow).into(),
        name: Color::TerminalDefault.into(),
        separator: Color::TerminalDefault.into(),
        abbreviation: Color::Light(BaseColor::Red).into(),
    };
    val.style(&theme);

    if let Value::Object(_, ref mut map) = val {
        if let Some(Value::Array(_, ref mut arr)) = map.get_mut(&"Actors".to_owned()) {
            if let Some(Value::Object(ref mut f, _)) = arr.get_mut(1) {
                f.expanded = false;
            }
            if let Some(Value::Object(_, ref mut obj)) = arr.get_mut(0) {
                if let Some(Value::Array(ref mut f, _)) = obj.get_mut(&"children".to_owned()) {
                    f.expanded = true;
                }
                if let Some(Value::String(ref mut f, _)) = obj.get_mut(&"photo".to_owned()) {
                    f.expanded = false;
                }
            }
        }
    }

    siv.add_fullscreen_layer(TextView::new(val.abbreviate(80, &theme)));

    siv.run();
}
