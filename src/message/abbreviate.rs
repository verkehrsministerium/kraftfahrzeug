use cursive::utils::markup::StyledString;

use super::{Value, ValueKind};

impl<'a> Value<'a> {
    pub fn abbreviate(&self, available: usize) -> StyledString {
        self.abbreviate_inner(available).1
    }

    fn abbreviate_inner(&self, available: usize) -> (bool, StyledString) {
        match &self.kind {
            ValueKind::Null | ValueKind::Bool(_) | ValueKind::Number(_) |
            ValueKind::String(_) | ValueKind::Binary(_) => {
                let mut styled = self.prefix.clone();
                styled.append(self.content.clone());
                styled.append(self.postfix.clone());

                if available < styled.width() {
                    (true, StyledString::styled("…", self.theme.abbreviation))
                } else {
                    (false, styled)
                }
            },
            recursive => {
                let iter: Box<dyn std::iter::Iterator<Item = &Value>> = match recursive {
                    ValueKind::Array(a) => Box::new(a.iter()),
                    ValueKind::Object(map) => Box::new(map.values()),
                    _ => unreachable!(),
                };
                let mut styled = self.prefix.clone();

                styled.append_plain(" ");
                let mut width = styled.width() + self.postfix.width() + 2;

                if available < width + 2 {
                    (true, StyledString::styled("…", self.theme.abbreviation))
                } else {
                    for value in iter {
                        // extra +1 because of the whitespace after this value
                        let (abbr, fmt) = value.abbreviate_inner(
                            available.checked_sub(width + 1).unwrap_or(0),
                        );
                        styled.append(fmt);
                        styled.append_plain(" ");

                        if abbr {
                            break;
                        }

                        width = styled.width() + self.postfix.width();
                    }
                    styled.append(self.postfix.clone());

                    (false, styled)
                }
            }
        }
    }
}
