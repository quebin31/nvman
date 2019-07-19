use std::fmt;
use shells::bash;
use crate::systemd::{
    Status,
    Service,
    ServiceStatus,
};

use crate::utils::checks;

/// Possible optimus modes: intel, nvidia
#[derive(Debug)]
pub enum Mode {
    Intel,
    Nvidia,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mode = match self {
            Mode::Intel => "intel",
            Mode::Nvidia => "nvidia",
        };

        write!(f, "{}", mode)
    }
}

/// Get the current mode using optimus-manager
pub fn get_mode() -> Mode {
    let (_code, stdout, _) = bash!("optimus-manager --print-mode");

    if stdout.contains("nvidia") {
        Mode::Nvidia
    } else {
        Mode::Intel
    }
}

/// Set (switch) the current mode using optimus-manager
pub fn set_mode(mode: Mode) {
    let optimus = Service::new("optimus-manager");
    let bumblebee = Service::new("bumblebeed");

    if checks::check_gdm_prime() == checks::GdmPrimeFlag::Bad {
        return;
    }

    info_print!("Switching to {}", mode.to_string());

    bumblebee.stop();
    optimus.start();

    bash!("optimus-manager --switch {} --no-confirm", mode.to_string());
}

/// Get the current startup using optimus-manager
pub fn get_startup() -> Mode {
    let (_code, stdout, _) = bash!("optimus-manager --print-startup");

    if stdout.contains("nvidia") {
        Mode::Nvidia
    } else {
        Mode::Intel
    }
}

/// Set the default startup using optimus-manager
pub fn set_startup(startup: Mode) {
    let optimus = Service::new("optimus-manager");
    let bumblebee = Service::new("bumblebeed");

    info_print!("Setting default startup to {}", startup.to_string());

    let ServiceStatus(opt_active, _) = optimus.status();
    optimus.start();

    bash!("optimus-manager --set-startup {}", startup.to_string());
    if opt_active == Status::Inactive {
        optimus.stop();
    }
}
