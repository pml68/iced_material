use iced_widget::checkbox::{Catalog, Status, Style, StyleFn};
use iced_widget::core::{Background, Border, Color, border};

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

pub fn styled(
    background_color: Color,
    background_unchecked: Option<Color>,
    icon_color: Color,
    border_color: Color,
    text_color: Option<Color>,
    is_checked: bool,
) -> Style {
    Style {
        background: Background::Color(if is_checked {
            background_color
        } else {
            background_unchecked.unwrap_or(Color::TRANSPARENT)
        }),
        icon_color,
        border: if is_checked {
            border::rounded(2)
        } else {
            Border {
                color: border_color,
                width: 2.0,
                radius: 2.into(),
            }
        },
        text_color,
    }
}

pub fn default(theme: &Theme, status: Status) -> Style {
    let surface = theme.colors().surface;
    let primary = theme.colors().primary;

    match status {
        Status::Active { is_checked } => styled(
            primary.color,
            None,
            primary.on_primary,
            surface.on_surface_variant,
            Some(surface.on_surface),
            is_checked,
        ),
        Status::Hovered { is_checked } => styled(
            mix(primary.color, surface.on_surface, HOVERED_LAYER_OPACITY),
            Some(Color {
                a: HOVERED_LAYER_OPACITY,
                ..surface.on_surface
            }),
            primary.on_primary,
            surface.on_surface_variant,
            Some(surface.on_surface),
            is_checked,
        ),
        Status::Disabled { is_checked } => styled(
            disabled_text(surface.on_surface),
            None,
            surface.color,
            disabled_text(surface.on_surface),
            Some(surface.on_surface),
            is_checked,
        ),
    }
}

pub fn error(theme: &Theme, status: Status) -> Style {
    let surface = theme.colors().surface;
    let error = theme.colors().error;

    match status {
        Status::Active { is_checked } => styled(
            error.color,
            None,
            error.on_error,
            error.color,
            Some(error.color),
            is_checked,
        ),
        Status::Hovered { is_checked } => styled(
            mix(error.color, surface.on_surface, HOVERED_LAYER_OPACITY),
            Some(Color {
                a: HOVERED_LAYER_OPACITY,
                ..error.color
            }),
            error.on_error,
            error.color,
            Some(error.color),
            is_checked,
        ),
        Status::Disabled { is_checked } => styled(
            disabled_text(surface.on_surface),
            None,
            surface.color,
            disabled_text(surface.on_surface),
            Some(surface.on_surface),
            is_checked,
        ),
    }
}
