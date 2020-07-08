use crate::nvidia;
use std::process::Command;

pub fn run(cmd_and_args: &[String]) -> i32 {
    info_print!("Starting program");

    let command = Command::new("primusrun")
        .args(cmd_and_args)
        .spawn()
        .map_err(|_| {
            error_print!("Failed to start the command!");
        });

    let code = match command.unwrap().wait() {
        Ok(status) => status.code().unwrap(),

        Err(_) => {
            error_print!("Program was terminated by signal!");
            1
        }
    };

    info_print!("Finished program");

    if nvidia::state().is_on() {
        nvidia::set_state(nvidia::State::Off);
    }

    code
}
