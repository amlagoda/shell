use crate::control::{mode_command, mode_interactive};
use crate::env::{get_args, get_history_log_path, split_env_path};
use crate::fmt::NewLine;
use crate::history::{download as download_history_log, History};
use crate::io::Stdio;
use crate::session::State;
use std::io::Error;

mod command;
mod complete;
mod control;
mod core;
mod cursor;
mod env;
mod fmt;
mod fs;
mod history;
mod io;
mod keyboard;
mod parser;
mod process;
mod session;
mod structure;

fn main() -> Result<(), Error> {
    let mut stdio = Stdio::new();
    let mut state = State::new();
    let mut history = History::new();
    let mut newline = NewLine::new();

    let bin_paths = split_env_path()?;
    let bin_paths = bin_paths.iter().map(|r| r.as_str()).collect();
    let args = get_args();

    if let Some(path) = get_history_log_path() {
        download_history_log(&mut history, path.as_str())?;
    }

    if args.is_empty() {
        newline.set_stdout_start(true);
        newline.set_stderr_start(true);

        mode_interactive(&mut state, &mut stdio, &mut history, &newline, &bin_paths)
    } else {
        let input = args.join(" ");
        state.terminal().input().push_as_system(input.as_str());

        mode_command(&mut state, &mut stdio, &mut history, &newline, &bin_paths)
    }
}
