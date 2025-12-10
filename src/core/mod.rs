use crate::command::CommandResult;
use crate::command::{run_command as run_builtin, to_builtin};
use crate::env::get_current_exe;
use crate::fs::{search_executable_file_in_paths as find_bin, write_to_file};
use crate::parser::Parsed;
use crate::process::{spawn_pipe_process, spawn_process};
use std::io::{pipe, Error, PipeReader, PipeWriter};
use std::process::{Child, Output};

pub fn run(parseds: Vec<Parsed>, bin_paths: &Vec<&str>) -> Result<CommandResult, Error> {
    let len = parseds.len();

    if len > 1 {
        Ok(run_chain(parseds)?)
    } else if len == 1 {
        Ok(run_single(parseds.into_iter().next().unwrap(), bin_paths)?)
    } else {
        Err(Error::other("empty parseds"))
    }
}

fn run_chain(parseds: Vec<Parsed>) -> Result<CommandResult, Error> {
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

fn run_single(parsed: Parsed, bin_paths: &Vec<&str>) -> Result<CommandResult, Error> {
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

    Ok(result)
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
