use crate::core::run;
use crate::env::get_history_log_path;
use crate::history::{upload as upload_log, Log};
use crate::io::Stdio;
use crate::parser::parse;
use crate::session::State;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{Error, Write};

pub fn handle(
    stdio: &mut Stdio,
    state: &mut State,
    log: &mut Log,
    bin_paths: &Vec<&str>,
) -> Result<bool, Error> {
    let input = state.terminal().input().get();

    if input.is_none() {
        return Ok(false);
    }

    disable_raw_mode()?;

    let parseds = parse(input.unwrap())?.unwrap();
    let output_starts_newline = true;

    let mut is_exit = run(
        &parseds.iter().collect(),
        stdio,
        log,
        bin_paths,
        output_starts_newline,
    )?;

    enable_raw_mode()?;

    state.terminal().input().reset();
    log.reset();

    if is_exit {
        if let Some(file_path) = get_history_log_path() {
            upload_log(log, file_path.as_str(), false)?;
        }
        is_exit = true;
    } else {
        write!(stdio.stdout(), "\r\n$ ")?;
        stdio.stdout().flush()?;
    }

    Ok(is_exit)
}
