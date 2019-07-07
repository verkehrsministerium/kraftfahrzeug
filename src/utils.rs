use cursive::Cursive;
use cursive::theme::{ColorType, ColorStyle, Color, PaletteColor, BaseColor, Theme};
use cursive::views::{SelectView};
use cursive::utils::markup::StyledString;

/// Repeat the string `s` `n` times by concatenating.
pub fn repeat_str<S: Into<String> + Clone>(s: S, n: usize) -> String {
    std::iter::repeat(s.into()).take(n).collect::<String>()
}

// Get a theme instance which respects the terminals foreground and background colors.
pub fn custom_theme_from_cursive(siv: &Cursive) -> Theme {
    let mut theme = siv.current_theme().clone();

    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    theme.palette[PaletteColor::Shadow] = Color::TerminalDefault;
    theme.palette[PaletteColor::View] = Color::TerminalDefault;
    theme.palette[PaletteColor::Primary] = Color::TerminalDefault;
    theme.palette[PaletteColor::Secondary] = Color::Dark(BaseColor::Black);
    theme.palette[PaletteColor::Tertiary] = Color::TerminalDefault;
    theme.palette[PaletteColor::TitlePrimary] = Color::TerminalDefault;
    theme.palette[PaletteColor::TitleSecondary] = Color::TerminalDefault;
    theme.palette[PaletteColor::Highlight] = Color::Light(BaseColor::White);
    theme.palette[PaletteColor::HighlightInactive] = Color::TerminalDefault;

    theme
}

pub fn highlight_list_item<T: 'static>(list: &mut SelectView<T>) {
    let selection = list.selected_id();
    for i in 0..list.len() {
        let item = match list.get_item_mut(i) {
            Some(i) => i.0,
            None => continue,
        };

        *item = StyledString::with_spans(item.source(), item.spans_raw().iter().map(|span| {
            let mut new_span = span.clone();
            if let Some(ref mut color_style) = new_span.attr.color {
                match selection {
                    Some(idx) if idx == i => {
                        color_style.front = match color_style.front {
                            ColorType::Palette(PaletteColor::Primary) => PaletteColor::Secondary.into(),
                            ColorType::Color(Color::Light(c)) => Color::Dark(c).into(),
                            c => c,
                        };
                        color_style.back = match color_style.back {
                            ColorType::Palette(PaletteColor::View) => PaletteColor::Highlight.into(),
                            c => c,
                        };
                    },
                    _ => {
                        color_style.front = match color_style.front {
                            ColorType::Palette(PaletteColor::Secondary) => PaletteColor::Primary.into(),
                            ColorType::Color(Color::Dark(c)) => Color::Light(c).into(),
                            c => c,
                        };
                        color_style.back = match color_style.back {
                            ColorType::Palette(PaletteColor::Highlight) => PaletteColor::View.into(),
                            c => c,
                        };
                    },
                }
            } else {
                new_span.attr.color = Some(match selection {
                    Some(idx) if idx == i => {
                        ColorStyle {
                            front: ColorType::Palette(PaletteColor::Secondary),
                            back: ColorType::Palette(PaletteColor::Highlight),
                        }
                    },
                    _ => {
                        ColorStyle {
                            front: ColorType::Palette(PaletteColor::Primary),
                            back: ColorType::Palette(PaletteColor::View),
                        }
                    }
                });
            }

            new_span
        }).collect());
    }
}
