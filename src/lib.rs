use std::borrow::Cow;

use iced_widget::core::theme::{Base, Style};
use iced_widget::core::{color, Color};
use utils::{lightness, mix};

pub mod button;
pub mod checkbox;
pub mod combo_box;
pub mod container;
#[cfg(feature = "dialog")]
pub mod dialog;
#[cfg(feature = "markdown")]
pub mod markdown;
pub mod menu;
pub mod pane_grid;
pub mod pick_list;
pub mod progress_bar;
#[cfg(feature = "qr_code")]
pub mod qr_code;
pub mod radio;
pub mod rule;
pub mod scrollable;
pub mod slider;
#[cfg(feature = "svg")]
pub mod svg;
pub mod text;
pub mod text_editor;
pub mod text_input;
pub mod toggler;
pub mod utils;

#[allow(clippy::cast_precision_loss)]
macro_rules! from_argb {
    ($hex:expr) => {{
        let hex = $hex as u32;

        let a = ((hex & 0xff000000) >> 24) as f32 / 255.0;
        let r = (hex & 0x00ff0000) >> 16;
        let g = (hex & 0x0000ff00) >> 8;
        let b = (hex & 0x000000ff);

        ::iced_widget::core::color!(r as u8, g as u8, b as u8, a)
    }};
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(from = "Custom", into = "Custom"))]
pub enum Theme {
    Dark,
    Light,
    #[cfg(feature = "system-theme")]
    System,
    Custom(Custom),
}

impl Theme {
    pub const ALL: &'static [Self] = &[Self::Dark, Self::Light];

    pub fn new(name: impl Into<Cow<'static, str>>, colorscheme: ColorScheme) -> Self {
        Self::Custom(Custom {
            name: name.into(),
            is_dark: lightness(colorscheme.surface.color) <= 0.5,
            colorscheme,
        })
    }

    pub const fn new_const(name: &'static str, colorscheme: ColorScheme) -> Self {
        Self::Custom(Custom {
            name: Cow::Borrowed(name),
            is_dark: lightness(colorscheme.surface.color) <= 0.5,
            colorscheme,
        })
    }

