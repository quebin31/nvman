#[macro_use]
mod macros;
mod bumblebee;
mod nvidia;
mod optimus;
mod settings;
mod systemd;
mod utils;

use shells::bash;
use std::env;
use std::process;
use users::get_current_uid;

use crate::systemd::Service;

fn run_app() -> Result<(), ()> {
    let args: Vec<_> = env::args().collect();

    if get_current_uid() == 0 {
        if let Some(cmd) = args.get(1) {
            if cmd != "inspector" && cmd != "default" {
                error_print!("Only `inspector` and `default` commands can be run as root!");
                return Err(());
            }
        } else {
            error_print!("Don\'t run this script as root!");
            return Err(());
        }
    }

    utils::checks::check_nvman_service();
    if args.len() == 1 {
        utils::print::print_status();
        return Ok(());
    }

    match args[1].as_str() {
        "nvidia" => {
            if utils::checks::check_optimus_mode().is_bad() {
                return Err(());
            }

            if let Some(arg) = args.get(2) {
                match arg.as_str() {
                    "on" => nvidia::set_state(nvidia::State::On),
                    "off" => nvidia::set_state(nvidia::State::Off),
                    "auto" => nvidia::toggle_state(),
                    invalid => {
                        error_print!("Invalid value `{}`", invalid);
                        utils::print::print_usage_nvidia();
                        return Err(());
                    }
                }
            } else {
                error_print!("Please provide a value!");
                utils::print::print_usage_nvidia();
                return Err(());
            }
        }

        "run" => {
            if utils::checks::check_optimus_mode().is_bad() {
                return Err(());
            }

            if args.get(2).is_some() {
                let optimus = Service::new("optimus-manager");
                let bumblebee = Service::new("bumblebeed");

                optimus.stop();
                bumblebee.start();

                bumblebee::run(&args[2..]);

                bumblebee.stop();
                optimus.start();
            } else {
                error_print!("Please provide a command to run!");
                utils::print::print_usage_run();
                return Err(());
            }
        }

        "switch" => {
            if let Some(arg) = args.get(2) {
                if utils::checks::check_gdm_prime().is_bad() {
                    return Err(());
                }

                match arg.as_str() {
                    "nvidia" => {
                        info_print!("If your screen keeps black, switch across ttys");
                        bash!("sleep 1");
                        optimus::set_mode(optimus::Mode::Nvidia)
                    }

                    "intel" => {
                        info_print!("If your screen keeps black, switch across ttys");
                        bash!("sleep 1");
                        optimus::set_mode(optimus::Mode::Intel)
                    }

                    "auto" => {
                        info_print!("If your screen keeps black, switch across ttys");
                        bash!("sleep 1");
                        optimus::toggle_mode()
                    }

                    invalid => {
                        error_print!("Invalid value `{}`", invalid);
                        utils::print::print_usage_switch();
                        return Err(());
                    }
                }
            } else {
                error_print!("Please provide a value!");
                utils::print::print_usage_switch();
                return Err(());
            }
        }

        "startup" => {
            if let Some(arg) = args.get(2) {
                match arg.as_str() {
                    "nvidia" => optimus::set_startup(optimus::Mode::Nvidia),
                    "intel" => optimus::set_startup(optimus::Mode::Intel),
                    invalid => {
                        error_print!("Invalid value `{}`", invalid);
                        utils::print::print_usage_startup();
                        return Err(());
                    }
                }
            } else {
                error_print!("Please provide a value!");
                utils::print::print_usage_startup();
                return Err(());
            }
        }

        "default" => {
            if let Some(arg) = args.get(2) {
                if get_current_uid() != 0 {
                    let (code, stdout, _) =
                        sudo_bash!("{} default {}", std::env::args().next().unwrap(), arg);

                    print!("{}", stdout);
                    if code != 0 {
                        return Err(());
                    } else {
                        return Ok(());
                    }
                }

                match arg.as_str() {
                    "optimus" => {
                        info_print!("Setting {} as default service", arg);
                        settings::set_default_service("optimus")
                    }

                    "bumblebee" => {
                        info_print!("Setting {} as default service", arg);
                        settings::set_default_service("bumblebee")
                    }

                    invalid => {
                        error_print!("Invalid value `{}`", invalid);
                        utils::print::print_usage_default();
                        return Err(());
                    }
                }
            } else {
                error_print!("Please provide a value!");
                utils::print::print_usage_default();
                return Err(());
            }
        }

        "use" => {
            if let Some(arg) = args.get(2) {
                let optimus = Service::new("optimus-manager");
                let bumblebee = Service::new("bumblebeed");

                match arg.as_str() {
                    "optimus" => {
                        bumblebee.stop();
                        optimus.start();
                    }

                    "bumblebee" => {
                        optimus.stop();
                        bumblebee.start();
                    }

                    invalid => {
                        error_print!("Invalid value `{}`", invalid);
                        utils::print::print_usage_use();
                        return Err(());
                    }
                }
            } else {
                error_print!("Please provide a value!");
                utils::print::print_usage_use();
                return Err(());
            }
        }

        "stop" => {
            if let Some(arg) = args.get(2) {
                let optimus = Service::new("optimus-manager");
                let bumblebee = Service::new("bumblebeed");

                match arg.as_str() {
                    "optimus" => optimus.stop(),
                    "bumblebee" => bumblebee.stop(),
                    invalid => {
                        error_print!("Invalid value `{}`", invalid);
                        utils::print::print_usage_stop();
                        return Err(());
                    }
                };
            } else {
                error_print!("Please provide a value!");
                utils::print::print_usage_stop();
                return Err(());
            }
        }

        "inspector" => {
            let (default, non_default) = match settings::get_default_service().as_str() {
                "optimus" => (Service::new("optimus-manager"), Service::new("bumblebeed")),
                "bumblebee" => (Service::new("bumblebeed"), Service::new("optimus-manager")),
                _ => unreachable!(),
            };

            if non_default.enabled() {
                non_default.disable();
            }

            if !default.enabled() {
                default.enable();
            }
        }

        "status" => {
            utils::print::print_status();
        }

        "help" => {
            utils::print::print_help();
        }

        invalid_command => {
            error_print!("Invalid command `{}`", invalid_command);
            utils::print::print_help();
        }
    }

    Ok(())
}

fn main() -> Result<(), ()> {
    process::exit(match run_app() {
        Ok(_) => 0,
        Err(_) => 1,
    });
}
