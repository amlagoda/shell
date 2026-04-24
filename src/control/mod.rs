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

fn run(
    key: &KeyEvent,
    state: &mut State,
    stdio: &mut Stdio,
    log: &mut Log,
    commands: &Vec<&str>,
    bin_paths: &Vec<&str>,
    current_dir: &str,
) -> Result<Exit, Error> {
    let mut exit = Exit::No;
    let action = to_action(key);

    if action.is_none() {
        return Ok(exit);
    }

    match action.unwrap() {
        TerminalAction::Command => exit = Exit::from(command(stdio, state, log, bin_paths)?),
        TerminalAction::Exit => exit = Exit::Yes,
        TerminalAction::HistoryNext => history(&HistoryDirection::Next, state, stdio, log)?,
        TerminalAction::HistoryPrev => history(&HistoryDirection::Prev, state, stdio, log)?,
        TerminalAction::InputAdd(symbol) => input_add(symbol.to_string().as_str(), state, stdio)?,
        TerminalAction::InputSub => input_sub(state, stdio)?,
        TerminalAction::InputComplete => {
            input_complete(state, stdio, commands, bin_paths, current_dir)?
        }
    };

    Ok(exit)
}

pub enum Exit {
    Yes,
    No,
}

impl Exit {
    fn from(is_exit: bool) -> Exit {
        if is_exit {
            Exit::Yes
        } else {
            Exit::No
        }
    }
}
