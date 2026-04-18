use crate::io::Stdio;
use crate::session::State;
use std::io::{Error, Write};

pub fn handle(data: &str, state: &mut State, stdio: &mut Stdio) -> Result<(), Error> {
    write!(stdio.stdout(), "{}", data)?;
    stdio.stdout().flush()?;

    state.terminal().input().push_as_user(data);

    Ok(())
}
