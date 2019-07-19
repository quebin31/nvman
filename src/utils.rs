pub mod print {
    use crate::nvidia;
    use crate::systemd::Service;

    use ansi_term::Colour::White;
   
    pub fn print_status() {
        let optimus = Service::new("optimus-manager");
        let bumblebee = Service::new("bumblebeed");

        let wbold = White.bold();

        let optimus_title = wbold.paint(format!("{:<16}", "Optimus"));
        let bumbleebee_title = wbold.paint(format!("{:<16}", "Bumblebee"));
        let nvidia_title = wbold.paint(format!("{:<16}", "NVIDIA GPU"));
        let default_title  = wbold.paint(format!("{:<16}", "Default service"));
        let mode_title = wbold.paint(format!("{:<16}", "Optimus mode"));
        let startup_title  = wbold.paint(format!("{:<16}", "Optimus startup"));

        println!("{} : {}", optimus_title, optimus.status());
        println!("{} : {}", bumbleebee_title, bumblebee.status());
        println!("{} : {}", nvidia_title, nvidia::state());
        println!("{} : {}", default_title, "optimus");
        println!("{} : {}", mode_title, super::optimus::get_mode());
        println!("{} : {}", startup_title, super::optimus::get_startup());
    }
}


pub mod checks {
    use shells::bash;
    use crate::systemd::{
        Status, 
        Service,
        ServiceStatus,
    };


    #[derive(Debug)]
    pub enum GdmPrimeFlag {
        Ok,
        Bad,
    };

    pub fn check_gdm_prime() -> GdmPrimeFlag {
        let (code, gdm_name, _) = bash!("pacman -Qq gdm");
        if code != 0 {
            return GdmPrimeFlag::Ok;
        }

        let gdm = Service::new("gdm");

        let ServiceStatus(_, gdm_enable) = gdm.status();
        if gdm_enable == Status::Disabled {
            return GdmPrimeFlag::Ok;
        }

        let mut flag = GdmPrimeFlag::Ok;
        if gdm_name != "gdm-prime" {
            error_print!{
                "Looks like you're using gdm, optimus-manager currently depends \
                on gdm-prime (tweaked version of gdm)."
            }
            flag = GdmPrimeFlag::Bad;
        }

        let (code, stdout, _) = bash!("cat /etc/gdm/custom.conf | grep \'\\<WaylandEnable\\>\' | cut -dW -f 1");
        if stdout != "#" {
            error_print!{
                "gdm is currently using Wayland, disable it in /etc/gdm/custom.conf (WaylandEnable=false)"
            }
            flag = GdmPrimeFlag::Bad;
        }

        return flag;
    }
}