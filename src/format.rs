use crate::config::Config;
use crate::p4;
use std::collections::HashMap;
use std::vec::Vec;

type FormatHandler = fn(&String, &Config, &mut Vec<String>);

fn client_handler(client: &String, config: &Config, output:  &mut Vec<String>) {
    output.push(config.tmux.styles.client.clone());
    output.push(client.clone());
}

fn login_handler(_client: &String, config: &Config, output: &mut Vec<String>) {
    if p4::is_logged_in() {
        output.push(config.tmux.styles.login.clone());
        output.push(config.tmux.icons.login.clone());
    }
    else {
        output.push(config.tmux.styles.logout.clone());
        output.push(config.tmux.icons.logout.clone());
    }
}

pub fn format_output(client: &String, config: &Config) -> String {
    let mut handlers: HashMap<String, FormatHandler> = HashMap::new();
    handlers.insert("client".to_string(), client_handler);
    handlers.insert("login".to_string(), login_handler);
    let mut output_fragments = Vec::new();
    output_fragments.push(config.tmux.styles.clear.clone());

    for entry in &config.tmux.format {
        if handlers.contains_key(entry) {
            handlers.get(entry).unwrap()(client, config, &mut output_fragments);
        }
        else {
            output_fragments.push(entry.clone());
        }

        output_fragments.push(config.tmux.styles.clear.clone());
    }

    return output_fragments.join("");
}

