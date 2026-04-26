mod handler;

use self::handler::{command, exit, history as history_get, HistoryDirection};
use self::handler::{input_add, input_complete, input_sub};
use crate::command::get_command_list;
use crate::history::History;
use crate::io::Stdio;
use crate::keyboard::{to_action, TerminalAction};
use crate::session::State;
use crate::setting::Setting;
use crossterm::event::{read as get_pressed_key, KeyEvent};
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

    loop {
        let pressed_key = get_pressed_key()?.as_key_event();

        if pressed_key.is_none() {
            continue;
        }

        let available_commands = get_command_list();
        let available_commands = available_commands.iter().map(|r| r.as_str()).collect();

        let need_exit = run_handler(
            state,
            stdio,
            history,
            setting,
            &pressed_key.unwrap(),
            &available_commands,
        )?;

        state.keyboard().set_previous_key(pressed_key.unwrap());

        if need_exit {
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
    pressed_key: &KeyEvent,
    available_commands: &Vec<&str>,
) -> Result<bool, Error> {
    let mut need_exit = false;
    let action = to_action(pressed_key);

    if action.is_none() {
        return Ok(need_exit);
    }

    match action.unwrap() {
        TerminalAction::Command => need_exit = command(stdio, state, history, setting)?,

        TerminalAction::Exit => {
            exit(stdio)?;
            need_exit = true;
        }
        TerminalAction::HistoryNext => history_get(&HistoryDirection::Next, state, stdio, history)?,
        TerminalAction::HistoryPrev => history_get(&HistoryDirection::Prev, state, stdio, history)?,
        TerminalAction::InputAdd(symbol) => input_add(symbol.to_string().as_str(), state, stdio)?,
        TerminalAction::InputSub => input_sub(state, stdio)?,
        TerminalAction::InputComplete => input_complete(state, stdio, setting, available_commands)?,
    };

    Ok(need_exit)
}
