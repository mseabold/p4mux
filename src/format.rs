use crate::config::Config;
use crate::p4;
use std::collections::HashMap;
use std::vec::Vec;

#[derive(Debug)]
struct FormatState<'a> {
    client: &'a String,
    status_counts: Option<p4::StatusCounts>,
    open_counts: Option<p4::OpenCounts>,
    working_path: Option<&'a String>,
    logged_in: bool
}

type FormatHandler = fn(&mut FormatState, &Config, &mut Vec<String>);

fn check_get_status_counts(state: &mut FormatState, config: &Config) {
    if state.status_counts.is_none() {
        let counts = p4::get_status_counts(config, state.working_path).unwrap();
        if let Some(opened) = counts.open.as_ref() {
            state.open_counts = Some(opened.clone());
        }
        state.status_counts = Some(counts);
    }
}

fn check_get_open_counts(state: &mut FormatState) {
    if state.open_counts.is_none() {
        state.open_counts = p4::get_open_counts(state.working_path);
    }
}

fn client_handler(state: &mut FormatState, config: &Config, output:  &mut Vec<String>) {
    output.push(config.tmux.styles.client.clone());
    output.push(state.client.clone());
}

fn login_handler(state: &mut FormatState, config: &Config, output: &mut Vec<String>) {
    if state.logged_in {
        output.push(format!("{}{}{}", config.tmux.styles.login, config.tmux.icons.login, config.tmux.styles.clear));
    }
    else {
        output.push(format!("{}{}{}", config.tmux.styles.logout, config.tmux.icons.logout, config.tmux.styles.clear));
    }
}

fn open_add_handler(state: &mut FormatState, config: &Config, output:  &mut Vec<String>) {
    if !state.logged_in {
        return;
    }

    check_get_open_counts(state);

    if let Some(counts) = state.open_counts.as_ref() {
        let added = counts.add;

        if added > 0 {
            output.push(format!("{}{}{}{}", config.tmux.styles.add, added.to_string(), config.tmux.icons.add, config.tmux.styles.clear));
        }
    }
}

fn open_edit_handler(state: &mut FormatState, config: &Config, output:  &mut Vec<String>) {
    if !state.logged_in {
        return;
    }

    check_get_open_counts(state);

    if let Some(counts) = state.open_counts.as_ref() {
        let edited = counts.edit;

        if edited > 0 {
            output.push(format!("{}{}{}{}", config.tmux.styles.edit, edited.to_string(), config.tmux.icons.edit, config.tmux.styles.clear));
        }
    }
}

fn open_delete_handler(state: &mut FormatState, config: &Config, output:  &mut Vec<String>) {
    if !state.logged_in {
        return;
    }

    check_get_open_counts(state);

    if let Some(counts) = state.open_counts.as_ref() {
        let deleted = counts.delete;

        if deleted > 0 {
            output.push(format!("{}{}{}{}", config.tmux.styles.delete, deleted.to_string(), config.tmux.icons.delete, config.tmux.styles.clear));
        }
    }
}

fn reconcile_add_handler(state: &mut FormatState, config: &Config, output:  &mut Vec<String>) {
    if !state.logged_in {
        return;
    }

    check_get_status_counts(state, config);

    if let Some(counts) = state.status_counts.as_ref() {
        let added = counts.add_reconcile;

        if added > 0 {
            output.push(format!("{}{}{}{}", config.tmux.styles.reconcile_add, added.to_string(), config.tmux.icons.add, config.tmux.styles.clear));
        }
    }
}

fn reconcile_edit_handler(state: &mut FormatState, config: &Config, output:  &mut Vec<String>) {
    if !state.logged_in {
        return;
    }

    check_get_status_counts(state, config);

    if let Some(counts) = state.status_counts.as_ref() {
        let edited = counts.edit_reconcile;

        if edited > 0 {
            output.push(format!("{}{}{}{}", config.tmux.styles.reconcile_edit, edited.to_string(), config.tmux.icons.edit, config.tmux.styles.clear));
        }
    }
}

fn status_handler(state: &mut FormatState, config: &Config, output:  &mut Vec<String>) {
    let mut st_segs: Vec<String> = Vec::new();
    open_add_handler(state, config, &mut st_segs);
    open_edit_handler(state, config, &mut st_segs);
    open_delete_handler(state, config, &mut st_segs);
    reconcile_add_handler(state, config, &mut st_segs);
    reconcile_edit_handler(state, config, &mut st_segs);

    output.push(st_segs.join(config.tmux.status_sep.as_str()));
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
        open_counts: None,
        working_path: path,
        logged_in: p4::is_logged_in(path)
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

