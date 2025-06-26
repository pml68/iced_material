use iced_widget::core::{Background, Color, border};
use iced_widget::slider::{
    Catalog, Handle, HandleShape, Rail, Status, Style, StyleFn,
};

use super::Theme;
use crate::utils::{HOVERED_LAYER_OPACITY, PRESSED_LAYER_OPACITY, mix};

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

pub fn styled(left: Color, right: Color, handle_radius: f32) -> Style {
    Style {
        rail: Rail {
            backgrounds: (left.into(), right.into()),
            width: 8.0,
            border: border::rounded(400),
        },
        handle: Handle {
            shape: HandleShape::Circle {
                radius: handle_radius,
            },
            background: Background::Color(left),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
    }
}

pub fn default(theme: &Theme, status: Status) -> Style {
    let surface = theme.colors().surface;
    let primary = theme.colors().primary;
    let secondary = theme.colors().secondary;

    match status {
        Status::Active => {
            styled(primary.color, secondary.secondary_container, 12.0)
        }
        Status::Hovered => styled(
            mix(primary.color, surface.on_surface, HOVERED_LAYER_OPACITY),
            secondary.secondary_container,
            12.0,
        ),
        Status::Dragged => styled(
            mix(primary.color, surface.on_surface, PRESSED_LAYER_OPACITY),
            secondary.secondary_container,
            11.0,
        ),
    }
}
