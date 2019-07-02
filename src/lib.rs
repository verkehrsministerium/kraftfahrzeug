pub mod abstract_object;

use cursive::Cursive;
use cursive::theme::{PaletteColor, Theme, Color, BaseColor};

pub fn custom_theme_from_cursive(siv: &Cursive) -> Theme {
    // We'll return the current theme with a small modification.
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
