use crate::device::Device;
use std::process::Command;
use std::thread::JoinHandle;

pub struct Audio;

impl Audio {
    pub fn new() -> Self {
        Self {}
    }
}

impl Device for Audio {
    fn name(&self) -> String {
        "Audio".to_string()
    }

    fn level(&self) -> u64 {
        let cmd = Command::new("/usr/bin/amixer").
            arg("get").arg("Master").
            output().expect("amixer not installed");
        let output = std::str::from_utf8(&cmd.stdout).unwrap();
        let a = match output.find('[') {
            None => panic!("could not parse amixer output"),
            Some(i) => i + 1,
        };
        let b = match output.find('%') {
            None => panic!("could not parse amixer output"),
            Some(i) => i,
        };
        output[a..b].parse().unwrap()
    }

    fn set_level(&self, level: u64) -> JoinHandle<()> {
        std::thread::spawn(move || {
            Command::new("/usr/bin/amixer").
                arg("set").arg("Master").arg(format!("{}%", level)).
                output().expect("amixer not installed");
        })
    }
}
