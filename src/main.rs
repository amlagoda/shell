use self::command::get_command_list;
use self::control::{mode_command, mode_interactive};
use self::env::{get_args, get_bin_paths, get_current_dir, get_history_log_path};
use self::fmt::NewLine;
use self::history::{download as download_history_log, upload as upload_history_log, History};
use self::io::Stdio;
use self::session::State;
use self::setting::{ProgramMode, Setting};
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
mod setting;
mod structure;

fn main() -> Result<(), Error> {
    let mut stdio = Stdio::new();
    let mut state = State::new();
    let mut history = History::new();
    let mut new_line = NewLine::new();

    let current_dir = get_current_dir();
    let bin_paths = get_bin_paths()?;
    let args = get_args();
    let available_commands = get_command_list();

    if let Some(path) = get_history_log_path() {
        download_history_log(&mut history, path.as_str())?;
    }

    if args.is_empty() {
        new_line.set_stdout_start(true);
        new_line.set_stderr_start(true);

        let setting = Setting::from(
            ProgramMode::Interactive,
            new_line,
            bin_paths,
            current_dir,
            available_commands,
        );

        mode_interactive(&mut state, &mut stdio, &mut history, &setting)?;
    } else {
        let input = args.join(" ");
        state.input().push_as_system(input.as_str());

        let setting = Setting::from(
            ProgramMode::Command,
            new_line,
            bin_paths,
            current_dir,
            available_commands,
        );

        mode_command(&mut state, &mut stdio, &mut history, &setting)?;
    }

    if let Some(file_path) = get_history_log_path() {
        upload_history_log(&mut history, file_path.as_str(), false)?;
    }

    Ok(())
}
