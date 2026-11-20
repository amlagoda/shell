use crate::command::{run_command as run_builtin, to_command as to_builtin};
use crate::control::Exit;
use crate::fmt::NewLine;
use crate::fs::{clone_descriptor_as_file, clone_descriptor_as_nonblock_file, transfer_data};
use crate::fs::{find_bins_by_name, get_write_file};
use crate::history::{download_from_parseds as download_to_history, History};
use crate::io::Stdio;
use crate::parser::Parsed;
use crate::pipeline::{create_pipe, mass_create_pipes, Pipeline, PipelinesResult};
use crate::process::{pid, to_group, Fork};
use crate::setting::Setting;
use std::fs::File;
use std::io::{Error, Write};
use std::mem::drop;
use std::os::fd::{AsRawFd, IntoRawFd};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};

pub fn run(
    parseds: &Vec<&Parsed>,
    stdio: &mut Stdio,
    history: &mut History,
    setting: &Setting,
) -> Result<Exit, Error> {
    let len = parseds.len();

    if len == 0 {
        return Err(Error::other("empty parseds"));
    }

    download_to_history(history, parseds);

    if len == 1 {
        let parsed = parseds.first().unwrap();

        if let Some(builtin) = to_builtin(parsed.command()) {
            if builtin.is_exit() {
                return Ok(Exit::Yes);
            }

            if !builtin.is_blocking() {
                // native run single, builtin and non-blocking command
                // does not control the "exit"
                run_native(
                    parsed,
                    stdio,
                    history,
                    setting.new_line(),
                    Some(&setting.bin_paths()),
                )?;

                return Ok(Exit::No);
            }
        }
    }

    // other commands run as forks
    // control the "exit"
    run_forks(
        parseds,
        stdio,
        history,
        setting.new_line(),
        &setting.bin_paths(),
    )
}

fn run_native(
    parsed: &Parsed,
    stdio: &mut Stdio,
    history: &mut History,
    newline: &NewLine,
    bin_paths: Option<&Vec<&str>>,
) -> Result<(), Error> {
    let builtin = to_builtin(parsed.command()).ok_or(Error::other("not builtin"))?;
    let args = parsed.args();

    if let Some(redirect) = parsed.redirect() {
        let file = get_write_file(redirect.path(), redirect.is_append())?;
        let stdin = (*stdio.stdin()).try_clone()?;
        let mut stdout = (*stdio.stdout()).try_clone()?;
        let mut stderr = (*stdio.stderr()).try_clone()?;
        let mut newline = NewLine::default();

        if redirect.is_stderr() {
            stderr = file;
            newline.set_stderr_end(true);
            newline.set_stdout_start(true);
        } else {
            stdout = file;
            newline.set_stdout_end(true);
            newline.set_stderr_start(true);
        }

        // try_clone (dup) creates new descriptors with new
        // numbers, so there is no double closing of the same
        // descriptors when first drop the new stdio and
        // then the main stdio
        let mut stdio = Stdio::from(stdin, stdout, stderr);

        return run_builtin(
            &builtin,
            args.as_ref(),
            &mut stdio,
            history,
            &newline,
            bin_paths,
        );
    }

    run_builtin(&builtin, args.as_ref(), stdio, history, newline, bin_paths)
}

