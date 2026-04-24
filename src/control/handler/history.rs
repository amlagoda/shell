use crate::cursor::move_left as cursor_move_left;
use crate::history::Log;
use crate::io::Stdio;
use crate::session::State;
use std::io::{Error, Write};

pub fn handle(
    direction: &Direction,
    state: &mut State,
    stdio: &mut Stdio,
    log: &mut Log,
) -> Result<(), Error> {
    let input = state.terminal().input();

    if input.has_user_typing() {
        return Ok(());
    }

    let command = match direction {
        Direction::Next => log.next(),
        Direction::Prev => log.prev(),
    };

    if let Some(command) = command {
        if let Some(data) = input.get() {
            cursor_move_left(stdio.stdout(), data.len() as u16)?;
        }

        write!(stdio.stdout(), "{}", command)?;

        input.reset();
        input.push_as_system(command.as_str());
    } else {
        write!(stdio.stdout(), "\x07")?;
    }

    stdio.stdout().flush()?;

    Ok(())
}

pub enum Direction {
    Next,
    Prev,
}
