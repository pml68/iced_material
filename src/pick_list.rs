use iced_widget::core::{Background, border};
use iced_widget::pick_list::{Catalog, Status, Style, StyleFn};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> <Self as Catalog>::Class<'a> {
        Box::new(default)
    }

    fn style(
        &self,
        class: &<Self as Catalog>::Class<'_>,
        status: Status,
    ) -> Style {
        class(self, status)
    }
}

pub fn default(theme: &Theme, status: Status) -> Style {
    let surface = theme.colors().surface;

    let active = Style {
        text_color: surface.on_surface,
        placeholder_color: surface.on_surface_variant,
        handle_color: surface.on_surface_variant,
        background: Background::Color(surface.surface_container.highest),
        border: border::rounded(4),
    };

    match status {
        Status::Active => active,
        Status::Hovered => Style {
            background: Background::Color(surface.surface_container.highest),
            ..active
        },
        Status::Opened { .. } => Style {
            background: Background::Color(surface.surface_container.highest),
            border: border::rounded(4),
            ..active
        },
    }
}
