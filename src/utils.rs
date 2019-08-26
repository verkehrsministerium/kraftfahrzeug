use cursive::theme::{BaseColor, Color, ColorStyle, ColorType, Palette, PaletteColor, BorderStyle, Theme};
use cursive::utils::markup::StyledString;
use cursive::views::SelectView;
use cursive::{Cursive, Printer};
use cursive::view::{View, ViewWrapper};

/// Repeat the string `s` `n` times by concatenating.
pub fn repeat_str<S: Into<String> + Clone>(s: S, n: usize) -> String {
    std::iter::repeat(s.into()).take(n).collect::<String>()
}

pub fn kfz_primary(siv: &mut Cursive) -> ColorStyle {
    let palette = &siv.current_theme().palette;
    ColorStyle {
        front: ColorType::Color(*palette.custom("kfz-primary-fg").unwrap()),
        back: ColorType::Color(*palette.custom("kfz-primary-bg").unwrap()),
    }
}

pub fn secondary() -> ColorStyle {
    ColorStyle::new(
        PaletteColor::Secondary,
        Color::Dark(BaseColor::Black),
    )
}

// Get a theme instance which respects the terminals foreground and background colors.
pub fn kfz_theme() -> Theme {
    let mut palette = Palette::default();
    palette[PaletteColor::Background] = Color::TerminalDefault;
    palette[PaletteColor::Shadow] = Color::TerminalDefault;
    palette[PaletteColor::View] = Color::TerminalDefault;
    palette[PaletteColor::Primary] = Color::TerminalDefault;
    palette[PaletteColor::Secondary] = Color::Dark(BaseColor::White);
    palette[PaletteColor::Tertiary] = Color::Dark(BaseColor::Black);
    palette[PaletteColor::TitlePrimary] = Color::TerminalDefault;
    palette[PaletteColor::TitleSecondary] = Color::TerminalDefault;
    palette[PaletteColor::Highlight] = Color::Light(BaseColor::White);
    palette[PaletteColor::HighlightInactive] = Color::Dark(BaseColor::White);
    palette.set_color("kfz-primary-bg", Color::Dark(BaseColor::Blue));
    palette.set_color("kfz-primary-fg", Color::Light(BaseColor::White));

    Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        palette,
    }
}

pub trait PrimaryView: Sized {
    fn with_primary_view(self) -> PrimaryViewWrapper<Self> {
        PrimaryViewWrapper {
            view: self,
        }
    }
}

impl<T: View> PrimaryView for T {}

pub struct PrimaryViewWrapper<T> {
    view: T,
}

impl<T: View> ViewWrapper for PrimaryViewWrapper<T> {
    cursive::wrap_impl!(self.view: T);

    fn wrap_draw(&self, printer: &Printer) {
        let mut patched_theme = printer.theme.clone();
        patched_theme.palette[PaletteColor::View] =
            *patched_theme.palette.custom("kfz-primary-bg").unwrap();
        printer.with_theme(&patched_theme, |printer| self.view.draw(printer));
    }
}

pub fn highlight_list_item<T: 'static>(list: &mut SelectView<T>) {
    let selection = list.selected_id();
    for i in 0..list.len() {
        let item = match list.get_item_mut(i) {
            Some(i) => i.0,
            None => continue,
        };

        *item = StyledString::with_spans(
            item.source(),
            item.spans_raw()
                .iter()
                .map(|span| {
                    let mut new_span = span.clone();
                    if let Some(ref mut color_style) = new_span.attr.color {
                        match selection {
                            Some(idx) if idx == i => {
                                color_style.front = match color_style.front {
                                    ColorType::Palette(PaletteColor::Primary) => {
                                        PaletteColor::Tertiary.into()
                                    }
                                    ColorType::Color(Color::Light(c)) => Color::Dark(c).into(),
                                    c => c,
                                };
                                color_style.back = match color_style.back {
                                    ColorType::Palette(PaletteColor::View) => {
                                        PaletteColor::Highlight.into()
                                    }
                                    c => c,
                                };
                            }
                            _ => {
                                color_style.front = match color_style.front {
                                    ColorType::Palette(PaletteColor::Tertiary) => {
                                        PaletteColor::Primary.into()
                                    }
                                    ColorType::Color(Color::Dark(c)) => Color::Light(c).into(),
                                    c => c,
                                };
                                color_style.back = match color_style.back {
                                    ColorType::Palette(PaletteColor::Highlight) => {
                                        PaletteColor::View.into()
                                    }
                                    c => c,
                                };
                            }
                        }
                    } else {
                        new_span.attr.color = Some(match selection {
                            Some(idx) if idx == i => ColorStyle {
                                front: ColorType::Palette(PaletteColor::Tertiary),
                                back: ColorType::Palette(PaletteColor::Highlight),
                            },
                            _ => ColorStyle {
                                front: ColorType::Palette(PaletteColor::Primary),
                                back: ColorType::Palette(PaletteColor::View),
                            },
                        });
                    }

                    new_span
                })
                .collect(),
        );
    }
}
