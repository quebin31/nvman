use clap::clap_app;

#[macro_use]
mod macros;

mod nvidia;
mod systemd;
mod utils;

fn main() {
    let mut nvman = clap_app! { nvman =>
        (version: "2.0")
        (author: "Kevin D. <quebin31@gmail.com>")
        (about: "Manage your NVIDIA powered laptop, the right way")
        (@subcommand nvidia =>
            (about: "Turn on/off your NVIDIA GPU, useful for CUDA (values: on, off, auto)")
            (@subcommand on => 
             (about: "Turn on your NVIDIA GPU"))
            (@subcommand off => 
             (about: "Turn off your NVIDIA GPU"))
            (@subcommand auto => 
             (about: "Toggle the state of your NVIDIA GPU")))
            //(@arg state: +required "New GPU state (values: on, off, auto)"))
        (@subcommand run =>
            (about: "Run a program using primus-run")
            (@arg program: +required ... "The program to be run, and its arguments"))
        (@subcommand switch =>
            (about: "Switch xsession GPU, this will log out you (values: auto, intel, nvidia)")
            (@arg gpu: +required "Which GPU to use now (values: auto, intel, nvidia)"))
        (@subcommand startup =>
            (about: "Set the starting GPU when booting up (values: nvidia, intel, nvidia_once)")
            (@arg gpu: +required "Which GPU to start at boot (values: nvidia, intel, nvidia_once)"))
        (@subcommand default =>
            (about: "Set a default service to start at boot (values: optimus, bumblebee)")
            (@arg service: +required "The service to be started at boot (values: optimus, bumblebee)"))
        (@subcommand status =>
            (about: "Show the overall status (default subcommand)"))
    };

    let matches = nvman
        .clone()
        .get_matches_safe()
        .unwrap_or_else(|e| e.exit());


    let nvman_service = systemd::Service::new("nvman");
    if nvman_service.status().1 == systemd::Status::Disabled {
        warn_print!("`nvman.service` is not enabled, potential risk!");
    }

    match matches.subcommand() {
        ("status", _) | ("", _) => {
            utils::print::print_status();
        }

        ("nvidia", Some(sub)) => {
            let state = sub.value_of("state").unwrap();

            match state.to_lowercase().as_str() {
                "on" => nvidia::set_state(nvidia::State::On),
                "off" => nvidia::set_state(nvidia::State::Off),
                "auto" =>  nvidia::toggle_state(),
                invalid => {
                    error_print!("Invalid value `{}`", invalid);
                    println!("{} {}", sub.usage(), "(state: on, off, auto)");
                }
            }
        }

        ("startup", Some(sub)) => {
            let startup = sub.value_of("gpu").unwrap();
            match startup.to_lowercase().as_str() {
                "nvidia" => utils::optimus::set_startup(utils::optimus::Mode::Nvidia),
                "intel" => utils::optimus::set_startup(utils::optimus::Mode::Intel),
                invalid => {
                    error_print!("Invalid value `{}`", invalid);
                    println!("{} {}", sub.usage(), "(gpu: nvidia, intel)");
                }
            }
        }

        ("switch", Some(sub)) => {
            unimplemented!()
        }

        ("run", Some(sub)) => {
            unimplemented!()
        }
        _ => unreachable!(),
    }
}
