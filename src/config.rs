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
    #[arg(
        short = 'a',
        long = "async",
        help_heading = "General",
        help = "Enable async rendering"
    )]
    pub async_mode: bool,

    #[arg(
        short = 'b',
        long = "bold",
        default_value_t = 1,
        help_heading = "Appearance",
        help = "Bold mode: 0=off, 1=random, 2=all"
    )]
    pub bold: u8,

    #[arg(
        short = 'C',
        long = "colorfile",
        help_heading = "Appearance",
        help = "Load user palette from a file"
    )]
    pub colorfile: Option<PathBuf>,

    #[arg(
        short = 'c',
        long = "color",
        default_value = "green",
        help_heading = "Appearance",
        help = "Color theme (see --list-colors)"
    )]
    pub color: String,

    #[arg(
        short = 'D',
        long = "defaultbg",
        help_heading = "Appearance",
        help = "Use terminal default background"
    )]
    pub defaultbg: bool,

    #[arg(
        short = 'd',
        long = "density",
        default_value_t = 1.0,
        help_heading = "Performance",
        help = "Droplet density (higher = more rain)"
    )]
    pub density: f32,

    #[arg(
        short = 'F',
        long = "fullwidth",
        help_heading = "General",
        help = "Use full terminal width"
    )]
    pub fullwidth: bool,

    #[arg(
        short = 'f',
        long = "fps",
        default_value_t = 60.0,
        help_heading = "Performance",
        help = "Target FPS"
    )]
    pub fps: f64,

    #[arg(
        long = "duration",
        help_heading = "General",
        help = "Stop after N seconds"
    )]
    pub duration: Option<f64>,

    #[arg(
        short = 'g',
        long = "glitchms",
        default_value = "300,400",
        help_heading = "Glitch (Advanced)",
        help = "Glitch duration range in ms: LOW,HIGH"
    )]
    pub glitch_ms: U16Range,

    #[arg(
        short = 'G',
        long = "glitchpct",
        default_value_t = 10.0,
        help_heading = "Glitch (Advanced)",
        help = "Glitch chance in percent (0-100)"
    )]
    pub glitch_pct: f32,

    #[arg(
        short = 'l',
        long = "lingerms",
        default_value = "1,3000",
        help_heading = "Glitch (Advanced)",
        help = "Linger time range in ms: LOW,HIGH"
    )]
    pub linger_ms: U16Range,

    #[arg(
        short = 'M',
        long = "shadingmode",
        default_value_t = 0,
        help_heading = "Appearance",
        help = "Shading mode: 0=random, 1=distance-from-head"
    )]
    pub shading_mode: u8,

    #[arg(
        short = 'm',
        long = "message",
        help_heading = "General",
        help = "Overlay message"
    )]
    pub message: Option<String>,

    #[arg(
        long = "message-no-border",
        alias = "mB",
        help_heading = "General",
        help = "Draw message box without border (use with --message)"
    )]
    pub message_no_border: bool,

    #[arg(
        long = "maxdpc",
        default_value_t = 3,
        help_heading = "Performance",
        help = "Max droplets per column (1-3)"
    )]
    pub max_droplets_per_column: u8,

    #[arg(
        long = "noglitch",
        help_heading = "Glitch (Advanced)",
        help = "Disable glitch effects"
    )]
    pub noglitch: bool,

    #[arg(
        short = 'r',
        long = "rippct",
        default_value_t = 33.33333,
        help_heading = "Glitch (Advanced)",
        help = "Die-early chance in percent (0-100)"
    )]
    pub rippct: f32,

    #[arg(
        short = 'S',
        long = "speed",
        default_value_t = 8.0,
        help_heading = "Performance",
        help = "Characters per second (rain speed)"
    )]
    pub speed: f32,

    #[arg(
        short = 's',
        long = "screensaver",
        help_heading = "General",
        help = "Screensaver mode (exit on keypress)"
    )]
    pub screensaver: bool,

    #[arg(
        long = "shortpct",
        default_value_t = 50.0,
        help_heading = "Glitch (Advanced)",
        help = "Chance for short droplets in percent (0-100)"
    )]
    pub shortpct: f32,

    #[arg(
        long = "charset",
        default_value = "binary",
        help_heading = "Charset",
        help = "Charset preset (see --list-charsets)"
    )]
    pub charset: String,

    #[arg(
        long = "chars",
        help_heading = "Charset",
        help = "Custom characters override"
    )]
    pub chars: Option<String>,

    #[arg(
        long = "colormode",
        help_heading = "Appearance",
        help = "Force color mode: 0,16,256,32(truecolor)"
    )]
    pub colormode: Option<u16>,

    #[arg(
        long = "help-detail",
        help_heading = "Help",
        help = "Show detailed help for all parameters and exit"
    )]
    pub help_detail: bool,

    #[arg(
        long = "list-charsets",
        help_heading = "Help",
        help = "List available charset presets and exit"
    )]
    pub list_charsets: bool,

    #[arg(
        long = "list-colors",
        help_heading = "Help",
        help = "List available color themes and exit"
    )]
    pub list_colors: bool,

    #[arg(
        long = "info",
        help_heading = "Help",
        help = "Print version info and exit"
    )]
    pub info: bool,
}

