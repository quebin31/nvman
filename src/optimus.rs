use crate::systemd::Service;
use shells::bash;
use std::fmt;

/// Possible optimus modes: intel, nvidia
#[derive(Debug, PartialEq)]
pub enum Mode {
    Intel,
    Nvidia,
}

impl Mode {
    pub fn is_nvidia(&self) -> bool {
        *self == Mode::Nvidia
    }
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

    info_print!("Switching to {}", mode.to_string());

    bumblebee.stop();
    optimus.start();

    bash!("optimus-manager --switch {} --no-confirm", mode.to_string());
}

pub fn toggle_mode() {
    let mode = get_mode();
    match mode {
        Mode::Intel => set_mode(Mode::Nvidia),
        Mode::Nvidia => set_mode(Mode::Intel),
    }
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
