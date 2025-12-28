use std::path::Path;
use std::fs;
use std::io::Result;
use std::vec::Vec;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default)]
pub struct PerforceConfig {
    pub p4conf: String,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct TmuxIcons {
    pub login: String,
    pub logout: String
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct TmuxStyles {
    pub clear: String,
    pub login: String,
    pub logout: String,
    pub client: String
}

#[derive(Deserialize)]
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
        }
    }
}

impl Default for TmuxIcons {
    fn default() -> Self {
        TmuxIcons {
            login: "󱘖".to_string(),
            logout: "".to_string()
        }
    }
}

impl Default for TmuxStyles {
    fn default() -> Self {
        TmuxStyles {
            clear: "#[fg=default]".to_string(),
            login: "#[fg=green]".to_string(),
            logout: "#[fg=red]".to_string(),
            client: "#[fg=white,bold]".to_string()
        }
    }
}

impl Default for TmuxConfig {
    fn default() -> Self {
        TmuxConfig {
            format: Vec::from(["client".to_string(), " ".to_string(), "login".to_string()]),
            icons: Default::default(),
            styles: Default::default()
        }
    }
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default)]
    pub perforce: PerforceConfig,

    #[serde(default)]
    pub tmux: TmuxConfig
}



pub fn get_config() -> Result<Config> {
    let path = Path::new("/home/matt/.p4mux.conf");
    let conf_str = fs::read_to_string(path)?;
    let config = toml::from_str(&conf_str).unwrap();
    return Ok(config);
}
