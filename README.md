# p4mux
P4 in your tmux status bar


![demo](https://github.com/user-attachments/assets/1e0932fb-c343-4348-9c4a-d7d7416bbd7f)

## Introduction
Inspired by [gitmux](https://github.com/arl/gitmux), this provides a live status of your current Perforce workspace in your tmux status bar

## Requirements
- Tmux (at least 2.1+)
- P4 cli (p4) installed in PATH
- A P4CONFIG-style perforce configuration file at the root of the workspace file provided by either P4CONFIG environment variable or p4mux config
- A [nerd font](https://www.nerdfonts.com/) for the icons

## Installing
### Binary release
Download the [latest](https://github.com/mseabold/p4mux/releases/latest) release and extract to location in PATH

### Cargo via git
You can install using [cargo](https://doc.rust-lang.org/cargo/) directly from git with:

```shell
cargo install --git https://github.com/mseabold/p4mux
```

### Compiling from source

Clone the repo:
```shell
git clone https://github.com/mseabold/p4mux && cd p4mux
```
Build and install p4mux:
```shell
cargo install --path .
```

## Enabling in TMUX
Add the call to p4mux in your tmux configuration:
```shell
set -ag status-right '#(p4mux "#{pane_current_path}")'
```

## Usage
P4mux provides the following parameters
```
Usage: p4mux [OPTIONS] [PATH]

Arguments:
  [PATH]

Options:
  -v, --verbose
  -p, --print-cfg
  -c, --config <CONFIG>
  -h, --help             Print help
  -V, --version          Print version
```
By default, `p4mux` will use the current working directory and search upward for your P4CONF file. If `PATH` is supplied, it will start at the supplied directory and work upward.

## Configuration

### Config File
P4mux is functional out of the box with a default configuration. However it provides the ability to configure both perforce parameters and display icons/styles. Configuration is done via a TOML file. This file can either by supplied via the `-c` option at the command line or by placing it at `$HOME/.p4mux.conf`. The `-c` option will override any existing config file at the default `$HOME` location. The current default config can be printed using the `-p` option, which will display as follows:
```toml
[perforce]
p4conf = ".p4.conf"
status_flags = "-m"

[tmux]
format = ["client", " ", "login", " ", "status"]
status_sep = " "

[tmux.icons]
login = "󱘖"
logout = ""
add = ""
edit = ""
delete = "󰆴"

[tmux.styles]
clear = "#[none]"
login = "#[fg=green]"
logout = "#[fg=red]"
client = "#[fg=white,bold]"
add = "#[fg=yellow,bold]"
edit = "#[fg=yellow,bold]"
delete = "#[fg=yellow,bold]"
reconcile_add = "#[fg=red,bold]"
reconcile_edit = "#[fg=red,bold]"
```
Note that you do not need to specify every parameter in your own configuration file. Any parameter not in the file will use the preset default value.

### Configuration Parameters

#### Perforce
Configuration parmaters for how `p4mux` detects and interacts with the perforce workspace and client

| Parameter | Usage |
| --------- | ----- |
| `p4conf`  | Name of P4CONFIG file to search for to identify the p4 workspace root and client name. This will always be superceded by the `P4CONFIG` environment variable when set |
| `status_flags` | Additional flags to pass to the p4 client when performing the `p4 status` command. The default value is to pass `-m` to speed up `reconcile_edit` checks |
#### Format

You can control the format of the status output for the `tmux.format` configuration parameters. This parameter is a list of either raw strings or directives that tell `p4mux` what to fetch and display. Any string not recognized as a directory will be appending as-is. The following directives are currently available

| Directive | Usage |
| --------- | ----- |
| `client`  | The perforce client name in the current workspace. This is pulled from the `P4CONF` file. |
| `login`   | The current login session state |
| `add`     | Current number of files opened for add |
| `edit`    | Current number of files opened for edit |
| `delete`  | Current number of files opened for delete |
| `reconcile_add` | Current number of files which exist in the workspace but are not currently tracked by perforce. I.e. the number of files that show via `p4 status -a` |
| `reconcile_edit` | Currnet number of files which are not currently opened for edit but whose contents differ from that of the depot file. I.e. files that show via `p4 status -e` |
| `status` | A meta-directive that contains `add`, `edit`, `delete`, `reconcile_add`, and `reconcile_edit` all separated by `tmux.status_sep` |
