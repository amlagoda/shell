mod handler;

use crate::control::handler::{
    command, exit, history, input_add, input_complete, input_sub, HistoryDirection,
};
use crate::history::Log;
use crate::io::Stdio;
use crate::keyboard::{to_action, TerminalAction};
use crate::session::State;
use crossterm::event::KeyEvent;
use std::io::Error;

pub fn run_interactive(
    key: &KeyEvent,
    state: &mut State,
    stdio: &mut Stdio,
    log: &mut Log,
    commands: &Vec<&str>,
    bin_paths: &Vec<&str>,
    current_dir: &str,
    output_starts_newline: bool,
) -> Result<bool, Error> {
    let mut is_exit = false;
    let action = to_action(key);

    if action.is_none() {
        return Ok(is_exit);
    }

    match action.unwrap() {
        TerminalAction::Command => {
            is_exit = command(stdio, state, log, bin_paths, output_starts_newline)?
        }

        TerminalAction::Exit => {
            exit(stdio)?;
            is_exit = true;
        }
        TerminalAction::HistoryNext => history(&HistoryDirection::Next, state, stdio, log)?,
        TerminalAction::HistoryPrev => history(&HistoryDirection::Prev, state, stdio, log)?,
        TerminalAction::InputAdd(symbol) => input_add(symbol.to_string().as_str(), state, stdio)?,
        TerminalAction::InputSub => input_sub(state, stdio)?,
        TerminalAction::InputComplete => {
            input_complete(state, stdio, commands, bin_paths, current_dir)?
        }
    };

    Ok(is_exit)
}

pub fn run_command(
    state: &mut State,
    stdio: &mut Stdio,
    log: &mut Log,
    bin_paths: &Vec<&str>,
    output_starts_newline: bool,
) -> Result<bool, Error> {
    command(stdio, state, log, bin_paths, output_starts_newline)
}
