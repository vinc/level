mod audio;
mod device;
mod screen;

use crate::audio::Audio;
use crate::device::Device;
use crate::screen::Screen;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::Write;
use std::io;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion;

fn load_device(name: &str) -> Box<dyn Device> {
    match name {
        "a" | "audio" => Box::new(Audio::new()),
        "s" | "screen" => Box::new(Screen::new()),
        _ => panic!("wrong device"),
    }
}

fn load_bar(name: &str, level: u64) -> ProgressBar {
    let bar = ProgressBar::new(100);
    bar.set_style(
        ProgressStyle::default_bar().
        template("{msg} [{bar:40.green/white}] {pos:>3}%").
        progress_chars("##-")
    );
    bar.set_draw_delta(1);
    bar.set_message(name);
    bar.set_position(level);
    bar
}

fn print_usage() {
    println!("Usage: level [<audio|screen>] [percent]");
}

fn main() {
    let mut show_usage = false;
    let args: Vec<String> = std::env::args().filter(|arg| {
        if arg == "--help" {
            show_usage = true;
        }
        !arg.starts_with("--")
    }).collect();
    if show_usage || !(1..4).contains(&args.len()) {
        return print_usage();
    }

    let mut device = load_device(if args.len() == 1 { "audio" } else { &args[1] });

    if args.len() == 3 {
        match args[2].parse() {
            Ok(level) => {
                if level <= 100 {
                    device.set_level(level).join().expect("Error while setting level");
                } else {
                    println!("Could not set {} level to {}%", device.name().to_lowercase(), level);
                }
            },
            Err(_) => {
                println!("Could not parse {} level", device.name().to_lowercase());
            }
        }
    } else {
        let mut level = device.level();
        let stdin = io::stdin();
        let mut stdout = io::stdout().into_raw_mode().unwrap();
        write!(stdout, "{}", termion::cursor::Hide).unwrap();
        stdout.lock().flush().unwrap();

        let bar = load_bar(&device.name(), level);

        for key in stdin.keys() {
            level = match key.unwrap() {
                Key::Char('q') | Key::Ctrl('c') | Key::Esc => {
                    break;
                },
                Key::Char('^') => {
                    0
                },
                Key::Char('$') => {
                    100
                },
                Key::Char(' ') => {
                    let device_name = if device.name() == "Audio" { "screen" } else { "audio" };
                    device = load_device(device_name);
                    bar.set_message(&device.name());
                    device.level()
                },
                Key::Left | Key::Down => {
                    level - std::cmp::min(level, 1)
                },
                Key::Right | Key::Up => {
                    level + 1
                },
                Key::PageDown => {
                    level - std::cmp::min(level, 10)
                },
                Key::PageUp => {
                    level + 10
                },
                _ => {
                    level
                },
            };
            if level > 100 {
                level = 100;
            }
            bar.set_position(level);
            device.set_level(level);
        }

        bar.abandon();
        write!(stdout, "{}", termion::cursor::Show).unwrap();
    }
}
