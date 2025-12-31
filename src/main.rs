// Copyright (c) 2025 rezk_nightky

mod cell;
mod charset;
mod cloud;
mod config;
mod droplet;
mod frame;
mod palette;
mod runtime;
mod terminal;

use std::env;
use std::time::{Duration, Instant};

use clap::builder::styling::{AnsiColor as ClapAnsiColor, Color as ClapColor};
use clap::builder::styling::{Effects as ClapEffects, Style as ClapStyle};
use clap::builder::Styles as ClapStyles;
use clap::{CommandFactory, FromArgMatches};
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};

use crate::charset::{build_chars, charset_from_str, parse_user_hex_chars};
use crate::cloud::Cloud;
use crate::config::{
    color_enabled_stdout, default_params_usage_for_help, print_help_detail, print_list_charsets,
    print_list_colors, Args,
};
use crate::frame::Frame;
use crate::runtime::{BoldMode, ColorMode, ColorScheme, ShadingMode};
use crate::terminal::Terminal;

const HELP_TEMPLATE_PLAIN: &str = "\
{before-help}{about-with-newline}
USAGE:
  {usage}

{all-args}{after-help}";

const HELP_TEMPLATE_COLOR: &str = "\
{before-help}{about-with-newline}
\x1b[1;36mUSAGE:\x1b[0m
  {usage}

{all-args}{after-help}";

fn clap_styles() -> ClapStyles {
    ClapStyles::styled()
        .header(
            ClapStyle::new()
                .effects(ClapEffects::BOLD)
                .fg_color(Some(ClapColor::Ansi(ClapAnsiColor::Cyan))),
        )
        .usage(
            ClapStyle::new()
                .effects(ClapEffects::BOLD)
                .fg_color(Some(ClapColor::Ansi(ClapAnsiColor::Green))),
        )
        .literal(ClapStyle::new().fg_color(Some(ClapColor::Ansi(ClapAnsiColor::Yellow))))
        .placeholder(ClapStyle::new().fg_color(Some(ClapColor::Ansi(ClapAnsiColor::Magenta))))
}

fn clamp_f64(v: f64, min: f64, max: f64, fallback: f64) -> f64 {
    if !v.is_finite() {
        return fallback;
    }
    v.clamp(min, max)
}

fn clamp_f32(v: f32, min: f32, max: f32, fallback: f32) -> f32 {
    if !v.is_finite() {
        return fallback;
    }
    v.clamp(min, max)
}

fn default_to_ascii() -> bool {
    let lang = env::var("LANG").unwrap_or_default();
    !lang.to_ascii_uppercase().contains("UTF")
}

fn detect_color_mode(args: &Args) -> ColorMode {
    if let Some(m) = args.colormode {
        return match m {
            0 => ColorMode::Mono,
            16 => ColorMode::Color16,
            32 => ColorMode::TrueColor,
            256 => ColorMode::Color256,
            _ => {
                eprintln!("invalid --colormode: {} (allowed: 0,16,256,32)", m);
                std::process::exit(1);
            }
        };
    }

    let colorterm = env::var("COLORTERM")
        .unwrap_or_default()
        .to_ascii_lowercase();
    if colorterm.contains("truecolor") || colorterm.contains("24bit") {
        return ColorMode::TrueColor;
    }

    let term = env::var("TERM").unwrap_or_default().to_ascii_lowercase();
    if term.contains("256color") {
        return ColorMode::Color256;
    }

    ColorMode::Color16
}

fn parse_color_scheme(s: &str) -> Result<ColorScheme, String> {
    match s.trim().to_ascii_lowercase().as_str() {
        "green" => Ok(ColorScheme::Green),
        "green2" => Ok(ColorScheme::Green2),
        "green3" => Ok(ColorScheme::Green3),
        "yellow" => Ok(ColorScheme::Yellow),
        "orange" => Ok(ColorScheme::Orange),
        "red" => Ok(ColorScheme::Red),
        "blue" => Ok(ColorScheme::Blue),
        "cyan" => Ok(ColorScheme::Cyan),
        "gold" => Ok(ColorScheme::Gold),
        "rainbow" => Ok(ColorScheme::Rainbow),
        "purple" => Ok(ColorScheme::Purple),
        "neon" | "synthwave" => Ok(ColorScheme::Neon),
        "fire" | "inferno" => Ok(ColorScheme::Fire),
        "ocean" | "deep-sea" | "deep_sea" | "deepsea" => Ok(ColorScheme::Ocean),
        "forest" | "jungle" => Ok(ColorScheme::Forest),
        "vaporwave" => Ok(ColorScheme::Vaporwave),
        "gray" | "grey" => Ok(ColorScheme::Gray),
        "snow" => Ok(ColorScheme::Snow),
        "aurora" => Ok(ColorScheme::Aurora),
        "fancy-diamond" | "fancy_diamond" | "fancydiamond" => Ok(ColorScheme::FancyDiamond),
        "cosmos" => Ok(ColorScheme::Cosmos),
        "nebula" => Ok(ColorScheme::Nebula),
        _ => Err(format!("invalid color: {} (see --list-colors)", s)),
    }
}

fn main() -> std::io::Result<()> {
    let mut cmd = Args::command();
    cmd = cmd.styles(clap_styles());
    cmd = cmd.before_help(default_params_usage_for_help());
    let help_template = if color_enabled_stdout() {
        HELP_TEMPLATE_COLOR
    } else {
        HELP_TEMPLATE_PLAIN
    };
    cmd = cmd.help_template(help_template);
    cmd.build();

    if cmd.get_arguments().any(|a| a.get_id().as_str() == "help") {
        cmd = cmd.mut_arg("help", |a| a.help_heading("HELP"));
    }
    cmd.build();
    let matches = cmd.get_matches();
    let args = Args::from_arg_matches(&matches).unwrap_or_else(|e| e.exit());

    if args.list_charsets {
        print_list_charsets();
        return Ok(());
    }

    if args.list_colors {
        print_list_colors();
        return Ok(());
    }

    if args.help_detail {
        print_help_detail();
        return Ok(());
    }

    if args.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if args.info {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        println!("author: {}", env!("CARGO_PKG_AUTHORS"));
        println!("{}", env!("CARGO_PKG_DESCRIPTION"));
        return Ok(());
    }

    let def_ascii = default_to_ascii();
    let color_mode = detect_color_mode(&args);

    let shading_mode = match args.shading_mode {
        1 => ShadingMode::DistanceFromHead,
        _ => ShadingMode::Random,
    };

    let bold_mode = match args.bold.min(2) {
        0 => BoldMode::Off,
        2 => BoldMode::All,
        _ => BoldMode::Random,
    };

    let color_scheme = match parse_color_scheme(&args.color) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let mut term = Terminal::new()?;
    let (w, h) = term.size()?;

    let mut cloud = Cloud::new(
        color_mode,
        args.fullwidth,
        shading_mode,
        bold_mode,
        args.async_mode,
        args.defaultbg,
        color_scheme,
    );

    cloud.glitchy = !args.noglitch;
    cloud.set_glitch_pct(clamp_f32(args.glitch_pct, 0.0, 100.0, 10.0) / 100.0);

    let glitch_low = args.glitch_ms.low.clamp(1, 5000);
    let glitch_high = args.glitch_ms.high.clamp(1, 5000);
    cloud.set_glitch_times(glitch_low, glitch_high);

    let linger_low = args.linger_ms.low.clamp(1, 60000);
    let linger_high = args.linger_ms.high.clamp(1, 60000);
    cloud.set_linger_times(linger_low, linger_high);

    cloud.short_pct = clamp_f32(args.shortpct, 0.0, 100.0, 50.0) / 100.0;
    cloud.die_early_pct = clamp_f32(args.rippct, 0.0, 100.0, 33.33333) / 100.0;
    cloud.set_max_droplets_per_column(args.max_droplets_per_column.clamp(1, 3));

    cloud.set_droplet_density(clamp_f32(args.density, 0.01, 5.0, 1.0));
    cloud.set_chars_per_sec(clamp_f32(args.speed, 0.001, 1000.0, 8.0));

    let mut user_ranges: Vec<(char, char)> = Vec::new();
    if let Some(spec) = &args.chars {
        match parse_user_hex_chars(spec) {
            Ok(list) => {
                if list.len() % 2 != 0 {
                    eprintln!("--chars: odd number of unicode chars given (must be even)");
                    std::process::exit(1);
                }
                for pair in list.chunks(2) {
                    let a = pair[0];
                    let b = pair[1];
                    user_ranges.push((a, b));
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }

    let charset = match charset_from_str(&args.charset, def_ascii) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let chars = build_chars(charset, &user_ranges, def_ascii);
    cloud.init_chars(chars);
    cloud.reset(w, h);

    if let Some(msg) = &args.message {
        cloud.set_message_border(!args.message_no_border);
        cloud.set_message(msg);
    }

    let mut frame = Frame::new(w, h, cloud.palette.bg);

    let start_time = Instant::now();
    let end_time = args.duration.and_then(|s| {
        if !s.is_finite() || s <= 0.0 {
            return None;
        }
        let s = s.clamp(0.1, 86400.0);
        Some(start_time + Duration::from_secs_f64(s))
    });

    let target_fps = clamp_f64(args.fps, 1.0, 240.0, 60.0);
    let target_period = Duration::from_secs_f64(1.0 / target_fps);
    let mut next_frame = Instant::now();

    while cloud.raining {
        if end_time.is_some_and(|end| Instant::now() >= end) {
            cloud.raining = false;
            break;
        }
        let mut pending_resize: Option<(u16, u16)> = None;

        loop {
            while Terminal::poll_event(Duration::from_millis(0))? {
                let ev = Terminal::read_event()?;
                match ev {
                    Event::Resize(nw, nh) => {
                        pending_resize = Some((nw, nh));
                    }
                    Event::Key(k) if k.kind == KeyEventKind::Press => {
                        if args.screensaver {
                            cloud.raining = false;
                            break;
                        }

                        match (k.code, k.modifiers) {
                            (KeyCode::Esc, _) => cloud.raining = false,
                            (KeyCode::Char('q'), _) => cloud.raining = false,
                            (KeyCode::Char(' '), _) => {
                                cloud.reset(frame.width, frame.height);
                                cloud.force_draw_everything();
                            }
                            (KeyCode::Char('a'), _) => {
                                cloud.set_async(!cloud.async_mode);
                            }
                            (KeyCode::Char('p'), _) => {
                                cloud.toggle_pause();
                            }
                            (KeyCode::Up, _) => {
                                let mut cps = cloud.chars_per_sec;
                                if cps <= 0.5 {
                                    cps *= 2.0;
                                } else {
                                    cps += 1.0;
                                }
                                cloud.set_chars_per_sec(cps.min(1000.0));
                            }
                            (KeyCode::Down, _) => {
                                let mut cps = cloud.chars_per_sec;
                                if cps <= 1.0 {
                                    cps /= 2.0;
                                } else {
                                    cps -= 1.0;
                                }
                                cloud.set_chars_per_sec(cps.max(0.001));
                            }
                            (KeyCode::Left, _) => {
                                if cloud.glitchy {
                                    let gp = (cloud.glitch_pct - 0.05).max(0.0);
                                    cloud.set_glitch_pct(gp);
                                }
                            }
                            (KeyCode::Right, _) => {
                                if cloud.glitchy {
                                    let gp = (cloud.glitch_pct + 0.05).min(1.0);
                                    cloud.set_glitch_pct(gp);
                                }
                            }
                            (KeyCode::Tab, _) => {
                                let sm = if cloud.shading_distance {
                                    ShadingMode::Random
                                } else {
                                    ShadingMode::DistanceFromHead
                                };
                                cloud.set_shading_mode(sm);
                            }
                            (KeyCode::Char('-'), _) => {
                                let d = (cloud.droplet_density - 0.25).max(0.01);
                                cloud.set_droplet_density(d);
                            }
                            (KeyCode::Char('+'), _) | (KeyCode::Char('='), KeyModifiers::SHIFT) => {
                                let d = (cloud.droplet_density + 0.25).min(5.0);
                                cloud.set_droplet_density(d);
                            }
                            (KeyCode::Char('1'), _) => cloud.set_color_scheme(ColorScheme::Green),
                            (KeyCode::Char('2'), _) => cloud.set_color_scheme(ColorScheme::Green2),
                            (KeyCode::Char('3'), _) => cloud.set_color_scheme(ColorScheme::Green3),
                            (KeyCode::Char('4'), _) => cloud.set_color_scheme(ColorScheme::Gold),
                            (KeyCode::Char('5'), _) => cloud.set_color_scheme(ColorScheme::Neon),
                            (KeyCode::Char('6'), _) => cloud.set_color_scheme(ColorScheme::Red),
                            (KeyCode::Char('7'), _) => cloud.set_color_scheme(ColorScheme::Blue),
                            (KeyCode::Char('8'), _) => cloud.set_color_scheme(ColorScheme::Cyan),
                            (KeyCode::Char('9'), _) => cloud.set_color_scheme(ColorScheme::Purple),
                            (KeyCode::Char('0'), _) => cloud.set_color_scheme(ColorScheme::Gray),
                            (KeyCode::Char('!'), _) => cloud.set_color_scheme(ColorScheme::Rainbow),
                            (KeyCode::Char('@'), _) => cloud.set_color_scheme(ColorScheme::Yellow),
                            (KeyCode::Char('#'), _) => cloud.set_color_scheme(ColorScheme::Orange),
                            (KeyCode::Char('$'), _) => cloud.set_color_scheme(ColorScheme::Fire),
                            (KeyCode::Char('%'), _) => {
                                cloud.set_color_scheme(ColorScheme::Vaporwave)
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

            if !cloud.raining || pending_resize.is_some() {
                break;
            }

            let now = Instant::now();
            if now >= next_frame {
                break;
            }

            let mut timeout = next_frame - now;
            if let Some(end) = end_time {
                if now >= end {
                    break;
                }
                timeout = timeout.min(end - now);
            }
            let _ = Terminal::poll_event(timeout)?;
        }

        if !cloud.raining {
            break;
        }

        if let Some((nw, nh)) = pending_resize {
            cloud.reset(nw, nh);
            frame = Frame::new(nw, nh, cloud.palette.bg);
            cloud.force_draw_everything();
        }

        cloud.rain(&mut frame);
        if frame.is_dirty_all() || !frame.dirty_indices().is_empty() {
            term.draw(&mut frame)?;
        }

        next_frame += target_period;
        let now = Instant::now();
        if now > next_frame {
            next_frame = now;
        }
    }

    Ok(())
}
