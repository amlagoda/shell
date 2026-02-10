mod pipeline;
mod process;

use crate::command::{run_command as run_builtin, to_command as to_builtin, NewLine, PrintFact};
use crate::core::pipeline::Pipeline;
use crate::core::process::Process;
use crate::env::get_current_exe;
use crate::fs::{open_file, search_executable_file_in_paths as find_bin};
use crate::io::Stdio;
use crate::parser::Parsed;
use crate::Exit;
use libc::{fcntl as c_fcntl, waitpid as c_waitpid, F_SETFL, O_NONBLOCK};
use std::fs::File;
use std::io::{pipe, Error, PipeReader, PipeWriter};
use std::io::{ErrorKind, Stdout};
use std::io::{Read, Write};
use std::os::fd::{AsRawFd, FromRawFd};
use std::process::{exit, Child, Output};
use std::ptr;
use std::thread::sleep;
use std::time::Duration;

pub fn run(
    parseds: &Vec<Parsed>,
    stdio: &mut Stdio,
    bin_paths: &Vec<&str>,
) -> Result<PrintFact, Error> {
    let len = parseds.len();

    if len == 0 {
        return Err(Error::other("empty parseds"));
    }

    if len == 1 {
        let parsed = parseds.first().unwrap();

        if let Some(builtin) = to_builtin(parsed.command()) {
            if !builtin.is_blocking() {
                // native run single, builtin and non-blocking commands
                let args = parsed.args();

                if let Some(redirect) = parsed.redirect() {
                    let file = open_file(redirect.path(), redirect.is_append())?;
                    let stdin = (*stdio.stdin()).try_clone()?;
                    let mut stdout = (*stdio.stdout()).try_clone()?;
                    let mut stderr = (*stdio.stderr()).try_clone()?;
                    let mut new_line =
                        NewLine::new(false /* stdout */, true /* stderr */);

                    if redirect.is_stderr() {
                        stderr = file;
                        new_line = NewLine::new(true /* stdout */, false /* stderr */);
                    } else {
                        stdout = file;
                    }

                    let mut stdio = Stdio::new(stdin, stdout, stderr);

                    let print_fact = run_builtin(
                        &builtin,
                        &mut stdio,
                        args.as_ref(),
                        Some(&bin_paths),
                        &new_line,
                    )?;

                    let print_fact = if redirect.is_stderr() {
                        PrintFact::new(print_fact.is_stdout(), false)
                    } else {
                        PrintFact::new(false, print_fact.is_stderr())
                    };

                    return Ok(print_fact);
                }

                let new_line = NewLine::new(true /* stdout */, true /* stderr */);

                return run_builtin(&builtin, stdio, args.as_ref(), Some(&bin_paths), &new_line);
            }
        }
    }
    // other commands run as forks
    run_forks(parseds, stdio, bin_paths)
}

fn run_forks(
    parseds: &Vec<Parsed>,
    stdio: &mut Stdio,
    bin_paths: &Vec<&str>,
) -> Result<PrintFact, Error> {
    let mut pipelines: Vec<Pipeline> = vec![];

    for _ in 0..parseds.len() {
        let result = Pipeline::try_new();

        if let Err(err) = result {
            close_all(pipelines);

            return Err(err);
        }

        pipelines.push(result.unwrap());
    }

    let mut processes: Vec<Process> = vec![];

    for (number, parsed) in parseds.iter().enumerate() {
        // if parsed.command() == "exit" {
        // close_all(pipelines);
        // kill all processes
        // return Ok(Exit::Yes);
        // }

        let result = Process::try_new();

        if let Err(err) = result {
            close_all(pipelines);
            // kill all processes
            return Err(err);
        }

        let process: Process = result.unwrap();

        if process.is_child() {
            let is_first_process = number == 0;

            if !is_first_process {
                let result = process.set_stdin(pipelines[number - 1].get_read_end());

                if let Err(err) = result {
                    close_all(pipelines);
                    // kill this process
                    return Err(err);
                }
            }

            let result = process.set_stdout(pipelines[number].get_write_end());

            if let Err(err) = result {
                close_all(pipelines);
                // kill this process
                return Err(err);
            }

            close_all(pipelines);

            if let Some(command) = to_builtin(parsed.command()) {
                // builtin
                return Err(Error::other("stub"));
            } else {
                let err = process.hot_reload_bin(parsed.command(), parsed.args());

                return Err(err);
            }
        }

        processes.push(process);
    }

    let mut last_read_end = 0;
    for (number, pipeline) in pipelines.iter_mut().enumerate() {
        if number == parseds.len() - 1 {
            last_read_end = pipeline.get_read_end();
        }
        pipeline.close_write_end();
    }

    let result = unlock_buf_and_wrap_to_file(last_read_end);

    if let Err(err) = result {
        close_all(pipelines);
        // kill all pocesses
        return Err(err);
    }

    let last_read_end = result.unwrap();
    match read_file_to_stdout(last_read_end, stdio.stdout()) {
        // Ok(std) => stdout = std,
        Ok(_) => {}
        Err(err) => {
            close_all(pipelines);
            // kill app processes
            return Err(err);
        }
    }
    close_all(pipelines);
    let last_process_pid = processes.last().unwrap().get_pid();
    blocking_wait_process(last_process_pid);

    // kill all processes

    // заглушка
    Ok(PrintFact::new(false, false))
}

