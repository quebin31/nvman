use std::process::Command;
use crate::nvidia;
use crate::systemd::Service;

pub fn run(cmd: &str, args: &[String]) -> i32 {
    info_print!("Starting {}", cmd);

    let command = Command::new(cmd)
        .args(args)
        .spawn()
        .map_err(|_| {
            error_print!("Failed to start the command!");
        });

    let code = match command.unwrap().wait() {
        Ok(status) => {
            status.code().unwrap()
        }

        Err(_) => {
            error_print!("Program was terminated by signal!");
            1
        }
    };

    info_print!("Finished {}", cmd);

    if nvidia::state().is_on() {
        nvidia::set_state(nvidia::State::Off);
    }

    return code;
}