pub fn print_list_charsets() {
    println!(
        "AVAILABLE CHARSET PRESETS:\n\
  auto\n\
      Auto-select (ASCII_SAFE when non-UTF, otherwise matrix)\n\
  matrix\n\
      Letters + digits + katakana (no punctuation)\n\
  ascii\n\
      Letters + digits + punctuation\n\
  extended\n\
      Digits + punctuation + katakana\n\
  english\n\
      Letters only\n\
  digits | dec | decimal\n\
      Digits only\n\
  punc\n\
      Punctuation only\n\
  binary | bin | 01\n\
      0 and 1\n\
  hex | hexadecimal\n\
      0-9 and A-F\n\
  katakana\n\
  greek\n\
  cyrillic\n\
  hebrew\n\
  devanagari\n\
  braille\n\
  runic"
    );
}

pub fn print_list_colors() {
    println!(
        "AVAILABLE COLOR THEMES:\n\
  green\n\
  green2\n\
  green3\n\
  yellow\n\
  orange\n\
  red\n\
  blue\n\
  cyan\n\
  gold\n\
  rainbow\n\
  purple\n\
  pink\n\
  pink2\n\
  vaporwave\n\
  gray | grey\n\
  user\n\
      Use palette from --colorfile"
    );
}

pub fn print_help_detail() {
    println!(
        "cosmostrix --help-detail\n\nUSAGE:\n  cosmostrix [OPTIONS]\n\nGENERAL:\n  -a, --async\n      Enable async rendering.\n      Example: cosmostrix -a\n\n  -s, --screensaver\n      Screensaver mode (exit on keypress).\n      Example: cosmostrix -s\n\n  -F, --fullwidth\n      Use full terminal width.\n      Example: cosmostrix -F\n\n  --duration <seconds>\n      Stop after N seconds.\n      Example: cosmostrix --duration 10\n\n  -m, --message <text>\n      Overlay message.\n      Example: cosmostrix -m \"hello\"\n\nAPPEARANCE:\n  -c, --color <name>\n      Set theme (green, rainbow, vaporwave, user, ...).\n      Example: cosmostrix --color rainbow\n\n  -C, --colorfile <path>\n      Load user palette. When set, color scheme becomes 'user'.\n      Example: cosmostrix --colorfile ./colors.csv\n\n  --colormode <0|16|256|32>\n      Force color mode; otherwise auto-detected from TERM/COLORTERM.\n      Example: cosmostrix --colormode 256\n\n  -b, --bold <0|1|2>\n      Bold style (0 off, 1 random, 2 all).\n      Example: cosmostrix --bold 2\n\n  -M, --shadingmode <0|1>\n      Shading (0 random, 1 distance-from-head).\n      Example: cosmostrix -M 1\n\n  -D, --defaultbg\n      Use terminal default background.\n      Example: cosmostrix -D\n\nPERFORMANCE:\n  -f, --fps <number>\n      Target FPS.\n      Example: cosmostrix --fps 30\n\n  -S, --speed <number>\n      Characters per second (rain speed).\n      Example: cosmostrix --speed 12\n\n  -d, --density <number>\n      Droplet density.\n      Example: cosmostrix --density 1.25\n\n  --maxdpc <1..3>\n      Max droplets per column.\n      Example: cosmostrix --maxdpc 2\n\nCHARSET:\n  --charset <name>\n      Charset preset.\n      Example: cosmostrix --charset binary\n\n  --chars <string>\n      Custom character override (advanced).\n      Example: cosmostrix --chars \"01\"\n\nGLITCH (ADVANCED):\n  --noglitch\n      Disable glitch effects.\n      Example: cosmostrix --noglitch\n\n  -G, --glitchpct <0..100>\n      Glitch chance in percent.\n      Example: cosmostrix --glitchpct 5\n\n  -g, --glitchms <low,high>\n      Glitch duration range in ms.\n      Example: cosmostrix --glitchms 200,500\n\n  -l, --lingerms <low,high>\n      Linger duration range in ms.\n      Example: cosmostrix --lingerms 1,3000\n\n  --shortpct <0..100>\n      Short droplet chance in percent.\n      Example: cosmostrix --shortpct 40\n\n  -r, --rippct <0..100>\n      Die-early chance in percent.\n      Example: cosmostrix --rippct 20\n\nHELP:\n  --help\n      Show short help.\n\n  --help-detail\n      Show this detailed help.\n\n  --info\n      Print build info.\n"
    );

    println!("\nVALUE LISTS:\n  cosmostrix --list-charsets\n  cosmostrix --list-colors");
    println!("\nMESSAGE BOX:\n  --message-no-border, --mB\n      Draw filled box without border characters");
    println!();
    print_list_charsets();
    println!();
    print_list_colors();
}
