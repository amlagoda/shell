use crate::fmt::NewLine;
use crate::io::Stdio;
use std::io::{Error, Write};

pub fn run_command(
    stdio: &mut Stdio,
    newline: &NewLine,
    args: Option<&Vec<&str>>,
) -> Result<(), Error> {
    Ok(())
}
