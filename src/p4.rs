use std::path::Path;
use std::process::Command;
use std::fs;

pub fn get_client_from_conf(conf_path: &Path) -> Option<String> {
    if let Ok(conf) = fs::read_to_string(conf_path) {
        for line in conf.lines() {
            let split: Vec<&str> = line.split("=").collect();

            match split[0] {
                "P4CLIENT" => { return Some(String::from(split[1])) },
                _ => {}
            }
        }
    }

    return None;
}

pub fn is_logged_in() -> bool {
    let output = Command::new("p4").arg("login").arg("-s").output().expect("failed to execute p4 command");

    return output.status.success();
}
