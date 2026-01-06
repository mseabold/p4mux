# p4mux
P4 in your tmux status bar


![demo](https://github.com/user-attachments/assets/1e0932fb-c343-4348-9c4a-d7d7416bbd7f)

## Introduction
Inspired by [gitmux](https://github.com/arl/gitmux), this provides a live status of your current Perforce workspace in your tmux status bar

## Requirements
- Tmux (at least 2.1+)
- P4 cli (p4) installed in PATH
- A P4CONFIG-style perforce configuration file at the root of the workspace file provided by either P4CONFIG environment variable or p4mux config

## Installing
### Binary release
Download the [latest](https://github.com/mseabold/p4mux/releases/latest) release and extract to location in PATH

### Cargo via git
You can install using [cargo](https://doc.rust-lang.org/cargo/) directly from git with:

```
cargo install --git https://github.com/mseabold/p4mux
```

### Compiling from source

Clone the repo:
```
git clone https://github.com/mseabold/p4mux && cd p4mux
```
Build and install p4mux:
```
cargo install --path .
```

