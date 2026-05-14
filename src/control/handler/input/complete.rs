use crate::io::Stdio;
use crate::keyboard::TerminalAction;
use crate::session::State;
use crate::{complete::complete_input, setting::Setting};
use std::io::{Error, Write};

pub fn handle(stdio: &mut Stdio, state: &mut State, setting: &Setting) -> Result<(), Error> {
    let input = state.terminal().input().get();

    if input.is_none() {
        return not_found(stdio);
    }

    let completion = complete_input(input.unwrap(), setting);

    if completion.is_none() {
        return not_found(stdio);
    }

    let completion = completion.unwrap();
    let variants = completion.get_variants();

    if variants.is_none() {
        return one_found(completion.get_selected().unwrap(), stdio, state);
    }

    if let Some(previous_action) = state.previous_action() {
        if matches!(previous_action, TerminalAction::InputComplete) {
            let input = state.terminal().input().get();
            return more_found(input.unwrap(), variants.unwrap(), stdio);
        }
    }

    not_found(stdio)
}

fn more_found(current: &str, found: Vec<&str>, stdio: &mut Stdio) -> Result<(), Error> {
    let to_print = format!("\r\n{}\r\n$ {}", found.join("  "), current);

    write!(stdio.stdout(), "{}", to_print)?;
    stdio.stdout().flush()?;

    Ok(())
}

fn one_found(found: &str, stdio: &mut Stdio, state: &mut State) -> Result<(), Error> {
    write!(stdio.stdout(), "{}", found)?;
    stdio.stdout().flush()?;

    state.terminal().input().push_as_system(found);

    Ok(())
}

fn not_found(stdio: &mut Stdio) -> Result<(), Error> {
    write!(stdio.stdout(), "\x07")?;
    stdio.stdout().flush()?;

    Ok(())
}
