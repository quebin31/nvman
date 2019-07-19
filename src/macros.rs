#[macro_export]
macro_rules! sudo_bash {
    ($($cmd:tt)*) => {{
        use ansi_term::Colour::Green;
        use shells::{bash, execute_with};

        let (code, _, _) = bash!("sudo -n true");
        if code == 1 {
            println!("{} Privelege scalation needed", Green.paint("==>"));
        }

        execute_with("bash", &format!("sudo {}", format!($($cmd)*)))
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