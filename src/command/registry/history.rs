use crate::command::fmt::NewLine;
use crate::history::{download as history_download, upload as history_upload, Log};
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

    let (count, loaders) = validated.unwrap();

    if let Some(loaders) = loaders {
        load_mode(stdio, newline, log, loaders)
    } else {
        print_mode(stdio, newline, log, count)
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
    loaders: Vec<Loader>,
) -> Result<(), Error> {
    for loader in loaders {
        match loader.operation {
            Operation::Download => download(stdio, newline, log, loader.file_path.as_str())?,
            Operation::Upload(UploadMode::Rewrite) => {
                history_upload(log, loader.file_path.as_str(), false)?
            }
            Operation::Upload(UploadMode::Append) => {
                history_upload(log, loader.file_path.as_str(), true)?
            }
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

    let download = history_download(log, file_path);

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

fn validate(args: Option<&Vec<&str>>) -> Result<(Option<usize>, Option<Vec<Loader>>), Error> {
    const LOAD_FLAGS: [&str; 3] = ["-r", "-w", "-a"];
    const MODE_NOT_DEFINED: u8 = 0;
    const MODE_PRINT: u8 = 1;
    const MODE_LOAD: u8 = 2;
    let mut mode = MODE_NOT_DEFINED;

    let err = Error::other("incorrect arguments");
    let mut loaders: Option<Vec<Loader>> = None;
    let mut count: Option<usize> = None;

    if args.is_none() {
        return Ok((count, loaders));
    }

    let mut iter = args.unwrap().into_iter();
    while let Some(arg) = iter.next() {
        let arg = *arg;

        if let Ok(parsed) = arg.parse::<usize>() {
            if mode != MODE_NOT_DEFINED {
                return Err(err);
            }

            mode = MODE_PRINT;
            count = Some(parsed);
        } else if LOAD_FLAGS.contains(&arg) {
            if ![MODE_NOT_DEFINED, MODE_LOAD].contains(&mode) {
                return Err(err);
            }

            mode = MODE_LOAD;
            let operation = Operation::try_from(arg)?;
            let file_path = iter.next();

            if file_path.is_none() {
                return Err(err);
            }

            let loader = Loader::new(file_path.unwrap().to_string(), operation);

            loaders = if let Some(mut items) = loaders {
                items.push(loader);
                Some(items)
            } else {
                Some(vec![loader])
            };
        } else {
            return Err(err);
        }
    }

    Ok((count, loaders))
}

struct Loader {
    file_path: String,
    operation: Operation,
}

impl Loader {
    fn new(file_path: String, operation: Operation) -> Loader {
        Loader {
            file_path,
            operation,
        }
    }
}

enum Operation {
    Download,
    Upload(UploadMode),
}

impl Operation {
    fn try_from(flag: &str) -> Result<Operation, Error> {
        match flag {
            "-r" => Ok(Operation::Download),
            "-w" => Ok(Operation::Upload(UploadMode::Rewrite)),
            "-a" => Ok(Operation::Upload(UploadMode::Append)),
            _ => Err(Error::other("not supported")),
        }
    }
}

enum UploadMode {
    Rewrite,
    Append,
}
