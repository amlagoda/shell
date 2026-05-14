use crate::io::Stdio;
use crate::session::State;
use std::io::{Error, Write};

pub fn handle(data: &str, stdio: &mut Stdio, state: &mut State) -> Result<(), Error> {
    write!(stdio.stdout(), "{}", data)?;
    stdio.stdout().flush()?;

    state.input().push_as_user(data);

    Ok(())
}
