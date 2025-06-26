use iced_widget::button::{Catalog, Status, Style, StyleFn};
use iced_widget::core::{Background, Border, Color, border};

use crate::Theme;
use crate::utils::{
    HOVERED_LAYER_OPACITY, PRESSED_LAYER_OPACITY, disabled_container,
    disabled_text, elevation, mix, shadow_from_elevation,
};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(filled)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

fn button(
    background: Color,
    foreground: Color,
    disabled: Color,
    shadow_color: Color,
    elevation_level: u8,
    status: Status,
) -> Style {
    let active = Style {
        background: Some(Background::Color(background)),
        text_color: foreground,
        border: border::rounded(400),
        shadow: shadow_from_elevation(elevation(elevation_level), shadow_color),
        snap: cfg!(feature = "crisp"),
    };

    match status {
        Status::Active => active,
        Status::Pressed => Style {
            background: Some(Background::Color(mix(
                background,
                foreground,
                HOVERED_LAYER_OPACITY,
            ))),
            ..active
        },
        Status::Hovered => Style {
            background: Some(Background::Color(mix(
                background,
                foreground,
                PRESSED_LAYER_OPACITY,
            ))),
            shadow: shadow_from_elevation(
                elevation(elevation_level + 1),
                shadow_color,
            ),
            ..active
        },
        Status::Disabled => Style {
            background: Some(Background::Color(disabled_container(disabled))),
            text_color: disabled_text(disabled),
            border: border::rounded(400),
            ..Default::default()
        },
    }
}

pub fn elevated(theme: &Theme, status: Status) -> Style {
    let surface = theme.colors().surface;

    let foreground = theme.colors().primary.color;
    let background = surface.surface_container.low;
    let disabled = surface.on_surface;

    let shadow_color = theme.colors().shadow;

    button(background, foreground, disabled, shadow_color, 1, status)
}

pub fn filled(theme: &Theme, status: Status) -> Style {
    let primary = theme.colors().primary;

    let foreground = primary.on_primary;
    let background = primary.color;
    let disabled = theme.colors().surface.on_surface;

    let shadow_color = theme.colors().shadow;

    button(background, foreground, disabled, shadow_color, 0, status)
}

pub fn filled_tonal(theme: &Theme, status: Status) -> Style {
    let secondary = theme.colors().secondary;

    let foreground = secondary.on_secondary_container;
    let background = secondary.secondary_container;
    let disabled = theme.colors().surface.on_surface;
    let shadow_color = theme.colors().shadow;

    button(background, foreground, disabled, shadow_color, 0, status)
}

pub fn outlined(theme: &Theme, status: Status) -> Style {
    let foreground = theme.colors().primary.color;
    let background = Color::TRANSPARENT;
    let disabled = theme.colors().surface.on_surface;

    let outline = theme.colors().outline.color;

    let border = match status {
        Status::Active | Status::Pressed | Status::Hovered => Border {
            color: outline,
            width: 1.0,
            radius: 400.into(),
        },
        Status::Disabled => Border {
            color: disabled_container(disabled),
            width: 1.0,
            radius: 400.into(),
        },
    };

    let style = button(
        background,
        foreground,
        disabled,
        Color::TRANSPARENT,
        0,
        status,
    );

    Style { border, ..style }
}

pub fn text(theme: &Theme, status: Status) -> Style {
    let foreground = theme.colors().primary.color;
    let background = Color::TRANSPARENT;
    let disabled = theme.colors().surface.on_surface;

    let style = button(
        background,
        foreground,
        disabled,
        Color::TRANSPARENT,
        0,
        status,
    );

    match status {
        Status::Hovered | Status::Pressed => style,
        Status::Active | Status::Disabled => Style {
            background: None,
            ..style
        },
    }
}