    pub fn name(&self) -> Cow<'static, str> {
        match self {
            Self::Dark => "Dark".into(),
            Self::Light => "Light".into(),
            #[cfg(feature = "system-theme")]
            Self::System => "System".into(),
            Self::Custom(custom) => custom.name.clone(),
        }
    }

    pub fn is_dark(&self) -> bool {
        match self {
            Self::Dark => true,
            Self::Light => false,
            #[cfg(feature = "system-theme")]
            Self::System => !dark_light::detect()
                .ok()
                .is_some_and(|mode| mode == dark_light::Mode::Light),
            Self::Custom(custom) => custom.is_dark,
        }
    }

    pub fn colors(&self) -> ColorScheme {
        match self {
            Self::Dark => ColorScheme::DARK,
            Self::Light => ColorScheme::LIGHT,
            #[cfg(feature = "system-theme")]
            Self::System => {
                if dark_light::detect()
                    .ok()
                    .is_some_and(|mode| mode == dark_light::Mode::Light)
                {
                    ColorScheme::LIGHT
                } else {
                    ColorScheme::DARK
                }
            }
            Self::Custom(custom) => custom.colorscheme,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        #[cfg(feature = "system-theme")]
        {
            Self::System
        }

        #[cfg(not(feature = "system-theme"))]
        {
            Self::Dark
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Base for Theme {
    fn base(&self) -> Style {
        Style {
            background_color: self.colors().surface.color,
            text_color: self.colors().surface.on_surface,
        }
    }

    fn palette(&self) -> Option<iced_widget::theme::Palette> {
        let colors = self.colors();

        Some(iced_widget::theme::Palette {
            background: colors.surface.color,
            text: colors.surface.on_surface,
            primary: colors.primary.color,
            success: colors.primary.primary_container,
            warning: mix(from_argb!(0xffffff00), colors.primary.color, 0.25),
            danger: colors.error.color,
        })
    }
}

#[cfg(feature = "animate")]
impl iced_anim::Animate for Theme {
    fn components() -> usize {
        ColorScheme::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        let mut colorscheme = self.colors();
        colorscheme.update(components);
        *self = Self::Custom(Custom {
            name: "Animating Theme".into(),
            is_dark: self.is_dark(),
            colorscheme,
        });
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        self.colors().distance_to(&end.colors())
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        let mut colorscheme = self.colors();
        colorscheme.lerp(&start.colors(), &end.colors(), progress);
        *self = Self::Custom(Custom {
            name: "Animating Theme".into(),
            is_dark: self.is_dark(),
            colorscheme,
        });
    }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Custom {
    pub name: Cow<'static, str>,
    pub is_dark: bool,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub colorscheme: ColorScheme,
}

impl From<Custom> for Theme {
    fn from(custom: Custom) -> Self {
        Self::Custom(custom)
    }
}

impl From<Theme> for Custom {
    fn from(theme: Theme) -> Self {
        Self {
            name: theme.name(),
            is_dark: theme.is_dark(),
            colorscheme: theme.colors(),
        }
    }
}

impl Clone for Custom {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            is_dark: self.is_dark,
            colorscheme: self.colorscheme,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.name.clone_from(&source.name);
        self.is_dark = source.is_dark;
        self.colorscheme = source.colorscheme;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ColorScheme {
    pub primary: Primary,
    pub secondary: Secondary,
    pub tertiary: Tertiary,
    pub error: Error,
    pub surface: Surface,
    pub inverse: Inverse,
    pub outline: Outline,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub shadow: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub scrim: Color,
}

#[allow(clippy::cast_precision_loss)]
impl ColorScheme {
    const DARK: Self = Self {
        primary: Primary {
            color: color!(0x9bd4a1),
            on_primary: color!(0x003916),
            primary_container: color!(0x1b5129),
            on_primary_container: color!(0xb6f1bb),
        },
        secondary: Secondary {
            color: color!(0xb8ccb6),
            on_secondary: color!(0x233425),
            secondary_container: color!(0x394b3a),
            on_secondary_container: color!(0xd3e8d1),
        },
        tertiary: Tertiary {
            color: color!(0xa1ced7),
            on_tertiary: color!(0x00363e),
            tertiary_container: color!(0x1f4d55),
            on_tertiary_container: color!(0xbdeaf4),
        },
        error: Error {
            color: color!(0xffb4ab),
            on_error: color!(0x690005),
            error_container: color!(0x93000a),
            on_error_container: color!(0xffdad6),
        },
        surface: Surface {
            color: color!(0x101510),
            on_surface: color!(0xe0e4dc),
            on_surface_variant: color!(0xc1c9be),
            surface_container: SurfaceContainer {
                lowest: color!(0x0b0f0b),
                low: color!(0x181d18),
                base: color!(0x1c211c),
                high: color!(0x262b26),
                highest: color!(0x313631),
            },
        },
        inverse: Inverse {
            inverse_surface: color!(0xe0e4dc),
            inverse_on_surface: color!(0x2d322c),
            inverse_primary: color!(0x34693f),
        },
        outline: Outline {
            color: color!(0x8b9389),
            variant: color!(0x414941),
        },
        shadow: color!(0x000000),
        scrim: from_argb!(0x4d000000),
    };

    const LIGHT: Self = Self {
        primary: Primary {
            color: color!(0x34693f),
            on_primary: color!(0xffffff),
            primary_container: color!(0xb6f1bb),
            on_primary_container: color!(0x1b5129),
        },
        secondary: Secondary {
            color: color!(0x516351),
            on_secondary: color!(0xffffff),
            secondary_container: color!(0xd3e8d1),
            on_secondary_container: color!(0x394b3a),
        },
        tertiary: Tertiary {
            color: color!(0x39656d),
            on_tertiary: color!(0xffffff),
            tertiary_container: color!(0xbdeaf4),
            on_tertiary_container: color!(0x1f4d55),
        },
        error: Error {
            color: color!(0xba1a1a),
            on_error: color!(0xffffff),
            error_container: color!(0xffdad6),
            on_error_container: color!(0x93000a),
        },
        surface: Surface {
            color: color!(0xf7fbf2),
            on_surface: color!(0x181d18),
            on_surface_variant: color!(0x414941),
            surface_container: SurfaceContainer {
                lowest: color!(0xffffff),
                low: color!(0xf1f5ed),
                base: color!(0xebefe7),
                high: color!(0xe5e9e1),
                highest: color!(0xe0e4dc),
            },
        },
        inverse: Inverse {
            inverse_surface: color!(0x2d322c),
            inverse_on_surface: color!(0xeef2ea),
            inverse_primary: color!(0x9bd4a1),
        },
        outline: Outline {
            color: color!(0x727970),
            variant: color!(0xc1c9be),
        },
        shadow: color!(0x000000),
        scrim: from_argb!(0x4d000000),
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Primary {
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub color: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub on_primary: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub primary_container: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub on_primary_container: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Secondary {
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub color: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub on_secondary: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub secondary_container: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub on_secondary_container: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tertiary {
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub color: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub on_tertiary: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub tertiary_container: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub on_tertiary_container: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Error {
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub color: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub on_error: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub error_container: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub on_error_container: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Surface {
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub color: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub on_surface: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub on_surface_variant: Color,
    pub surface_container: SurfaceContainer,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SurfaceContainer {
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub lowest: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub low: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub base: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub high: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub highest: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Inverse {
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub inverse_surface: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub inverse_on_surface: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub inverse_primary: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Outline {
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub color: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub variant: Color,
}

#[cfg(feature = "serde")]
mod color_serde {
    use iced_widget::core::Color;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::utils::{color_to_argb, parse_argb};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)
            .map(|hex| parse_argb(&hex))?
            .unwrap_or(Color::TRANSPARENT))
    }

    pub fn serialize<S>(color: &Color, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        color_to_argb(*color).serialize(serializer)
    }
}
