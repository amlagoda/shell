mod structure;
mod validator;

use crate::command::fmt::NewLine;
use crate::command::registry::history::structure::Loader;
use crate::command::registry::history::validator::validate;
use crate::history::{download as download_log, upload as upload_log, Log};
use crate::io::Stdio;
use std::io::{BufWriter, Error, ErrorKind, Write};

pub fn run_command(
    stdio: &mut Stdio,
    newline: &NewLine,
    log: &mut Log,
    args: Option<&Vec<&str>>,
) -> Result<(), Error> {
    let validated = validate(args);

    if let Err(err) = validated {
        let msg = format!(
            "{}history: {}{}",
            newline.stderr_start(),
            err,
            newline.stderr_end()
        );

        write!(stdio.stderr(), "{}", msg)?;
        stdio.stderr().flush()?;

        return Ok(());
    }

    let validated = validated.unwrap();

    if let Some(loaders) = validated.get_loaders() {
        load_mode(stdio, newline, log, loaders)
    } else {
        print_mode(stdio, newline, log, validated.get_count())
    }
}

fn print_mode(
    stdio: &mut Stdio,
    newline: &NewLine,
    log: &mut Log,
    count: Option<usize>,
) -> Result<(), Error> {
    let (records, len) = log.lasts(count);
    let mut num = log.len() - len;
    let mut buffer = BufWriter::with_capacity(4096, stdio.stdout());

    for (mut iter, command) in records.enumerate() {
        iter += 1;
        num += 1;

        let mut to_print = format!("    {}  {}", num, command);

        if iter == 1 {
            to_print = format!("{}{}", newline.stdout_start(), to_print);
        }

        if iter == len {
            to_print = format!("{}{}", to_print, newline.stdout_end());
        } else {
            to_print = format!("{}\n", to_print);
        }

        buffer.write_all(to_print.as_bytes())?;
    }

    buffer.flush()?;

    Ok(())
}

fn load_mode(
    stdio: &mut Stdio,
    newline: &NewLine,
    log: &mut Log,
    loaders: Vec<&Loader>,
) -> Result<(), Error> {
    for loader in loaders {
        if loader.is_download() {
            download(stdio, newline, log, loader.get_file_path())?;
        } else {
            // is upload
            upload_log(log, loader.get_file_path(), loader.is_upload_append())?;
        }
    }

    Ok(())
}

fn download(
    stdio: &mut Stdio,
    newline: &NewLine,
    log: &mut Log,
    file_path: &str,
) -> Result<(), Error> {
    let msg = format!(
        "{}history: {}: No such file or directory{}",
        newline.stderr_start(),
        file_path,
        newline.stderr_end()
    );

    let download = download_log(log, file_path);

    if download.is_ok() {
        return Ok(());
    }

    let err = download.unwrap_err();

    if err.kind() == ErrorKind::NotFound {
        write!(stdio.stderr(), "{}", msg)?;
        stdio.stderr().flush()?;

        Ok(())
    } else {
        Err(err)
    }
}
