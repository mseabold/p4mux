use std::fs;
use std::io::Result;
use std::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
#[serde(default)]
pub struct PerforceConfig {
    pub p4conf: String,
    pub status_flags: String,
}

#[derive(Deserialize,Serialize)]
#[serde(default)]
pub struct TmuxIcons {
    pub login: String,
    pub logout: String,
    pub add: String,
    pub edit: String,
    pub delete: String
}

#[derive(Deserialize,Serialize, Debug)]
#[serde(default)]
pub struct TmuxStyles {
    pub clear: String,
    pub login: String,
    pub logout: String,
    pub client: String,
    pub add: String,
    pub edit: String,
    pub delete: String,
    pub reconcile_add: String,
    pub reconcile_edit: String,
}

#[derive(Deserialize,Serialize)]
#[serde(default)]
pub struct TmuxConfig {
    pub format: Vec<String>,

    #[serde(default)]
    pub icons: TmuxIcons,

    #[serde(default)]
    pub styles: TmuxStyles
}

impl Default for PerforceConfig {
    fn default() -> Self {
        PerforceConfig {
            p4conf: ".p4.conf".to_string(),
            status_flags: "-m".to_string()
        }
    }
}

impl Default for TmuxIcons {
    fn default() -> Self {
        TmuxIcons {
            login: "󱘖".to_string(),
            logout: "".to_string(),
            add: "".to_string(),
            edit: "".to_string(),
            delete: "󰆴".to_string()
        }
    }
}

impl Default for TmuxStyles {
    fn default() -> Self {
        TmuxStyles {
            clear: "#[fg=default]".to_string(),
            login: "#[fg=green]".to_string(),
            logout: "#[fg=red]".to_string(),
            client: "#[fg=white,bold]".to_string(),
            add: "#[fg=yellow,bold]".to_string(),
            edit: "#[fg=yellow,bold]".to_string(),
            delete: "#[fg=yellow,bold]".to_string(),
            reconcile_add: "#[fg=red,bold]".to_string(),
            reconcile_edit: "#[fg=red,bold]".to_string(),
        }
    }
}

impl Default for TmuxConfig {
    fn default() -> Self {
        TmuxConfig {
            format: Vec::from(["client".to_string(), " ".to_string(), "login".to_string(), " ".to_string(), "status".to_string()]),
            icons: Default::default(),
            styles: Default::default()
        }
    }
}

#[derive(Deserialize,Serialize)]
#[serde(default)]
pub struct Config {
    #[serde(default)]
    pub perforce: PerforceConfig,

    #[serde(default)]
    pub tmux: TmuxConfig
}

impl Default for Config {
    fn default() -> Self {
        Config {
            perforce: PerforceConfig {
                ..Default::default()
            },

            tmux: TmuxConfig {
                ..Default::default()
            }
        }
    }
}

pub fn get_config() -> Result<Config> {
    let mut config_str = String::new();

    if let Some(mut home) = std::env::home_dir() {
        home.push(".p4mux.conf");

        if home.is_file() {
            config_str = fs::read_to_string(home.as_path())?;
        }
    }
    let config = toml::from_str(&config_str).unwrap();
    return Ok(config);
}

pub fn get_default_config() -> Config {
    return Config {
        ..Default::default()
    }
}

pub fn print_config(config: &Config) {
    let conf_str = toml::to_string(config).unwrap();
    println!("{conf_str}");
}
