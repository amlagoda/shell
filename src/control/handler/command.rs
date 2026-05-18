use crate::core::run;
use crate::history::History;
use crate::io::Stdio;
use crate::parser::parse;
use crate::session::State;
use crate::setting::Setting;
use std::io::{Error, Write};

pub fn handle(
    stdio: &mut Stdio,
    state: &mut State,
    history: &mut History,
    setting: &Setting,
) -> Result<bool, Error> {
    let input = state.input().get();

    if input.is_none() {
        print_newline(stdio, setting)?;

        return Ok(false);
    }

    let parseds = parse(input.unwrap())?;

    if parseds.is_none() {
        print_newline(stdio, setting)?;

        return Ok(false);
    }

    let is_exit = run(&parseds.unwrap().iter().collect(), stdio, history, setting)?;

    state.input().reset();
    history.reset();

    if !is_exit {
        print_newline(stdio, setting)?;
    }

    Ok(is_exit)
}

fn print_newline(stdio: &mut Stdio, setting: &Setting) -> Result<(), Error> {
    if setting.is_interactive_mode() {
        write!(stdio.stdout(), "\r\n$ ")?;
    } else {
        write!(stdio.stdout(), "\r\n")?;
    }

    stdio.stdout().flush()?;

    Ok(())
}
