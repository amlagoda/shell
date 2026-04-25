mod handler;

use crate::command::get_command_list;
use crate::control::handler::{
    command, exit, history as history_get, input_add, input_complete, input_sub, HistoryDirection,
};
use crate::env::get_current_dir;
use crate::fmt::NewLine;
use crate::history::History;
use crate::io::Stdio;
use crate::keyboard::{to_action, TerminalAction};
use crate::session::State;
use crossterm::event::read as read_key;
use crossterm::event::KeyEvent;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{Error, Write};

pub fn mode_interactive(
    state: &mut State,
    stdio: &mut Stdio,
    history: &mut History,
    newline: &NewLine,
    bin_paths: &Vec<&str>,
) -> Result<(), Error> {
    enable_raw_mode()?;
    write!(stdio.stdout(), "\r$ ")?;
    stdio.stdout().flush()?;

    loop {
        let pressed_key = read_key()?.as_key_event();

        if pressed_key.is_none() {
            continue;
        }

        let available_commands = get_command_list();
        let available_commands = available_commands.iter().map(|r| r.as_str()).collect();

        let is_exit = run_interactive(
            &pressed_key.unwrap(),
            state,
            stdio,
            history,
            newline,
            &available_commands,
            bin_paths,
            get_current_dir().as_str(),
        )?;

        state.keyboard().set_previous_key(pressed_key.unwrap());

        if is_exit {
            break;
        }
    }

    disable_raw_mode()?;

    Ok(())
}

pub fn mode_command(
    state: &mut State,
    stdio: &mut Stdio,
    history: &mut History,
    newline: &NewLine,
    bin_paths: &Vec<&str>,
) -> Result<(), Error> {
    run_command(state, stdio, history, newline, bin_paths)?;

    Ok(())
}

pub fn run_interactive(
    key: &KeyEvent,
    state: &mut State,
    stdio: &mut Stdio,
    history: &mut History,
    newline: &NewLine,
    commands: &Vec<&str>,
    bin_paths: &Vec<&str>,
    current_dir: &str,
) -> Result<bool, Error> {
    let mut is_exit = false;
    let action = to_action(key);

    if action.is_none() {
        return Ok(is_exit);
    }

    match action.unwrap() {
        TerminalAction::Command => is_exit = command(stdio, state, history, newline, bin_paths)?,

        TerminalAction::Exit => {
            exit(stdio)?;
            is_exit = true;
        }
        TerminalAction::HistoryNext => history_get(&HistoryDirection::Next, state, stdio, history)?,
        TerminalAction::HistoryPrev => history_get(&HistoryDirection::Prev, state, stdio, history)?,
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
    history: &mut History,
    newline: &NewLine,
    bin_paths: &Vec<&str>,
) -> Result<bool, Error> {
    command(stdio, state, history, newline, bin_paths)
}
