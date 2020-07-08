use shells::bash;
use std::fs::File;
use std::io::Write;
use users::get_current_uid;

// TODO: This will change a lot if new settings come in
// TODO: - Add new setting `gpu_timeout`
pub fn set_default_service(service: &str) {
    if get_current_uid() != 0 {
        error_print!("This can be only run as root!");
        return;
    }

    let mut file = File::create("/etc/default/nvman").unwrap();
    let config = format!("default = {}", service);
    file.write_all(config.as_bytes()).unwrap();
}

pub fn get_default_service() -> String {
    let (_, contents, _) = bash!("cat /etc/default/nvman");

    let key_values = contents.split('\n');
    for key_value in key_values {
        let key_value: Vec<_> = key_value.split('=').collect();
        let (key, value) = (key_value.get(0), key_value.get(1));
        if key.is_none() || value.is_none() {
            continue;
        }

        let (key, value) = (key.unwrap().trim(), value.unwrap().trim());
        if key == "default" {
            return (*value).to_owned();
        }
    }

    warn_print!("Not found /etc/default/nvman or default is not present on it");
    info_print!("Creating default config");
    sudo_bash!("true");
    bash!("{} default optimus", std::env::args().next().unwrap());
    "optimus".to_owned()
}
