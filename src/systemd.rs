use std::fmt;

use shells::bash;
use ansi_term::Colour::{Green, Red, Yellow};

#[derive(Debug)]
pub struct Service {
    descriptor: String,
}

impl Service {
    pub fn new(descriptor: &str) -> Self {
        Self {
            descriptor: descriptor.to_owned(),
        }
    }

    pub fn enabled(&self) -> bool {
        let (_, stdout, _) = bash!("/usr/bin/systemctl status {}", self.descriptor);

        stdout.contains("enabled;")
    }

    pub fn active(&self) -> bool {
        let (_, stdout, _) = bash!("/usr/bin/systemctl status {}", self.descriptor);

        !stdout.contains("inactive")
    }

    pub fn enable(&self) -> bool {
        let (code, _, _) = sudo_bash!("/usr/bin/systemctl enable {}", self.descriptor);
        code == 0
    }

    pub fn disable(&self) -> bool {
        let (code, _, _) = sudo_bash!("/usr/bin/systemctl disable {}", self.descriptor);
        code == 0
    }

    pub fn start(&self) -> bool {
        let (code, _, _) = sudo_bash!("/usr/bin/systemctl start {}", self.descriptor);
        code == 0
    }

    pub fn stop(&self) -> bool {
        let (code, _, _) = sudo_bash!("/usr/bin/systemctl stop {}", self.descriptor);
        code == 0
    }
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let activeness = if self.active() {
            Green.paint(format!("{:<8}", "active"))
        } else {
            Red.paint(format!("{:<8}", "inactive"))
        };

        let enableness = if self.enabled() {
            Yellow.bold().paint(format!("{:<10}", "(enabled)"))
        } else {
            Yellow.paint(format!("{:<10}", "(disabled)"))
        };

        write!(f, "{} {}", activeness, enableness)
    }
}
