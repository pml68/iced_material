use iced_widget::core::{Background, Color};
use iced_widget::radio::{Catalog, Status, Style, StyleFn};

use super::Theme;
use crate::utils::{HOVERED_LAYER_OPACITY, disabled_text, mix};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn default(theme: &Theme, status: Status) -> Style {
    let surface = theme.colors().surface;
    let primary = theme.colors().primary;

    let active = Style {
        background: Color::TRANSPARENT.into(),
        dot_color: primary.color,
        border_width: 1.0,
        border_color: primary.color,
        text_color: None,
    };

    match status {
        Status::Active { is_selected } => Style {
            border_color: if is_selected {
                active.border_color
            } else {
                surface.on_surface
            },
            ..active
        },
        Status::Hovered { is_selected } => Style {
            dot_color: mix(
                primary.color,
                surface.on_surface,
                HOVERED_LAYER_OPACITY,
            ),
            border_color: if is_selected {
                mix(primary.color, surface.on_surface, HOVERED_LAYER_OPACITY)
            } else {
                disabled_text(surface.on_surface)
            },
            background: Background::Color(if is_selected {
                Color {
                    a: HOVERED_LAYER_OPACITY,
                    ..surface.on_surface
                }
            } else {
                Color::TRANSPARENT
            }),
            ..active
        },
    }
}
