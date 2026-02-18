mod io;
mod process;

use crate::command::fmt::NewLine;
use crate::command::{run_command as run_builtin, to_command as to_builtin};
use crate::core::io::{mass_close as mass_close_pipes, mass_create as mass_create_pipes};
use crate::core::process::{kill_forks, pid, to_group, Fork};
use crate::fs::{open_file, search_executable_file_in_paths as find_bin};
use crate::fs::{to_nonblock_file, transfer_data};
use crate::io::Stdio;
use crate::parser::Parsed;
use std::fs::File;
use std::io::{Error, Write};
use std::os::fd::FromRawFd;

pub fn run(parseds: &Vec<Parsed>, stdio: &mut Stdio, bin_paths: &Vec<&str>) -> Result<bool, Error> {
    let len = parseds.len();

    if len == 0 {
        return Err(Error::other("empty parseds"));
    }

    if len == 1 {
        let parsed = parseds.first().unwrap();

        if let Some(builtin) = to_builtin(parsed.command()) {
            if builtin.is_exit() {
                return Ok(true);
            }

            if !builtin.is_blocking() {
                // native run single, builtin and non-blocking command
                // does not control the "exit"
                run_native(&parsed, stdio, Some(bin_paths))?;
                return Ok(false);
            }
        }
    }

    // other commands run as forks
    // control the "exit"
    run_forks(parseds, stdio, bin_paths)
}

fn run_native(
    parsed: &Parsed,
    stdio: &mut Stdio,
    bin_paths: Option<&Vec<&str>>,
) -> Result<(), Error> {
    let builtin = to_builtin(parsed.command()).ok_or(Error::other("not builtin"))?;
    let args = parsed.args();

    if let Some(redirect) = parsed.redirect() {
        let file = open_file(redirect.path(), redirect.is_append())?;
        let stdin = (*stdio.stdin()).try_clone()?;
        let mut stdout = (*stdio.stdout()).try_clone()?;
        let mut stderr = (*stdio.stderr()).try_clone()?;
        let mut newline = NewLine::new();

        if redirect.is_stderr() {
            stderr = file;
            newline.stderr_end = true;
            newline.stdout_start = true;
        } else {
            stdout = file;
            newline.stdout_end = true;
            newline.stderr_start = true;
        }

        let mut stdio = Stdio::new(stdin, stdout, stderr);

        return run_builtin(&builtin, &mut stdio, &newline, args.as_ref(), bin_paths);
    }

    let mut newline = NewLine::new();
    newline.stdout_start = true;
    newline.stderr_start = true;

    run_builtin(&builtin, stdio, &newline, args.as_ref(), bin_paths)
}

fn run_forks(
    parseds: &Vec<Parsed>,
    stdio: &mut Stdio,
    bin_paths: &Vec<&str>,
) -> Result<bool, Error> {
    let len = parseds.len();
    let mut pipelines = mass_create_pipes(len)?;
    let mut forks: Vec<Fork> = vec![];
    let group_pid = pid();

    for (number, parsed) in parseds.iter().enumerate() {
        let command = parsed.command();

        if to_builtin(command).is_some() || find_bin(command, bin_paths).is_some() {
            let fork = Fork::try_new();

            if let Err(err) = fork {
                mass_close_pipes(pipelines);
                kill_forks(forks);
                return Err(err);
            }

            let fork = fork.unwrap();

            if fork.is_child() {
                to_group(0, group_pid);
                let is_first_command = number == 0;
                let stdout = (&pipelines[number]).write_end();

                if !is_first_command {
                    if let Err(err) = fork.set_stdin((&pipelines[number - 1]).read_end()) {
                        mass_close_pipes(pipelines);
                        return Err(err);
                    }
                }

                if let Err(err) = fork.set_stdout(stdout) {
                    mass_close_pipes(pipelines);
                    kill_forks(forks);
                    return Err(err);
                }

                if let Err(err) = fork.set_stderr(stdout) {
                    mass_close_pipes(pipelines);
                    kill_forks(forks);
                    return Err(err);
                }

                mass_close_pipes(pipelines);

                if let Some(builtin) = to_builtin(command) {
                    let mut stdio = unsafe {
                        Stdio::new(
                            File::from_raw_fd(1),
                            File::from_raw_fd(2),
                            File::from_raw_fd(2),
                        )
                    };

                    run_builtin(
                        &builtin,
                        &mut stdio,
                        &NewLine::new(), // all \r\n disabled
                        parsed.args().as_ref(),
                        Some(bin_paths),
                    )?;

                    //always return exit=true after the fork is completed
                    return Ok(true);
                } else {
                    // any return value is a error, which is equivalent to exit=true
                    return Err(fork.hot_reload_bin(parsed.command(), parsed.args()));
                }
            }

            forks.push(fork);
        } else {
            // что закрываем/удаляем?
            let msg = format!("{}: command not found", command);
            write!(stdio.stderr(), "\r\n{}", msg)?; // NewLine?
            stdio.stderr().flush()?;
        }
    }

    if forks.is_empty() {
        mass_close_pipes(pipelines);
        return Ok(false);
    }

    let mut last_read_end = 0;
    for (number, pipeline) in pipelines.iter_mut().enumerate() {
        if number == len - 1 {
            last_read_end = pipeline.read_end();
        }
        pipeline.close_write_end();
    }

    let file = to_nonblock_file(last_read_end);

    if let Err(err) = file {
        mass_close_pipes(pipelines);
        kill_forks(forks);
        return Err(err);
    }

    let mut file = file.unwrap();

    if let Err(err) = transfer_data(&mut file, stdio.stdout()) {
        mass_close_pipes(pipelines);
        kill_forks(forks);
        return Err(err);
    }

    pipelines.pop();
    mass_close_pipes(pipelines);
    forks.last().unwrap().blocking_waiting();
    kill_forks(forks);

    Ok(false)
}
