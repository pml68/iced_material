use iced_widget::core::border::Radius;
use iced_widget::rule::{Catalog, FillMode, Style, StyleFn};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(inset)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn inset(theme: &Theme) -> Style {
    Style {
        color: theme.colors().outline.variant,
        fill_mode: FillMode::Padded(8),
        radius: Radius::default(),
        snap: cfg!(feature = "crisp"),
    }
}
pub fn full_width(theme: &Theme) -> Style {
    Style {
        color: theme.colors().outline.variant,
        fill_mode: FillMode::Full,
        radius: Radius::default(),
        snap: cfg!(feature = "crisp"),
    }
}
