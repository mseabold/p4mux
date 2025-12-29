use std::env;
use std::path::PathBuf;
use clap::Parser;

use crate::format::format_output;

mod config;
mod p4;
mod format;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    path: Option<String>
}

fn get_conf_file(conf_name: &String, current_path: Option<&String>) -> Option<PathBuf> {
    let p4conf = match env::var("P4CONFIG") {
        Ok(conf) => conf,
        Err(_e) => conf_name.clone()
    };

    let mut current_dir = match current_path {
        Some(path) =>  PathBuf::from(path.as_str()),
        None => env::current_dir().unwrap()
    };

    loop {
        current_dir.push(&p4conf);

        if current_dir.is_file() {
            return Some(current_dir);
        }
        else {
            current_dir.pop();

            if !current_dir.pop() {
                return None
            }
        }
    }
}

fn main() {
    let conf: config::Config = config::get_config().unwrap();
    let args = Cli::parse();

    if let Some(p4conf_path) = get_conf_file(&conf.perforce.p4conf, args.path.as_ref()) {
        if let Some(p4_client) = p4::get_client_from_conf(&p4conf_path) {
            println!("{}", format_output(args.path.as_ref(), &p4_client, &conf));
        }
        else {
            println!("Unable to read p4 config");
        }
    }
    else {
        println!("Unable to find p4 config file");
    }
}
