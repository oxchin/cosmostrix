// Copyright (c) 2025 rezk_nightky

use crossterm::style::Color;

use crate::runtime::{ColorMode, ColorScheme};

#[derive(Clone, Debug)]
pub struct Palette {
    pub colors: Vec<Color>,
    pub bg: Option<Color>,
}

fn from_ansi_list(list: &[u8]) -> Vec<Color> {
    list.iter().map(|&v| Color::AnsiValue(v)).collect()
}

pub fn build_palette(
    scheme: ColorScheme,
    mode: ColorMode,
    default_background: bool,
) -> Palette {
    let mut bg = if default_background {
        None
    } else {
        Some(match mode {
            ColorMode::Color16 => Color::Black,
            ColorMode::TrueColor => Color::Rgb { r: 0, g: 0, b: 0 },
            _ => Color::AnsiValue(16),
        })
    };

    let colors: Vec<Color> = match scheme {
        ColorScheme::Green => match mode {
            ColorMode::Mono => vec![Color::White],
            ColorMode::Color16 => vec![Color::DarkGreen, Color::Green],
            _ => from_ansi_list(&[234, 22, 28, 35, 78, 84, 159]),
        },
        ColorScheme::Green2 => match mode {
            ColorMode::Mono => vec![Color::White],
            ColorMode::Color16 => vec![
                Color::DarkGrey,
                Color::DarkGreen,
                Color::Green,
                Color::White,
            ],
            _ => from_ansi_list(&[28, 34, 76, 84, 120, 157, 231]),
        },
        ColorScheme::Green3 => match mode {
            ColorMode::Mono => vec![Color::White],
            ColorMode::Color16 => vec![Color::DarkGreen, Color::White],
            _ => from_ansi_list(&[22, 28, 34, 70, 76, 82, 157]),
        },
        ColorScheme::Gold => match mode {
            ColorMode::Mono => vec![Color::White],
            ColorMode::Color16 => vec![
                Color::DarkGrey,
                Color::DarkYellow,
                Color::Yellow,
                Color::White,
            ],
            _ => from_ansi_list(&[58, 94, 172, 178, 228, 230, 231]),
        },
        ColorScheme::Yellow => match mode {
            ColorMode::Mono => vec![Color::White],
            ColorMode::Color16 => vec![Color::DarkGrey, Color::Yellow, Color::White],
            _ => from_ansi_list(&[100, 142, 184, 226, 227, 229, 230]),
        },
        ColorScheme::Orange => match mode {
            ColorMode::Mono => vec![Color::White],
            ColorMode::Color16 => vec![Color::Red, Color::Grey],
            _ => from_ansi_list(&[52, 94, 130, 166, 202, 208, 231]),
        },
        ColorScheme::Red => match mode {
            ColorMode::Mono => vec![Color::White],
            ColorMode::Color16 => vec![Color::DarkRed, Color::Red, Color::White],
            _ => from_ansi_list(&[234, 52, 88, 124, 160, 196, 217]),
        },
        ColorScheme::Blue => match mode {
            ColorMode::Mono => vec![Color::White],
            ColorMode::Color16 => vec![Color::DarkBlue, Color::Blue, Color::White],
            _ => from_ansi_list(&[234, 17, 18, 19, 20, 21, 75, 159]),
        },
        ColorScheme::Cyan => match mode {
            ColorMode::Mono => vec![Color::White],
            ColorMode::Color16 => vec![Color::DarkCyan, Color::Cyan, Color::White],
            _ => from_ansi_list(&[24, 25, 31, 32, 38, 45, 159]),
        },
        ColorScheme::Purple => match mode {
            ColorMode::Mono => vec![Color::White],
            ColorMode::Color16 => vec![Color::Magenta, Color::Grey],
            _ => from_ansi_list(&[60, 61, 62, 63, 69, 111, 225]),
        },
        ColorScheme::Neon => match mode {
            ColorMode::Mono => vec![Color::White],
            ColorMode::Color16 => vec![Color::Blue, Color::Magenta, Color::Cyan, Color::White],
            _ => from_ansi_list(&[17, 18, 19, 54, 93, 129, 201, 51, 231]),
        },
        ColorScheme::Fire => match mode {
            ColorMode::Mono => vec![Color::White],
            ColorMode::Color16 => vec![
                Color::DarkRed,
                Color::Red,
                Color::DarkYellow,
                Color::Yellow,
                Color::White,
            ],
            _ => from_ansi_list(&[52, 88, 124, 160, 196, 202, 208, 214, 226, 231]),
        },
        ColorScheme::Ocean => match mode {
            ColorMode::Mono => vec![Color::White],
            ColorMode::Color16 => vec![Color::DarkBlue, Color::Blue, Color::DarkCyan, Color::Cyan, Color::White],
            _ => from_ansi_list(&[17, 18, 19, 24, 30, 37, 44, 51, 87, 159, 231]),
        },
        ColorScheme::Forest => match mode {
            ColorMode::Mono => vec![Color::White],
            ColorMode::Color16 => vec![Color::DarkGreen, Color::Green, Color::Yellow, Color::White],
            _ => from_ansi_list(&[22, 28, 34, 40, 46, 82, 118, 154, 190, 229, 231]),
        },
        ColorScheme::Vaporwave => match mode {
            ColorMode::Mono => vec![Color::White],
            ColorMode::Color16 => vec![
                Color::Magenta,
                Color::Magenta,
                Color::Yellow,
                Color::Cyan,
                Color::White,
            ],
            _ => from_ansi_list(&[
                53, 54, 55, 134, 177, 219, 214, 220, 227, 229, 87, 123, 159, 195, 231,
            ]),
        },
        ColorScheme::Gray => match mode {
            ColorMode::Mono => vec![Color::White],
            ColorMode::Color16 => vec![Color::DarkGrey, Color::Grey, Color::White],
            _ => from_ansi_list(&[234, 237, 240, 243, 246, 249, 251, 252, 231]),
        },
        ColorScheme::Rainbow => match mode {
            ColorMode::Mono => vec![Color::White],
            ColorMode::Color16 => vec![
                Color::Red,
                Color::Blue,
                Color::Yellow,
                Color::Green,
                Color::Cyan,
                Color::Magenta,
            ],
            _ => from_ansi_list(&[196, 208, 226, 46, 21, 93, 201]),
        },
        ColorScheme::Snow => match mode {
            ColorMode::Mono => vec![Color::White],
            ColorMode::Color16 => vec![Color::DarkGrey, Color::Grey, Color::White, Color::Cyan],
            _ => from_ansi_list(&[234, 240, 250, 252, 231, 117, 159]),
        },
        ColorScheme::Aurora => match mode {
            ColorMode::Mono => vec![Color::White],
            ColorMode::Color16 => vec![Color::DarkGreen, Color::Green, Color::Cyan, Color::Magenta],
            _ => from_ansi_list(&[22, 28, 34, 40, 45, 51, 93, 129, 159]),
        },
        ColorScheme::FancyDiamond => match mode {
            ColorMode::Mono => vec![Color::White],
            ColorMode::Color16 => vec![Color::Cyan, Color::White, Color::Magenta],
            _ => from_ansi_list(&[45, 51, 87, 123, 159, 195, 231, 225]),
        },
        ColorScheme::Cosmos => match mode {
            ColorMode::Mono => vec![Color::White],
            ColorMode::Color16 => vec![Color::DarkBlue, Color::Blue, Color::Magenta, Color::White],
            _ => from_ansi_list(&[17, 18, 19, 54, 55, 56, 57, 93, 129, 189, 225]),
        },
        ColorScheme::Nebula => match mode {
            ColorMode::Mono => vec![Color::White],
            ColorMode::Color16 => vec![Color::Magenta, Color::Red, Color::Blue, Color::White],
            _ => from_ansi_list(&[53, 54, 90, 126, 162, 198, 201, 207, 213, 219, 225]),
        },
    };

    if default_background {
        bg = None;
    }

    Palette { colors, bg }
}
