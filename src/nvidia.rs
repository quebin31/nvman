use shells::bash;
use std::fmt;

/// Enumeration representing the two possible states of
/// NVIDIA GPU
#[derive(Debug, PartialEq)]
pub enum State {
    On,
    Off,
    Unknown,
}

impl State {
    pub fn is_on(&self) -> bool {
        *self == State::On
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let state = match self {
            State::On => "on",
            State::Off => "off",
            State::Unknown => "unknown",
        };

        write!(f, "{}", state)
    }
}

/// Returns the current state of the NVIDIA GPU
pub fn state() -> State {
    let (_code, stdout, _) = bash!("/usr/bin/cat /proc/acpi/bbswitch | awk \'{{ printf $2 }}\'");

    match stdout.as_str() {
        "ON" => State::On,
        "OFF" => State::Off,
        _ => State::Unknown,
    }
}

/// Manually set the state of NVIDIA GPU
pub fn set_state(state: State) {
    info_print!("Turning {} the NVIDIA GPU", state);

    match state {
        State::On => {
            sudo_bash!("/usr/bin/tee /proc/acpi/bbswitch <<< ON");
        }

        State::Off => {
            sudo_bash!("/usr/bin/rmmod nvidia_uvm");
            sudo_bash!("/usr/bin/rmmod nvidia_modeset");
            sudo_bash!("/usr/bin/rmmod nvidia");
            sudo_bash!("/usr/bin/tee /proc/acpi/bbswitch <<< OFF");
        }

        State::Unknown => unreachable!(),
    }
}

/// Automatically toggle the state of NVIDIA GPU
pub fn toggle_state() {
    match state() {
        State::On => self::set_state(State::Off),
        State::Off => self::set_state(State::On),
        State::Unknown => (),
    }
}
