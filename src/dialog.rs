use iced_dialog::dialog::{Catalog, Style, StyleFn};
use iced_widget::container;
use iced_widget::core::{Background, border};

use super::{Theme, text};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> <Self as Catalog>::Class<'a> {
        Box::new(default)
    }

    fn default_container<'a>() -> <Self as container::Catalog>::Class<'a> {
        Box::new(default_container)
    }

    fn default_title<'a>() -> <Self as iced_widget::text::Catalog>::Class<'a> {
        Box::new(text::surface)
    }

    fn style(&self, class: &<Self as Catalog>::Class<'_>) -> Style {
        class(self)
    }
}

pub fn default_container(theme: &Theme) -> container::Style {
    let colors = theme.colors().surface;
    container::Style {
        background: Some(Background::Color(colors.surface_container.high)),
        text_color: Some(colors.on_surface_variant),
        border: border::rounded(28),
        ..container::Style::default()
    }
}

pub fn default(theme: &Theme) -> Style {
    Style {
        backdrop_color: theme.colors().scrim,
    }
}
