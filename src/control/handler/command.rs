use crate::control::Exit;
use crate::core::run;
use crate::fmt::new_line_raw;
use crate::io::Stdio;
use crate::parser::parse;
use crate::session::State;
use crate::setting::Setting;
use std::io::{Error, Write};

pub fn handle(stdio: &mut Stdio, state: &mut State, setting: &Setting) -> Result<Exit, Error> {
    let input = state.input().get();

    if input.is_none() {
        print_newline(stdio, setting)?;

        return Ok(Exit::No);
    }

    let parseds = parse(input.unwrap())?;

    if parseds.is_none() {
        print_newline(stdio, setting)?;

        return Ok(Exit::No);
    }

    let exit = run(
        &parseds.unwrap().iter().collect(),
        stdio,
        state.history(),
        setting,
    )?;

    state.input().reset();
    state.history().reset();

    if matches!(exit, Exit::No) {
        print_newline(stdio, setting)?;
    }

    Ok(exit)
}

fn print_newline(stdio: &mut Stdio, setting: &Setting) -> Result<(), Error> {
    if setting.is_interactive_mode() {
        write!(stdio.stdout(), "{}$ ", new_line_raw())?;
    } else {
        write!(stdio.stdout(), "{}", new_line_raw())?;
    }

    stdio.stdout().flush()?;

    Ok(())
}
