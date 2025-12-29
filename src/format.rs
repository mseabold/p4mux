use crate::config::Config;
use crate::p4;
use std::collections::HashMap;
use std::vec::Vec;

#[derive(Debug)]
struct FormatState<'a> {
    client: &'a String,
    status_counts: Option<p4::StatusCounts>,
    working_path: Option<&'a String>
}

type FormatHandler = fn(&mut FormatState, &Config, &mut Vec<String>);

fn check_get_status_counts(state: &mut FormatState, config: &Config) {
    if state.status_counts.is_none() {
        let counts = p4::get_status_counts(config, state.working_path).unwrap();
        state.status_counts = Some(counts);
    }
}

fn client_handler(state: &mut FormatState, config: &Config, output:  &mut Vec<String>) {
    output.push(config.tmux.styles.client.clone());
    output.push(state.client.clone());
}

fn login_handler(state: &mut FormatState, config: &Config, output: &mut Vec<String>) {
    if p4::is_logged_in(state.working_path) {
        output.push(config.tmux.styles.login.clone());
        output.push(config.tmux.icons.login.clone());
    }
    else {
        output.push(config.tmux.styles.logout.clone());
        output.push(config.tmux.icons.logout.clone());
    }
}

fn open_add_handler(state: &mut FormatState, config: &Config, output:  &mut Vec<String>) {
    check_get_status_counts(state, config);

    if let Some(counts) = state.status_counts.as_ref() {
        let added = counts.add;

        if added > 0 {
            output.push(config.tmux.styles.add.clone());
            output.push(added.to_string());
            output.push(config.tmux.icons.add.clone());
        }
    }
}

fn open_edit_handler(state: &mut FormatState, config: &Config, output:  &mut Vec<String>) {
    check_get_status_counts(state, config);

    if let Some(counts) = state.status_counts.as_ref() {
        let edited = counts.edit;

        if edited > 0 {
            output.push(config.tmux.styles.edit.clone());
            output.push(edited.to_string());
            output.push(config.tmux.icons.edit.clone());
        }
    }
}

fn open_delete_handler(state: &mut FormatState, config: &Config, output:  &mut Vec<String>) {
    check_get_status_counts(state, config);

    if let Some(counts) = state.status_counts.as_ref() {
        let edited = counts.delete;

        if edited > 0 {
            output.push(config.tmux.styles.delete.clone());
            output.push(edited.to_string());
            output.push(config.tmux.icons.delete.clone());
        }
    }
}

fn reconcile_add_handler(state: &mut FormatState, config: &Config, output:  &mut Vec<String>) {
    check_get_status_counts(state, config);

    if let Some(counts) = state.status_counts.as_ref() {
        let added = counts.add_reconcile;

        if added > 0 {
            output.push(config.tmux.styles.reconcile_add.clone());
            output.push(added.to_string());
            output.push(config.tmux.icons.add.clone());
        }
    }
}

fn reconcile_edit_handler(state: &mut FormatState, config: &Config, output:  &mut Vec<String>) {
    check_get_status_counts(state, config);

    if let Some(counts) = state.status_counts.as_ref() {
        let edited = counts.edit_reconcile;

        if edited > 0 {
            output.push(config.tmux.styles.reconcile_edit.clone());
            output.push(edited.to_string());
            output.push(config.tmux.icons.edit.clone());
        }
    }
}

fn status_handler(state: &mut FormatState, config: &Config, output:  &mut Vec<String>) {
    open_add_handler(state, config, output);
    open_edit_handler(state, config, output);
    open_delete_handler(state, config, output);
    reconcile_add_handler(state, config, output);
    reconcile_edit_handler(state, config, output);
}

pub fn format_output(path: Option<&String>, client: &String, config: &Config) -> String {
    let mut handlers: HashMap<String, FormatHandler> = HashMap::new();
    handlers.insert("client".to_string(), client_handler);
    handlers.insert("login".to_string(), login_handler);
    handlers.insert("add".to_string(), open_add_handler);
    handlers.insert("edit".to_string(), open_edit_handler);
    handlers.insert("delete".to_string(), open_delete_handler);
    handlers.insert("reconcile_add".to_string(), reconcile_add_handler);
    handlers.insert("reconcile_edit".to_string(), reconcile_edit_handler);
    handlers.insert("status".to_string(), status_handler);
    let mut output_fragments = Vec::new();
    output_fragments.push(config.tmux.styles.clear.clone());

    let mut state = FormatState {
        client: client,
        status_counts: None,
        working_path: path
    };

    for entry in &config.tmux.format {
        if handlers.contains_key(entry) {
            handlers.get(entry).unwrap()(&mut state, config, &mut output_fragments);
        }
        else {
            output_fragments.push(entry.clone());
        }

        output_fragments.push(config.tmux.styles.clear.clone());
    }

    return output_fragments.join("");
}

