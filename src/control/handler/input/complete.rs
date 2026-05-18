use crate::fmt::new_line_raw;
use crate::io::Stdio;
use crate::keyboard::TerminalAction;
use crate::session::State;
use crate::{complete::complete_input, setting::Setting};
use std::io::{Error, Write};

pub fn handle(
    stdio: &mut Stdio,
    state: &mut State,
    setting: &Setting,
    bell: &str,
) -> Result<(), Error> {
    let input = state.input().get();

    if input.is_none() {
        return not_found(stdio, bell);
    }

    let completion = complete_input(input.unwrap(), setting);

    if completion.is_none() {
        return not_found(stdio, bell);
    }

    let completion = completion.unwrap();
    let variants = completion.get_variants();

    if variants.is_none() {
        return one_found(stdio, state, completion.get_selected().unwrap());
    }

    if let Some(previous_action) = state.previous_action() {
        if matches!(previous_action, TerminalAction::InputComplete) {
            let input = state.input().get();
            return more_found(stdio, input.unwrap(), variants.unwrap());
        }
    }

    not_found(stdio, bell)
}

fn more_found(stdio: &mut Stdio, current: &str, found: Vec<&str>) -> Result<(), Error> {
    let to_print = format!(
        "{}{}{}$ {}",
        new_line_raw(),
        found.join("  "),
        new_line_raw(),
        current
    );

    write!(stdio.stdout(), "{}", to_print)?;
    stdio.stdout().flush()?;

    Ok(())
}

fn one_found(stdio: &mut Stdio, state: &mut State, found: &str) -> Result<(), Error> {
    write!(stdio.stdout(), "{}", found)?;
    stdio.stdout().flush()?;

    state.input().push_as_system(found);

    Ok(())
}

fn not_found(stdio: &mut Stdio, bell: &str) -> Result<(), Error> {
    write!(stdio.stdout(), "{}", bell)?;
    stdio.stdout().flush()?;

    Ok(())
}
