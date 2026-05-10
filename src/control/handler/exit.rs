use crate::io::Stdio;
use std::io::{Error, Write};

pub fn handle(stdio: &mut Stdio) -> Result<bool, Error> {
    write!(stdio.stdout(), "^C")?;
    stdio.stdout().flush()?;

    Ok(true)
}
