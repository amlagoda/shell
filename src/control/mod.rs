mod handler;

use self::handler::{command, exit as command_exit, history as history_get, HistoryDirection};
use self::handler::{input_add, input_complete, input_sub};
use crate::fmt::bell;
use crate::io::Stdio;
use crate::keyboard::{to_action, TerminalAction};
use crate::session::State;
use crate::setting::Setting;
use crossterm::event::read as get_pressed_key;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::cmp::PartialEq;
use std::io::{Error, Write};

pub fn mode_interactive(
    state: &mut State,
    stdio: &mut Stdio,
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
        let exit = run_handler(state, stdio, setting, &action, bell.as_str())?;

        state.set_previous_action(action);

        if exit == Exit::Yes {
            break;
        }
    }

    disable_raw_mode()?;

    Ok(())
}

pub fn mode_command(state: &mut State, stdio: &mut Stdio, setting: &Setting) -> Result<(), Error> {
    command(stdio, state, setting)?;

    Ok(())
}

fn run_handler(
    state: &mut State,
    stdio: &mut Stdio,
    setting: &Setting,
    action: &TerminalAction,
    bell: &str,
) -> Result<Exit, Error> {
    let mut exit = Exit::No;

    match action {
        TerminalAction::Command => {
            disable_raw_mode()?;
            exit = command(stdio, state, setting)?;
            enable_raw_mode()?;
        }
        TerminalAction::Exit => {
            command_exit(stdio)?;
            exit = Exit::Yes;
        }
        TerminalAction::HistoryNext => history_get(&HistoryDirection::Next, stdio, state, bell)?,
        TerminalAction::HistoryPrev => history_get(&HistoryDirection::Prev, stdio, state, bell)?,
        TerminalAction::InputAdd(symbol) => input_add(symbol.to_string().as_str(), stdio, state)?,
        TerminalAction::InputSub => input_sub(stdio, state)?,
        TerminalAction::InputComplete => input_complete(stdio, state, setting, bell)?,
    };

    Ok(exit)
}

pub enum Exit {
    Yes,
    No,
}

impl PartialEq for Exit {
    fn eq(&self, other: &Exit) -> bool {
        match (self, other) {
            (Exit::Yes, Exit::Yes) => true,
            (Exit::No, Exit::No) => true,
            _ => false,
        }
    }
}
