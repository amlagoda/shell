use crate::cursor::move_left as cursor_move_left;
use crate::io::Stdio;
use crate::session::State;
use std::io::{Error, Write};

pub fn handle(
    direction: &Direction,
    stdio: &mut Stdio,
    state: &mut State,
    bell: &str,
) -> Result<(), Error> {
    if state.input().has_user_typing() {
        return Ok(());
    }

    let command = match direction {
        Direction::Next => state.history().next(),
        Direction::Prev => state.history().prev(),
    };

    if let Some(command) = command {
        if let Some(data) = state.input().get() {
            cursor_move_left(stdio.stdout(), data.len() as u16)?;
        }

        write!(stdio.stdout(), "{}", command)?;

        state.input().reset();
        state.input().push_as_system(command.as_str());
    } else {
        write!(stdio.stdout(), "{}", bell)?;
    }

    stdio.stdout().flush()?;

    Ok(())
}

pub enum Direction {
    Next,
    Prev,
}
