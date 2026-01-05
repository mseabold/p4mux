use crate::config::Config;
use std::path::Path;
use std::process::Command;
use std::fs;
use serde::Deserialize;
use serde_json;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct P4StatusEntry {
    action: Option<String>,
    change: Option<String>,
}

#[derive(Default, Clone, Debug)]
pub struct OpenCounts {
    pub add: u32,
    pub delete: u32,
    pub edit: u32,
    pub move_add: u32,
    pub move_del: u32,
}

#[derive(Default, Debug)]
pub struct StatusCounts {
    pub open: Option<OpenCounts>,
    pub add_reconcile: u32,
    pub edit_reconcile: u32,
}

fn build_base_p4_cmd(path: Option<&String>) -> Command {
    let mut cmd = Command::new("p4");

    cmd.arg("-Mj").arg("-ztag");

    if let Some(ws_path) = path {
        cmd.arg("-d").arg(ws_path);
    }

    return cmd;
}

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

pub fn get_open_counts(path: Option<&String>) -> Option<OpenCounts> {
    let mut cmd = build_base_p4_cmd(path);
    cmd.arg("opened");
    let output = cmd.output().expect("failed to execute p4 command");

    if output.status.success() {
        let stdout_str = String::from_utf8(output.stdout).unwrap();
        let mut results: Vec<P4StatusEntry> = Vec::new();

        for line in stdout_str.lines() {
            let json_obj = serde_json::from_str(line).unwrap();
            results.push(json_obj);
        }

        let mut counts = OpenCounts { ..Default::default() };

        for result in results {
            if let Some(action) = result.action {
                match action.as_str() {
                    "add" => { counts.add += 1; },
                    "edit" => { counts.edit += 1; },
                    "delete" => { counts.delete += 1 },
                    "move/add" => { counts.move_add += 1},
                    "move/delete" => { counts.move_del += 1},
                    _ => {}
                }
            }
        }

        return Some(counts);
    }
    else {
        return None;
    }
}

pub fn get_status_counts(config: &Config, path: Option<&String>) -> Option<StatusCounts> {
    let mut cmd = build_base_p4_cmd(path);
    cmd.arg("status").arg(config.perforce.status_flags.as_str());
    let output = cmd.output().expect("failed to execute p4 command");

    if output.status.success() {
        let stdout_str = String::from_utf8(output.stdout).unwrap();
        let mut results: Vec<P4StatusEntry> = Vec::new();

        for line in stdout_str.lines() {
            let json_obj = serde_json::from_str(line).unwrap();
            results.push(json_obj);
        }

        let mut open_counts = OpenCounts { ..Default::default() };
        let mut counts = StatusCounts { ..Default::default() };

        for result in results {
            if let Some(action) = result.action {
                match action.as_str() {
                    "add" => { if result.change.is_some() { open_counts.add += 1; } else { counts.add_reconcile += 1} },
                    "edit" => { if result.change.is_some() { open_counts.edit += 1; } else { counts.edit_reconcile += 1} },
                    "delete" => { open_counts.delete += 1 },
                    "move/add" => { open_counts.move_add += 1},
                    "move/delete" => { open_counts.move_del += 1},
                    _ => {}
                }
            }
        }

        counts.open = Some(open_counts);

        return Some(counts);
    }
    else {
        return None;
    }
}

pub fn is_logged_in(path: Option<&String>) -> bool {
    let mut cmd = build_base_p4_cmd(path);
    cmd.arg("login").arg("-s");
    let output = cmd.output().expect("failed to execute p4 command");

    return output.status.success();
}
