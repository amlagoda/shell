mod pipeline;
mod process;

use crate::command::CommandResult;
use crate::command::{run_command as run_builtin, to_builtin};
use crate::core::pipeline::Pipeline;
use crate::core::process::Process;
use crate::env::get_current_exe;
use crate::fs::{search_executable_file_in_paths as find_bin, write_to_file};
use crate::parser::Parsed;
use crate::process::{spawn_pipe_process, spawn_process};
use libc::{fcntl as c_fcntl, waitpid as c_waitpid, F_SETFL, O_NONBLOCK};
use std::fs::File;
use std::io::{pipe, Error, PipeReader, PipeWriter};
use std::io::{ErrorKind, Stdout};
use std::io::{Read, Write};
use std::os::fd::FromRawFd;
use std::process::{Child, Output};
use std::ptr;
use std::thread::sleep;
use std::time::Duration;

pub fn run(parseds: Vec<Parsed>, bin_paths: &Vec<&str>, stdout: Stdout) -> Result<Stdout, Error> {
    let len = parseds.len();

    if len > 1 {
        Ok(run_chain(parseds, stdout)?)
    } else if len == 1 {
        Ok(run_single(
            parseds.into_iter().next().unwrap(),
            bin_paths,
            stdout,
        )?)
    } else {
        Err(Error::other("empty parseds"))
    }
}

fn run_chain(parseds: Vec<Parsed>, mut stdout: Stdout) -> Result<Stdout, Error> {
    let mut pipelines: Vec<Pipeline> = vec![];

    for _ in 0..parseds.len() {
        let result = Pipeline::try_new();

        if let Err(err) = result {
            for mut pipeline in pipelines {
                pipeline.close();
            }

            return Err(err); // not exit
        }

        pipelines.push(result.unwrap());
    }

    let mut processes: Vec<Process> = vec![];

    for (number, parsed) in parseds.iter().enumerate() {
        let result = Process::try_new();

        if let Err(err) = result {
            for mut pipeline in pipelines {
                pipeline.close();
            }
            // kill all processes
            return Err(err); // exit
        }

        let process: Process = result.unwrap();

        if process.is_child() {
            let is_first_process = number == 0;

            if !is_first_process {
                let result = process.set_stdin(pipelines[number - 1].get_read_end());

                if let Err(err) = result {
                    for mut pipeline in pipelines {
                        pipeline.close();
                    }
                    // kill this process
                    return Err(err); // exit
                }
            }

            let result = process.set_stdout(pipelines[number].get_write_end());

            if let Err(err) = result {
                for mut pipeline in pipelines {
                    pipeline.close();
                }
                // kill this process
                return Err(err); // exit
            }

            for mut pipeline in pipelines {
                pipeline.close();
            }

            let err = process.hot_reload_bin(parsed.command(), parsed.args());

            return Err(err); // exit
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
        return Err(err); // exit
    }

    let mut last_read_end = result.unwrap();
    let mut buffer = [0; 4096];

    loop {
        match last_read_end.read(&mut buffer) {
            Ok(read_bytes) => {
                if read_bytes == 0 {
                    break;
                }

                let output = match String::from_utf8(buffer[..=read_bytes].to_vec()) {
                    Ok(readed) => readed,
                    Err(err) => err.to_string(),
                };

                let _ = write!(stdout, "{}", output);
                let _ = stdout.flush();
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
    drop(last_read_end);

    for mut pipeline in pipelines {
        pipeline.close();
    }

    let last_process_pid = processes.last().unwrap().get_pid();

    blocking_wait_process(last_process_pid);

    // kill all processes

    Ok(stdout)
    // Ok(CommandResult::new(None, None))
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

fn run_chain_old(parseds: Vec<Parsed>) -> Result<CommandResult, Error> {
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
}

fn run_single(parsed: Parsed, bin_paths: &Vec<&str>, stdout: Stdout) -> Result<Stdout, Error> {
    let msg = format!("{}: not found", parsed.command());
    let mut result = CommandResult::new(Some(msg), None);

    if let Some(builtin) = to_builtin(parsed.command()) {
        result = run_builtin(&builtin, parsed.args().as_ref(), Some(bin_paths))?;
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

    Ok(stdout)
    // Ok(result)
}

fn output_to_result(output: Output) -> Result<CommandResult, Error> {
    let mut stdout: Option<String> = None;
    let mut stderr: Option<String> = None;

    if !output.stdout.is_empty() {
        let err = Error::other("stdout reading error");
        let r = String::from_utf8(output.stdout).map_err(|_| err)?;

        if !r.trim().is_empty() {
            stdout = Some(r.trim().to_string());
        }
    }

    if !output.stderr.is_empty() {
        let err = Error::other("stderr reading error");
        let r = String::from_utf8(output.stderr).map_err(|_| err)?;

        if !r.trim().is_empty() {
            stderr = Some(r.trim().to_string());
        }
    }

    Ok(CommandResult::new(stderr, stdout))
}
