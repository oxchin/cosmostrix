// Copyright (c) 2025 rezk_nightky

use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;

#[derive(Clone, Copy, Debug)]
pub struct U16Range {
    pub low: u16,
    pub high: u16,
}

impl FromStr for U16Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split_once(',')
            .ok_or_else(|| "expected: NUM1,NUM2".to_string())?;
        let low: u16 = a
            .trim()
            .parse()
            .map_err(|_| "invalid low value".to_string())?;
        let high: u16 = b
            .trim()
            .parse()
            .map_err(|_| "invalid high value".to_string())?;
        if low == 0 || high == 0 || low > high {
            return Err("range must be >0 and low <= high".to_string());
        }
        Ok(Self { low, high })
    }
}

#[derive(Parser, Debug, Clone)]
#[command(name = "cosmostrix")]
pub struct Args {
    #[arg(short = 'a', long = "async")]
    pub async_mode: bool,

    #[arg(short = 'b', long = "bold", default_value_t = 1)]
    pub bold: u8,

    #[arg(short = 'C', long = "colorfile")]
    pub colorfile: Option<PathBuf>,

    #[arg(short = 'c', long = "color", default_value = "green")]
    pub color: String,

    #[arg(short = 'D', long = "defaultbg")]
    pub defaultbg: bool,

    #[arg(short = 'd', long = "density", default_value_t = 1.0)]
    pub density: f32,

    #[arg(short = 'F', long = "fullwidth")]
    pub fullwidth: bool,

    #[arg(short = 'f', long = "fps", default_value_t = 60.0)]
    pub fps: f64,

    #[arg(long = "duration")]
    pub duration: Option<f64>,

    #[arg(short = 'g', long = "glitchms", default_value = "300,400")]
    pub glitch_ms: U16Range,

    #[arg(short = 'G', long = "glitchpct", default_value_t = 10.0)]
    pub glitch_pct: f32,

    #[arg(short = 'l', long = "lingerms", default_value = "1,3000")]
    pub linger_ms: U16Range,

    #[arg(short = 'M', long = "shadingmode", default_value_t = 0)]
    pub shading_mode: u8,

    #[arg(short = 'm', long = "message")]
    pub message: Option<String>,

    #[arg(long = "maxdpc", default_value_t = 3)]
    pub max_droplets_per_column: u8,

    #[arg(long = "noglitch")]
    pub noglitch: bool,

    #[arg(short = 'r', long = "rippct", default_value_t = 33.33333)]
    pub rippct: f32,

    #[arg(short = 'S', long = "speed", default_value_t = 8.0)]
    pub speed: f32,

    #[arg(short = 's', long = "screensaver")]
    pub screensaver: bool,

    #[arg(long = "shortpct", default_value_t = 50.0)]
    pub shortpct: f32,

    #[arg(long = "charset", default_value = "binary")]
    pub charset: String,

    #[arg(long = "chars")]
    pub chars: Option<String>,

    #[arg(long = "colormode")]
    pub colormode: Option<u16>,

    #[arg(long = "info")]
    pub info: bool,
}
