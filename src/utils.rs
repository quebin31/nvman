static VERSION: &str = "2.1.0";

pub mod print {
    use std::env;

    use crate::nvidia;
    use crate::optimus;
    use crate::settings;
    use crate::systemd::Service;

    use ansi_term::Colour::{Green, White, Yellow};

    pub fn print_status() {
        let optimus = Service::new("optimus-manager");
        let bumblebee = Service::new("bumblebeed");

        let wbold = White.bold();

        let optimus_title = wbold.paint(format!("{:<16} :", "Optimus"));
        let bumbleebee_title = wbold.paint(format!("{:<16} :", "Bumblebee"));
        let nvidia_title = wbold.paint(format!("{:<16} :", "NVIDIA GPU"));
        let default_title = wbold.paint(format!("{:<16} :", "Default service"));
        let mode_title = wbold.paint(format!("{:<16} :", "Optimus mode"));
        let startup_title = wbold.paint(format!("{:<16} :", "Optimus startup"));

        let default_service = settings::get_default_service();

        println!("{} {}", optimus_title, optimus);
        println!("{} {}", bumbleebee_title, bumblebee);
        println!("{} {}", nvidia_title, nvidia::state());
        println!("{} {}", default_title, default_service);
        println!("{} {}", mode_title, optimus::get_mode());
        println!("{} {}", startup_title, optimus::get_startup());
    }

    pub fn print_help() {
        println! {
            "{} {} [{}] [{}]",
            White.bold().paint("Usage:"),
            env::args().next().unwrap(),
            Green.paint("command"),
            Yellow.paint("arg(s)"),
        };

        println!("{} {}", White.bold().paint("Version:"), super::VERSION);
        println!();
        println!("{}", White.bold().paint("Commands:"));

        println! {
            "  {}  {}           Turn on/off NVIDIA GPU",
            Green.paint("nvidia"),
            Yellow.paint("<on|off|auto>"),
        };

        println! {
            "  {}     {}          Run command using primusrun",
            Green.paint("run"),
            Yellow.paint("<cmd> [args..]"),
        };

        println! {
            "  {}  {}     Switch GPU using optimus-manager",
            Green.paint("switch"),
            Yellow.paint("<nvidia|intel|auto>"),
        };

        println! {
            "  {} {}          Set startup GPU for optimus",
            Green.paint("startup"),
            Yellow.paint("<nvidia|intel>"),
        };

        println! {
            "  {} {}     Set default service started at boot",
            Green.paint("default"),
            Yellow.paint("<optimus|bumblebee>"),
        };

        println! {
            "  {}     {}     Use the specified service for this session",
            Green.paint("use"),
            Yellow.paint("<optimus|bumblebee>"),
        };

        println! {
            "  {}    {}     Stop using the specified service",
            Green.paint("stop"),
            Yellow.paint("<optimus|bumblebee>"),
        };

        println!();
        println! {
            "  {}                            Show this help",
            Green.paint("help"),
        };

        println! {
            "  {}                          Show the current status (default)",
            Green.paint("status"),
        };
    }

    pub fn print_usage_nvidia() {
        println! {
            "{} {} {} {}",
            White.bold().paint("Usage:"),
            env::args().next().unwrap(),
            Green.paint("nvidia"),
            Yellow.paint("<on|off|auto>"),
        };
    }

    pub fn print_usage_run() {
        println! {
            "{} {} {} {}",
            White.bold().paint("Usage:"),
            env::args().next().unwrap(),
            Green.paint("run"),
            Yellow.paint("<cmd> [args..]"),
        };
    }

    pub fn print_usage_switch() {
        println! {
            "{} {} {} {}",
            White.bold().paint("Usage:"),
            env::args().next().unwrap(),
            Green.paint("switch"),
            Yellow.paint("<nvidia|intel|auto>"),
        };
    }

    pub fn print_usage_startup() {
        println! {
            "{} {} {} {}",
            White.bold().paint("Usage:"),
            env::args().next().unwrap(),
            Green.paint("startup"),
            Yellow.paint("<nvidia|intel>"),
        };
    }

    pub fn print_usage_use() {
        println! {
            "{} {} {} {}",
            White.bold().paint("Usage:"),
            env::args().next().unwrap(),
            Green.paint("use"),
            Yellow.paint("<optimus|bumblebee>"),
        };
    }

    pub fn print_usage_stop() {
        println! {
            "{} {} {} {}",
            White.bold().paint("Usage:"),
            env::args().next().unwrap(),
            Green.paint("stop"),
            Yellow.paint("<optimus|bumblebee>"),
        };
    }

    pub fn print_usage_default() {
        println! {
            "{} {} {} {}",
            White.bold().paint("Usage:"),
            env::args().next().unwrap(),
            Green.paint("default"),
            Yellow.paint("<optimus|bumblebee>"),
        };
    }
}

pub mod checks {
    use crate::optimus;
    use crate::systemd::Service;
    use shells::bash;

    #[derive(Debug, PartialEq)]
    pub enum CheckResult {
        Ok,
        Bad,
    }

    impl CheckResult {
        pub fn is_bad(&self) -> bool {
            *self == CheckResult::Bad
        }
    }

    pub fn check_gdm_prime() -> CheckResult {
        let (code, gdm_name, _) = bash!("pacman -Qq gdm");
        if code != 0 {
            return CheckResult::Ok;
        }

        let gdm = Service::new("gdm");
        if !gdm.enabled() {
            return CheckResult::Ok;
        }

        let mut result = if gdm_name.trim() != "gdm-prime" {
            error_print! {
                "Looks like you're using gdm, optimus-manager currently depends \
                on gdm-prime (tweaked version of gdm available on the AUR)."
            }

            CheckResult::Bad
        } else {
            CheckResult::Ok
        };

        let (_, contents, _) = bash!("cat /etc/gdm/custom.conf");
        let contents = contents.split('\n');
        for line in contents {
            let line = line.trim();
            if !line.contains("WaylandEnable") {
                continue;
            }

            if line.contains('#') || line.contains("true") {
                error_print! {
                    "gdm is currently using Wayland, disable it in /etc/gdm/custom.conf (WaylandEnable=false)"
                }
                result = CheckResult::Bad;
            }
        }

        result
    }

    pub fn check_nvman_service() -> CheckResult {
        let nvman = Service::new("nvman");

        if !nvman.enabled() {
            warn_print!("Potential risk, nvman service is not enabled!");
            CheckResult::Bad
        } else {
            CheckResult::Ok
        }
    }

    pub fn check_optimus_mode() -> CheckResult {
        let optimus = Service::new("optimus");
        if optimus.active() && optimus::get_mode().is_nvidia() {
            error_print!("Already using nvidia with optimus!");
            CheckResult::Bad
        } else {
            CheckResult::Ok
        }
    }
}
