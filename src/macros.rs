#[macro_export]
macro_rules! sudo_bash {
    ($($cmd:tt)*) => {{
        use ansi_term::Colour::Green;
        use shells::{bash, execute_with};
        use users::get_current_uid;

        if get_current_uid() != 0 {
            let (code, _, _) = bash!("sudo -n true");

            if code != 0 {
                println!("{} Privilege scalation needed", Green.paint("==>"));
            }

            execute_with("bash", &format!("sudo {}", format!($($cmd)*)))
        } else {
            execute_with("bash", &format!($($cmd)*))
        }
    }};
}

#[macro_export]
macro_rules! error_print {
    ($($input:tt)*) => {{
        use ansi_term::Colour::Red;

        println!("{} {}", Red.bold().paint("Error:"), format!($($input)*))
    }};
}

#[macro_export]
macro_rules! warn_print {
    ($($input:tt)*) => {{
        use ansi_term::Colour::Yellow;

        println!("{} {}", Yellow.bold().paint("Warning:"), format!($($input)*))
    }};
}

#[macro_export]
macro_rules! info_print {
    ($($input:tt)*) => {{
        use ansi_term::Colour::Cyan;

        println!("{} {}", Cyan.bold().paint("==>"), format!($($input)*))
    }};
}
