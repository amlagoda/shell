mod handler;

use self::handler::{command, exit, history as history_get, HistoryDirection};
use self::handler::{input_add, input_complete, input_sub};
use crate::fmt::bell;
use crate::history::History;
use crate::io::Stdio;
use crate::keyboard::{to_action, TerminalAction};
use crate::session::State;
use crate::setting::Setting;
use crossterm::event::read as get_pressed_key;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{Error, Write};

pub fn mode_interactive(
    state: &mut State,
    stdio: &mut Stdio,
    history: &mut History,
    setting: &Setting,
) -> Result<(), Error> {
    enable_raw_mode()?;
    write!(stdio.stdout(), "\r$ ")?;
    stdio.stdout().flush()?;

    let bell = bell();

    loop {
        let pressed_key = get_pressed_key()?.as_key_event();

        if pressed_key.is_none() {
            continue;
        }

        let action = to_action(&pressed_key.unwrap());

        if action.is_none() {
            continue;
        }

        let action = action.unwrap();
        let is_exit = run_handler(state, stdio, history, setting, &action, bell.as_str())?;

        state.set_previous_action(action);

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
    setting: &Setting,
) -> Result<(), Error> {
    command(stdio, state, history, setting)?;

    Ok(())
}

fn run_handler(
    state: &mut State,
    stdio: &mut Stdio,
    history: &mut History,
    setting: &Setting,
    action: &TerminalAction,
    bell: &str,
) -> Result<bool, Error> {
    let mut is_exit = false;

    match action {
        TerminalAction::Command => {
            disable_raw_mode()?;
            is_exit = command(stdio, state, history, setting)?;
            enable_raw_mode()?;
        }
        TerminalAction::Exit => is_exit = exit(stdio)?,
        TerminalAction::HistoryNext => {
            history_get(&HistoryDirection::Next, stdio, state, history, bell)?
        }
        TerminalAction::HistoryPrev => {
            history_get(&HistoryDirection::Prev, stdio, state, history, bell)?
        }
        TerminalAction::InputAdd(symbol) => input_add(symbol.to_string().as_str(), stdio, state)?,
        TerminalAction::InputSub => input_sub(stdio, state)?,
        TerminalAction::InputComplete => input_complete(stdio, state, setting, bell)?,
    };

    Ok(is_exit)
}