fn read_file_to_stdout(mut file: File, stdout: &mut File) -> Result<(), Error> {
    let mut buffer = [0; 4096];

    loop {
        match file.read(&mut buffer) {
            Ok(read_bytes) => {
                if read_bytes == 0 {
                    break;
                }

                let output = match String::from_utf8(buffer[..=read_bytes].to_vec()) {
                    Ok(readed) => readed,
                    Err(err) => err.to_string(),
                };

                for line in output.split("\n").filter(|r| !["\n", "\0"].contains(r)) {
                    // println!("{:?}", line);
                    let _ = write!(stdout, "\r\n{}", line);
                    let _ = stdout.flush();
                }

                buffer = [0; 4096];
            }
            Err(err) => {
                if err.kind() == ErrorKind::WouldBlock {
                    sleep(Duration::from_millis(10));
                    continue;
                }

                return Err(err); // exit
            }
        }
    }

    Ok(())
}

fn close_all(pipelines: Vec<Pipeline>) {
    for mut pipeline in pipelines {
        pipeline.close();
    }
}

fn blocking_wait_process(pid: u32) {
    unsafe {
        c_waitpid(pid as i32, ptr::null_mut(), 0);
    };
}

fn unlock_buf_and_wrap_to_file(file_descriptor: u32) -> Result<File, Error> {
    if file_descriptor == 0 {
        return Err(Error::other("file descriptor is closed"));
    }

    let status = unsafe { c_fcntl(file_descriptor as i32, F_SETFL, O_NONBLOCK) };

    if status == -1 {
        return Err(Error::other("fcntl error"));
    }

    let file = unsafe { File::from_raw_fd(file_descriptor as i32) };

    Ok(file)
}

/*fn run_chain_old(parseds: Vec<Parsed>) -> Result<(), Error> {
    let current_exe = get_current_exe()?;
    let mut processes: Vec<Child> = vec![];
    let mut previous_output: Option<PipeReader> = None;

    for parsed in parseds.into_iter() {
        let mut stdout: Option<PipeWriter> = None;
        let mut stderr: Option<PipeWriter> = None;
        let (current_output, current_output_writer) = pipe()?;
        let mut bin = parsed.command();
        let mut args = parsed.args();

        if to_builtin(parsed.command()).is_some() {
            let def: Vec<&str> = vec![];
            let mut new_args = args.unwrap_or(def);
            new_args.insert(0, parsed.command());
            args = Some(new_args);
            bin = current_exe.as_str();
        }

        if let Some(pipeline) = parsed.pipeline() {
            if !pipeline.is_stdout() {
                stderr = Some(current_output_writer.try_clone()?);
            }
            stdout = Some(current_output_writer);
        }

        let process = spawn_pipe_process(bin, args.as_ref(), previous_output, stdout, stderr)?;
        processes.push(process);
        previous_output = Some(current_output);
    }

    if let Some(last_process) = processes.pop() {
        for mut process in processes {
            process.wait()?;
        }

        output_to_result(last_process.wait_with_output()?)
    } else {
        Err(Error::other("parseds is empty"))
    }
}*/

/*fn run_single(
    stdio: &mut Stdio,
    parsed: Parsed,
    bin_paths: &Vec<&str>,
    mut stdout: Stdout,
) -> Result<Stdout, Error> {
    let msg = format!("{}: not found", parsed.command());
    let mut result = CommandResult::new(Some(msg), None);

    if let Some(builtin) = to_builtin(parsed.command()) {
        result = run_builtin(stdio, &builtin, parsed.args().as_ref(), Some(bin_paths))?;
    } else if find_bin(parsed.command(), bin_paths).is_some() {
        let process = spawn_process(parsed.command(), parsed.args().as_ref())?;
        result = output_to_result(process.wait_with_output()?)?;
    }

    if let Some(redirect) = parsed.redirect() {
        let mut to_write = String::new();

        if !redirect.is_stderr() && result.output().is_some() {
            to_write.push_str(result.output().unwrap());

            if result.is_exit() {
                result = CommandResult::new_exit(result.output().map(|r| r.to_string()));
            } else {
                result = CommandResult::new(result.error().map(|r| r.to_string()), None);
            }
        }

        if redirect.is_stderr() && result.error().is_some() {
            to_write.push_str(result.error().unwrap());
            result = CommandResult::new(None, result.output().map(|r| r.to_string()));
        }

        if !to_write.is_empty() {
            to_write.push('\n');
        }

        write_to_file(redirect.path(), to_write.as_str(), redirect.is_append())?;
    }

    if let Some(error) = result.error() {
        let _ = write!(stdout, "{}", error);
        let _ = stdout.flush();
    }

    if let Some(output) = result.output() {
        let _ = write!(stdout, "{}", output);
        let _ = stdout.flush();
    }

    Ok(stdout)
    // Ok(result)
}*/
