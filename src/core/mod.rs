mod io;

use crate::command::fmt::NewLine;
use crate::command::{run_command as run_builtin, to_command as to_builtin};
use crate::core::io::create_pipe;
use crate::core::io::{mass_close as mass_close_pipes, mass_create as mass_create_pipes};
use crate::fs::{find_file, get_write_file};
use crate::fs::{to_independent_file, to_nonblock_file, transfer_data};
use crate::history::Log;
use crate::io::Stdio;
use crate::parser::Parsed;
use crate::process::{kill_forks, pid, to_group, Fork};
use std::fs::File;
use std::io::{Error, Write};
use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};

pub fn run(
    parseds: &Vec<&Parsed>,
    stdio: &mut Stdio,
    log: &mut Log,
    bin_paths: &Vec<&str>,
    output_starts_newline: bool,
) -> Result<bool, Error> {
    let len = parseds.len();

    if len == 0 {
        return Err(Error::other("empty parseds"));
    }

    to_log(parseds, log);

    if len == 1 {
        let parsed = parseds.first().unwrap();

        if let Some(builtin) = to_builtin(parsed.command()) {
            if builtin.is_exit() {
                return Ok(true);
            }

            if !builtin.is_blocking() {
                // native run single, builtin and non-blocking command
                // does not control the "exit"
                run_native(parsed, stdio, log, Some(bin_paths), output_starts_newline)?;
                return Ok(false);
            }
        }
    }

    // other commands run as forks
    // control the "exit"
    run_forks(parseds, stdio, log, bin_paths, output_starts_newline)
}

fn to_log(parseds: &Vec<&Parsed>, log: &mut Log) {
    let mut to_log = Vec::with_capacity(10);

    for parsed in parseds {
        let command = parsed.command().to_string();

        let args = parsed
            .args()
            .map_or_else(|| "".to_string(), |args| format!(" {}", args.join(" ")));

        to_log.push(format!("{}{}", command, args));
    }

    log.add(to_log);
}

fn run_native(
    parsed: &Parsed,
    stdio: &mut Stdio,
    log: &mut Log,
    bin_paths: Option<&Vec<&str>>,
    output_starts_newline: bool,
) -> Result<(), Error> {
    let builtin = to_builtin(parsed.command()).ok_or(Error::other("not builtin"))?;
    let args = parsed.args();

    if let Some(redirect) = parsed.redirect() {
        let file = get_write_file(redirect.path(), redirect.is_append())?;
        let stdin = (*stdio.stdin()).try_clone()?;
        let mut stdout = (*stdio.stdout()).try_clone()?;
        let mut stderr = (*stdio.stderr()).try_clone()?;
        let mut newline = NewLine::new();

        if redirect.is_stderr() {
            stderr = file;
            newline.set_stderr_end(true);
            newline.set_stdout_start(true);
        } else {
            stdout = file;
            newline.set_stdout_end(true);
            newline.set_stderr_start(true);
        }

        let mut stdio = Stdio::new(stdin, stdout, stderr);

        return run_builtin(
            &builtin,
            args.as_ref(),
            &mut stdio,
            log,
            &newline,
            bin_paths,
        );
    }

    let mut newline = NewLine::new();
    newline.set_stdout_start(output_starts_newline);
    newline.set_stderr_start(output_starts_newline);

    run_builtin(&builtin, args.as_ref(), stdio, log, &newline, bin_paths)
}

