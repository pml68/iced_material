use iced_widget::core::{Background, border};
use iced_widget::overlay::menu::{Catalog, Style, StyleFn};

use super::Theme;
use crate::utils::{HOVERED_LAYER_OPACITY, mix};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> <Self as Catalog>::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &<Self as Catalog>::Class<'_>) -> Style {
        class(self)
    }
}

pub fn default(theme: &Theme) -> Style {
    let colors = theme.colors().surface;

    Style {
        border: border::rounded(4),
        background: Background::Color(colors.surface_container.base),
        text_color: colors.on_surface,
        selected_background: Background::Color(mix(
            colors.surface_container.base,
            colors.on_surface,
            HOVERED_LAYER_OPACITY,
        )),
        selected_text_color: colors.on_surface,
    }
}
