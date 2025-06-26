use iced_widget::core::Color;
use iced_widget::toggler::{Catalog, Status, Style, StyleFn};

use super::Theme;
use crate::utils::{
    HOVERED_LAYER_OPACITY, disabled_container, disabled_text, mix,
};

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
    background: Color,
    foreground: Color,
    border: Option<Color>,
) -> Style {
    Style {
        background,
        background_border_width: if border.is_some() { 2.0 } else { 0.0 },
        background_border_color: border.unwrap_or(Color::TRANSPARENT),
        foreground,
        foreground_border_width: 0.0,
        foreground_border_color: Color::TRANSPARENT,
    }
}

pub fn default(theme: &Theme, status: Status) -> Style {
    let surface = theme.colors().surface;
    let primary = theme.colors().primary;

    match status {
        Status::Active { is_toggled } => {
            if is_toggled {
                styled(primary.color, primary.on_primary, None)
            } else {
                styled(
                    surface.surface_container.highest,
                    theme.colors().outline.color,
                    Some(theme.colors().outline.color),
                )
            }
        }
        Status::Hovered { is_toggled } => {
            if is_toggled {
                styled(primary.color, primary.primary_container, None)
            } else {
                styled(
                    mix(
                        surface.surface_container.highest,
                        surface.on_surface,
                        HOVERED_LAYER_OPACITY,
                    ),
                    surface.on_surface_variant,
                    Some(theme.colors().outline.color),
                )
            }
        }
        Status::Disabled => styled(
            disabled_container(surface.surface_container.highest),
            disabled_text(surface.on_surface),
            Some(disabled_text(surface.on_surface)),
        ),
    }
}
