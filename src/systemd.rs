use ansi_term::Colour::{Green, Red, Yellow};
use shells::bash;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Status {
    Enabled,
    Disabled,
    Active,
    Inactive,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = match self {
            Status::Enabled => "(enabled)",
            Status::Disabled => "(disabled)",
            Status::Active => "active",
            Status::Inactive => "inactive",
        };

        write!(f, "{}", status)
    }
}

#[derive(Debug)]
pub struct ServiceStatus(pub Status, pub Status);

impl fmt::Display for ServiceStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let acolor = match self.0 {
            Status::Active => Green.normal(),
            Status::Inactive => Red.normal(),
            _ => unreachable!(),
        };

        let activeness = acolor.paint(format!("{:<8}", self.0.to_string()));
        let enableness = Yellow.paint(format!("{:<10}", self.1.to_string()));

        write!(f, "{} {}", activeness, enableness)
    }
}

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

    pub fn status(&self) -> ServiceStatus {
        let (_code, stdout, _) = bash!("/usr/bin/systemctl status {}", self.descriptor);

        let activeness = if stdout.contains("inactive") {
            Status::Inactive
        } else {
            Status::Active
        };

        let enableness = if stdout.contains("enabled;") {
            Status::Enabled
        } else {
            Status::Disabled
        };

        ServiceStatus(activeness, enableness)
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
