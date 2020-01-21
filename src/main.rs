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
        "Audio" | "audio" => Box::new(Audio::new()),
        "Screen" | "screen" => Box::new(Screen::new()),
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if !(2..4).contains(&args.len()) {
        println!("Usage: level <audio|screen> [percent]");
        return
    }

    let device = load_device(&args[1]);

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
            match key.unwrap() {
                Key::Char('q') | Key::Ctrl('c') | Key::Esc => {
                    break;
                },
                Key::Left | Key::Down => {
                    if level > 0 {
                        level -= 1;
                    }
                },
                Key::Right | Key::Up => {
                    if level < 100 {
                        level += 1;
                    }
                },
                _ => {},
            }
            bar.set_position(level);
            device.set_level(level);
        }

        bar.abandon();
        write!(stdout, "{}", termion::cursor::Show).unwrap();
    }
}
