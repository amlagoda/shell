use crate::command::fmt::NewLine;
use crate::fs::{get_read_file, get_write_file};
use crate::io::Stdio;
use crate::storage::History;
use std::io::{BufRead, BufReader, BufWriter, Error, Write};

pub fn run_command(
    stdio: &mut Stdio,
    newline: &NewLine,
    history: &mut History,
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

    let (limit, loaders) = validated.unwrap();

    if let Some(loaders) = loaders {
        load_mode(stdio, newline, history, loaders)
    } else {
        print_mode(stdio, newline, history, limit)
    }
}

fn print_mode(
    stdio: &mut Stdio,
    newline: &NewLine,
    history: &mut History,
    limit: Option<usize>,
) -> Result<(), Error> {
    let history = history.all();

    if history.is_none() {
        return Ok(());
    }

    let mut records = history.unwrap();
    let mut buffer = BufWriter::with_capacity(4096, stdio.stdout());
    let len_start = records.len();
    let mut len_limit = len_start;
    let mut num = 1;

    if let Some(limit) = limit {
        records = records.into_iter().rev().take(limit).rev().collect();
        len_limit = records.len();

        if limit > 0 && limit < len_start {
            num = len_start - (limit - 1);
        }
    }

    for (mut iter, command) in records.iter().enumerate() {
        iter += 1;
        let mut to_print = format!("    {}  {}", num, command);

        if iter == 1 {
            to_print = format!("{}{}", newline.stdout_start(), to_print);
        }

        if iter == len_limit {
            to_print = format!("{}{}", to_print, newline.stdout_end());
        } else {
            to_print = format!("{}\n", to_print);
        }

        buffer.write_all(to_print.as_bytes())?;
        num += 1;
    }

    buffer.flush()?;

    Ok(())
}

fn load_mode(
    stdio: &mut Stdio,
    newline: &NewLine,
    history: &mut History,
    loaders: Vec<Loader>,
) -> Result<(), Error> {
    for loader in loaders {
        match loader.operation {
            Operation::Download => download(stdio, newline, history, loader.file_path.as_str())?,
            Operation::Upload(UploadMode::Rewrite) => {
                upload(history, loader.file_path.as_str(), false)?
            }
            Operation::Upload(UploadMode::Append) => {
                upload(history, loader.file_path.as_str(), true)?
            }
        }
    }

    Ok(())
}

fn download(
    stdio: &mut Stdio,
    newline: &NewLine,
    history: &mut History,
    file_path: &str,
) -> Result<(), Error> {
    let file = get_read_file(file_path);

    if file.is_err() {
        let msg = format!(
            "{}history: {}: No such file or directory{}",
            newline.stderr_start(),
            file_path,
            newline.stderr_end()
        );

        write!(stdio.stderr(), "{}", msg)?;
        stdio.stderr().flush()?;

        return Ok(());
    }

    let buffer = BufReader::with_capacity(4096, file.unwrap());
    for line in buffer.lines() {
        history.add(line?);
    }

    Ok(())
}

fn upload(history: &mut History, file_path: &str, append: bool) -> Result<(), Error> {
    let history = history.all();

    if history.is_none() {
        return Ok(());
    }

    let records = history.unwrap();
    let mut file = get_write_file(file_path, append)?;
    let mut buffer = BufWriter::with_capacity(4096, &mut file);

    for record in records {
        buffer.write_all(format!("{}\n", record).as_bytes())?;
    }

    buffer.flush()?;

    Ok(())
}

fn validate(args: Option<&Vec<&str>>) -> Result<(Option<usize>, Option<Vec<Loader>>), Error> {
    const LOAD_FLAGS: [&str; 3] = ["-r", "-w", "-a"];
    const MODE_NOT_DEFINED: u8 = 0;
    const MODE_PRINT: u8 = 1;
    const MODE_LOAD: u8 = 2;
    let mut mode = MODE_NOT_DEFINED;

    let err = Error::other("incorrect arguments");
    let mut loaders: Option<Vec<Loader>> = None;
    let mut limit: Option<usize> = None;

    if args.is_none() {
        return Ok((limit, loaders));
    }

    let mut iter = args.unwrap().into_iter();
    while let Some(arg) = iter.next() {
        let arg = *arg;

        if let Ok(parsed) = arg.parse::<usize>() {
            if mode != MODE_NOT_DEFINED {
                return Err(err);
            }

            mode = MODE_PRINT;
            limit = Some(parsed);
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

    Ok((limit, loaders))
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
