use crate::device::Device;
use std::process::Command;
use std::thread;

pub struct Screen;

impl Screen {
    pub fn new() -> Self {
        Self {}
    }
}

impl Device for Screen {
    fn name(&self) -> String {
        "Screen".to_string()
    }

    fn level(&self) -> u64 {
        let cmd = Command::new("/usr/bin/xbacklight").
            arg("-get").
            output().expect("backlight not installed");
        let output = std::str::from_utf8(&cmd.stdout).unwrap().trim();
        let n = match output.find('.') {
            None => output.len(),
            Some(i) => i,
        };
        output[0..n].parse().unwrap()
    }

    fn set_level(&self, level: u64) {
        thread::spawn(move || {
            Command::new("/usr/bin/xbacklight").
                arg("-set").arg(level.to_string()).
                output().expect("backlight not installed");
        });
    }
}
