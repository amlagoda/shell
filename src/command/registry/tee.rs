use crate::command::registry::PrintFact;
use crate::command::Stdio;
use crate::fs::open_file;
use std::io::{Error, ErrorKind, Read, Write};
use std::thread::sleep;
use std::time::Duration;

pub fn run_command(
    stdio: &mut Stdio,
    file_path: &str,
    file_append: bool,
) -> Result<PrintFact, Error> {
    let mut file = open_file(file_path, file_append)?;
    let mut buffer = [0; 4096];

    loop {
        match stdio.stdin().read(&mut buffer) {
            Ok(read_bytes) => {
                if read_bytes == 0 {
                    break;
                }

                let readed = String::from_utf8(buffer[..=read_bytes].to_vec())
                    .map_err(|err| Error::other(err.to_string()))?;

                write!(file, "{}", &readed)?;
                write!(stdio.stdout(), "{}", readed)?;
                stdio.stdout().flush()?;

                buffer = [0; 4096];
            }
            Err(err) => {
                if err.kind() == ErrorKind::WouldBlock {
                    sleep(Duration::from_millis(10));
                    continue;
                }

                return Err(err);
            }
        }
    }

    Ok(PrintFact::new(true, false))
}
