use crate::core::run;
use crate::env::get_history_log_path;
use crate::history::{upload as upload_log, History};
use crate::io::Stdio;
use crate::parser::parse;
use crate::session::State;
use crate::setting::Setting;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{Error, Write};

pub fn handle(
    stdio: &mut Stdio,
    state: &mut State,
    history: &mut History,
    setting: &Setting,
) -> Result<bool, Error> {
    let input = state.terminal().input().get();

    if input.is_none() {
        return Ok(false);
    }

    let parseds = parse(input.unwrap())?.unwrap();

    disable_raw_mode()?;
    let mut is_exit = run(&parseds.iter().collect(), stdio, history, setting)?;
    enable_raw_mode()?;

    state.terminal().input().reset();
    history.reset();

    if is_exit {
        if let Some(file_path) = get_history_log_path() {
            upload_log(history, file_path.as_str(), false)?;
        }
        is_exit = true;
    } else {
        write!(stdio.stdout(), "\r\n$ ")?;
        stdio.stdout().flush()?;
    }

    Ok(is_exit)
}