fn run_forks(
    parseds: &Vec<&Parsed>,
    stdio: &mut Stdio,
    log: &mut Log,
    bin_paths: &Vec<&str>,
    output_starts_newline: bool,
) -> Result<bool, Error> {
    let mut pipeline_stderr = create_pipe()?;
    let mut pipelines_stdout = mass_create_pipes(count_pipes(parseds))?;
    let mut forks: Vec<Fork> = vec![];
    let group_pid = pid();
    let mut number = 0;

    for parsed in parseds {
        let command = parsed.command();
        let only_executable = true;

        if to_builtin(command).is_some() || find_file(command, bin_paths, only_executable).is_some()
        {
            let fork = Fork::try_new();

            if let Err(err) = fork {
                mass_close_pipes(pipelines_stdout);
                pipeline_stderr.close();
                kill_forks(forks);
                return Err(err);
            }

            let fork = fork.unwrap();

            if fork.is_child() {
                to_group(0, group_pid);
                let is_first_command = number == 0;
                let stdout = pipelines_stdout[number].write_end();

                if !is_first_command {
                    if let Err(err) = fork.set_stdin(pipelines_stdout[number - 1].read_end()) {
                        mass_close_pipes(pipelines_stdout);
                        pipeline_stderr.close();
                        return Err(err);
                    }
                }

                if let Err(err) = fork.set_stdout(stdout) {
                    mass_close_pipes(pipelines_stdout);
                    pipeline_stderr.close();
                    return Err(err);
                }

                if let Err(err) = fork.set_stderr(pipeline_stderr.write_end()) {
                    mass_close_pipes(pipelines_stdout);
                    pipeline_stderr.close();
                    return Err(err);
                }

                if let Some(redirect) = parsed.redirect() {
                    if redirect.is_stderr() || (redirect.is_stdout() && parsed.pipeline().is_none())
                    {
                        let file = get_write_file(redirect.path(), redirect.is_append());

                        if let Err(err) = file {
                            mass_close_pipes(pipelines_stdout);
                            pipeline_stderr.close();
                            return Err(err);
                        }

                        let file_descriptor = file.unwrap().into_raw_fd() as u32;

                        let status = if redirect.is_stdout() {
                            fork.set_stdout(file_descriptor)
                        } else {
                            fork.set_stderr(file_descriptor)
                        };

                        if let Err(err) = status {
                            mass_close_pipes(pipelines_stdout);
                            pipeline_stderr.close();
                            return Err(err);
                        }
                    }
                }

                mass_close_pipes(pipelines_stdout);
                pipeline_stderr.close();

                if let Some(builtin) = to_builtin(command) {
                    let mut stdio = unsafe {
                        Stdio::new(
                            File::from_raw_fd(0),
                            File::from_raw_fd(1),
                            File::from_raw_fd(2),
                        )
                    };

                    let mut newline = NewLine::new();
                    newline.set_stdout_end(true);
                    newline.set_stderr_end(true);

                    run_builtin(
                        &builtin,
                        parsed.args().as_ref(),
                        &mut stdio,
                        log,
                        &newline,
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

            if parsed.pipeline().is_some() && parsed.redirect().is_some() {
                let redirect = parsed.redirect().unwrap();

                if redirect.is_stdout() {
                    number += 1;
                    let fork = Fork::try_new();

                    if let Err(err) = fork {
                        mass_close_pipes(pipelines_stdout);
                        pipeline_stderr.close();
                        kill_forks(forks);
                        return Err(err);
                    }

                    let fork = fork.unwrap();

                    if fork.is_child() {
                        to_group(0, group_pid);

                        if let Err(err) = fork.set_stdin(pipelines_stdout[number - 1].read_end()) {
                            mass_close_pipes(pipelines_stdout);
                            pipeline_stderr.close();
                            return Err(err);
                        }

                        if let Err(err) = fork.set_stdout(pipelines_stdout[number].write_end()) {
                            mass_close_pipes(pipelines_stdout);
                            pipeline_stderr.close();
                            return Err(err);
                        }

                        if let Err(err) = fork.set_stderr(pipeline_stderr.write_end()) {
                            mass_close_pipes(pipelines_stdout);
                            pipeline_stderr.close();
                            return Err(err);
                        }

                        mass_close_pipes(pipelines_stdout);
                        pipeline_stderr.close();

                        let mut args = vec![redirect.path()];

                        if redirect.is_append() {
                            args.push("-a");
                        }

                        args.reverse(); // required

                        // any return value is a error, which is equivalent to exit=true
                        return Err(fork.hot_reload_bin("tee", Some(args)));
                    }

                    forks.push(fork);
                }
            }
        } else {
            let msg = format!("{}: command not found", command);
            if output_starts_newline {
                write!(stdio.stderr(), "\r\n{}", msg)?;
            } else {
                write!(stdio.stderr(), "{}", msg)?;
            }

            stdio.stderr().flush()?;
        }

        number += 1;
    }

    if forks.is_empty() {
        mass_close_pipes(pipelines_stdout);
        pipeline_stderr.close();
        return Ok(false);
    }

    pipeline_stderr.close_write_end();
    for pipeline_stdout in pipelines_stdout.iter_mut() {
        pipeline_stdout.close_write_end();
    }

    let proceed = Arc::new(AtomicBool::new(true));
    let datasets = transfer_datasets(
        pipelines_stdout.last().unwrap().read_end(),
        pipeline_stderr.read_end(),
        stdio,
        &proceed,
    );

    if let Err(err) = datasets {
        kill_forks(forks);
        mass_close_pipes(pipelines_stdout);
        pipeline_stderr.close();
        return Err(err);
    }

    let memory_ordering = Ordering::Relaxed;
    let mut handlers: Vec<JoinHandle<Result<(), Error>>> = vec![];
    for (from, to, proceed) in datasets.unwrap() {
        // required transfer ownership
        handlers.push(spawn(move || {
            transfer_data(from, to, proceed, !output_starts_newline, memory_ordering)
        }));
    }

    forks.pop().unwrap().blocking_waiting();
    kill_forks(forks);
    proceed.store(false, memory_ordering);

    for handler in handlers {
        match handler.join() {
            Ok(Err(err)) => eprintln!("thread error: {}", err),
            Err(err) => eprintln!("thread panic: {:?}", err),
            _ => {}
        }
    }

    pipelines_stdout.pop();
    mass_close_pipes(pipelines_stdout);
    pipeline_stderr.close();

    Ok(false)
}

fn transfer_datasets(
    stdout: u32,
    stderr: u32,
    stdio: &mut Stdio,
    proceed: &Arc<AtomicBool>,
) -> Result<[(File, File, Arc<AtomicBool>); 2], Error> {
    let stdout_from = to_nonblock_file(stdout)?;
    let stderr_from = to_nonblock_file(stderr)?;

    let stdout_to = to_independent_file(stdio.stdout().as_raw_fd() as u32);
    let stderr_to = to_independent_file(stdio.stderr().as_raw_fd() as u32);

    let (stdout_proceed, stderr_proceed) = (Arc::clone(proceed), Arc::clone(proceed));

    Ok([
        (stdout_from, stdout_to, stdout_proceed),
        (stderr_from, stderr_to, stderr_proceed),
    ])
}

fn count_pipes(parseds: &Vec<&Parsed>) -> usize {
    let mut count = parseds.len();

    for parsed in parseds {
        if parsed.pipeline().is_some() && parsed.redirect().is_some() {
            let redirect = parsed.redirect().unwrap();

            if redirect.is_stdout() {
                count += 1;
            }
        }
    }

    count
}
