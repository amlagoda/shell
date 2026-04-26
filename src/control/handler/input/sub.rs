use crate::cursor::move_left as cursor_move_left;
use crate::io::Stdio;
use crate::session::State;
use std::io::Error;

pub fn handle(stdio: &mut Stdio, state: &mut State) -> Result<(), Error> {
    let input = state.terminal().input();

    if input.get().is_some() {
        cursor_move_left(stdio.stdout(), 1)?;
        input.remove_last(1);
    }

    Ok(())
}
