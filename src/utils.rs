use iced_widget::core::{Color, Shadow, Vector};

const COLOR_ERROR_MARGIN: f32 = 0.0001;

pub const HOVERED_LAYER_OPACITY: f32 = 0.08;
pub const PRESSED_LAYER_OPACITY: f32 = 0.1;

pub const DISABLED_TEXT_OPACITY: f32 = 0.38;
pub const DISABLED_CONTAINER_OPACITY: f32 = 0.12;

pub fn elevation(elevation_level: u8) -> f32 {
    (match elevation_level {
        0 => 0.0,
        1 => 1.0,
        2 => 3.0,
        3 => 6.0,
        4 => 8.0,
        _ => 12.0,
    } as f32)
}

pub fn shadow_from_elevation(elevation: f32, color: Color) -> Shadow {
    Shadow {
        color,
        offset: Vector {
            x: 0.0,
            y: elevation,
        },
        blur_radius: (elevation) * (1.0 + 0.4_f32.powf(elevation)),
    }
}

pub fn disabled_text(color: Color) -> Color {
    Color {
        a: DISABLED_TEXT_OPACITY,
        ..color
    }
}

pub fn disabled_container(color: Color) -> Color {
    Color {
        a: DISABLED_CONTAINER_OPACITY,
        ..color
    }
}

pub fn parse_argb(s: &str) -> Option<Color> {
    let hex = s.strip_prefix('#').unwrap_or(s);

    let parse_channel = |from: usize, to: usize| {
        let num = usize::from_str_radix(&hex[from..=to], 16).ok()? as f32 / 255.0;

        // If we only got half a byte (one letter), expand it into a full byte (two letters)
        Some(if from == to { num + num * 16.0 } else { num })
    };

    Some(match hex.len() {
        3 => Color::from_rgb(
            parse_channel(0, 0)?,
            parse_channel(1, 1)?,
            parse_channel(2, 2)?,
        ),
        4 => Color::from_rgba(
            parse_channel(1, 1)?,
            parse_channel(2, 2)?,
            parse_channel(3, 3)?,
            parse_channel(0, 0)?,
        ),
        6 => Color::from_rgb(
            parse_channel(0, 1)?,
            parse_channel(2, 3)?,
            parse_channel(4, 5)?,
        ),
        8 => Color::from_rgba(
            parse_channel(2, 3)?,
            parse_channel(4, 5)?,
            parse_channel(6, 7)?,
            parse_channel(0, 1)?,
        ),
        _ => None?,
    })
}

pub fn color_to_argb(color: Color) -> String {
    use std::fmt::Write;

    let mut hex = String::with_capacity(9);

    let [r, g, b, a] = color.into_rgba8();

    let _ = write!(&mut hex, "#");

    if a < u8::MAX {
        let _ = write!(&mut hex, "{a:02X}");
    }

    let _ = write!(&mut hex, "{r:02X}");
    let _ = write!(&mut hex, "{g:02X}");
    let _ = write!(&mut hex, "{b:02X}");

    hex
}

pub const fn lightness(color: Color) -> f32 {
    color.r * 0.299 + color.g * 0.587 + color.b * 0.114
}

pub fn mix(color1: Color, color2: Color, p2: f32) -> Color {
    if p2 <= 0.0 {
        return color1;
    } else if p2 >= 1.0 {
        return color2;
    }

    let p1 = 1.0 - p2;

    if (color1.a - 1.0).abs() > COLOR_ERROR_MARGIN || (color2.a - 1.0).abs() > COLOR_ERROR_MARGIN {
        let a = color1.a * p1 + color2.a * p2;
        if a > 0.0 {
            let c1 = color1.into_linear().map(|c| c * color1.a * p1);
            let c2 = color2.into_linear().map(|c| c * color2.a * p2);

            let [r, g, b] = [c1[0] + c2[0], c1[1] + c2[1], c1[2] + c2[2]].map(|u| u / a);

            return Color::from_linear_rgba(r, g, b, a);
        }
    }

    let c1 = color1.into_linear().map(|c| c * p1);
    let c2 = color2.into_linear().map(|c| c * p2);

    Color::from_linear_rgba(c1[0] + c2[0], c1[1] + c2[1], c1[2] + c2[2], c1[3] + c2[3])
}

#[cfg(test)]
mod tests {
    use super::{mix, Color};

    #[test]
    fn mixing() {
        let base = Color::from_rgba(1.0, 0.0, 0.0, 0.7);
        let overlay = Color::from_rgba(0.0, 1.0, 0.0, 0.2);

        assert_eq!(
            mix(base, overlay, 0.75).into_rgba8(),
            Color::from_linear_rgba(0.53846, 0.46154, 0.0, 0.325).into_rgba8()
        );
    }
}