fn run_forks(
    parseds: &Vec<&Parsed>,
    stdio: &mut Stdio,
    history: &mut History,
    newline: &NewLine,
    bin_paths: &Vec<&str>,
) -> Result<Exit, Error> {
    let mut pipes_stdout = match mass_create_pipes(count_pipes(parseds)) {
        PipelinesResult::Err(err) => return Err(err),
        PipelinesResult::None => return Err(Error::other("empty parseds")),
        PipelinesResult::Some(pipes) => pipes,
    };

    let mut pipe_stderr = create_pipe()?;
    let mut forks: Vec<Fork> = vec![];
    let group_pid = pid();
    let mut number = 0;

    for parsed in parseds {
        let command = parsed.command();

        if to_builtin(command).is_some() || find_bins_by_name(command, bin_paths).is_some() {
            let mut fork = Fork::try_new()?;

            if fork.is_child() {
                to_group(0, group_pid)?;
                fork.default_sigpipe();

                // before: 0,1,2 in stdio linked to parent descriptors
                set_fork_stdio(
                    &mut fork,
                    number,
                    &pipes_stdout.iter().collect(),
                    &pipe_stderr,
                    &parsed,
                )?;
                // after: 0,1,2 in stdio linked to new, pipeline descriptors
                // old (parent) descriptors automaticly closed by c_dup2

                drop(pipes_stdout);
                drop(pipe_stderr);

                if let Some(builtin) = to_builtin(command) {
                    let mut newline = NewLine::default();
                    newline.set_stdout_end(true);
                    newline.set_stderr_end(true);

                    run_builtin(
                        &builtin,
                        parsed.args().as_ref(),
                        stdio,
                        history,
                        &newline,
                        Some(bin_paths),
                    )?;

                    //always return exit=true after the fork is completed
                    return Ok(Exit::Yes);
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
                    let fork = Fork::try_new()?;

                    if fork.is_child() {
                        to_group(0, group_pid)?;
                        fork.set_stdin(pipes_stdout[number - 1].read_end())?;
                        fork.set_stdout(pipes_stdout[number].write_end())?;
                        fork.set_stderr(pipe_stderr.write_end())?;
                        drop(pipes_stdout);

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
            let msg = format!("{}{}: command not found", newline.stderr_start(), command);
            write!(stdio.stderr(), "{}", msg)?;
            stdio.stderr().flush()?;
        }

        number += 1;
    }

    if forks.is_empty() {
        return Ok(Exit::No);
    }

    pipe_stderr.close_write_end();
    for pipeline_stdout in pipes_stdout.iter_mut() {
        pipeline_stdout.close_write_end();
    }

    let proceed = Arc::new(AtomicBool::new(true));
    let datasets = transfer_datasets(
        pipes_stdout.last().unwrap().read_end(),
        pipe_stderr.read_end(),
        stdio,
        &proceed,
    )?;

    let memory_ordering = Ordering::Relaxed;
    let mut handlers: Vec<JoinHandle<Result<(), Error>>> = vec![];
    let skip_first_newline = newline.stdout_start().is_empty();

    for (from, to, proceed) in datasets {
        // for (from, to, proceed) in datasets.unwrap() {
        // required transfer ownership
        handlers.push(spawn(move || {
            transfer_data(from, to, proceed, skip_first_newline, memory_ordering)
        }));
    }

    forks.pop().unwrap().blocking_waiting();
    proceed.store(false, memory_ordering);

    for handler in handlers {
        match handler.join() {
            Ok(Err(err)) => eprintln!("thread error: {}", err),
            Err(err) => eprintln!("thread panic: {:?}", err),
            _ => {}
        }
    }

    Ok(Exit::No)
}

fn set_fork_stdio(
    fork: &mut Fork,
    fork_number: usize,
    pipes_stdout: &Vec<&Pipeline>,
    pipe_stderr: &Pipeline,
    parsed: &Parsed,
) -> Result<(), Error> {
    let is_first_command = fork_number == 0;
    let stdout = pipes_stdout[fork_number].write_end();

    if !is_first_command {
        fork.set_stdin(pipes_stdout[fork_number - 1].read_end())?;
    }

    fork.set_stdout(stdout)?;
    fork.set_stderr(pipe_stderr.write_end())?;

    let redirect = parsed.redirect();

    if redirect.is_none() {
        return Ok(());
    }

    let redirect = redirect.unwrap();

    if redirect.is_stderr() || (redirect.is_stdout() && parsed.pipeline().is_none()) {
        let file = get_write_file(redirect.path(), redirect.is_append())?;
        let file_descriptor = file.into_raw_fd();

        if redirect.is_stdout() {
            fork.set_stdout(file_descriptor)?;
        } else {
            fork.set_stderr(file_descriptor)?;
        }
    }

    Ok(())
}

// clone stdout/stderr/stdio.stdout/stdio.stderr
fn transfer_datasets(
    stdout: i32,
    stderr: i32,
    stdio: &mut Stdio,
    proceed: &Arc<AtomicBool>,
) -> Result<[(File, File, Arc<AtomicBool>); 2], Error> {
    let stdout_from = clone_descriptor_as_nonblock_file(stdout)?;
    let stderr_from = clone_descriptor_as_nonblock_file(stderr)?;

    let stdout_to = clone_descriptor_as_file(stdio.stdout().as_raw_fd())?;
    let stderr_to = clone_descriptor_as_file(stdio.stderr().as_raw_fd())?;

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
