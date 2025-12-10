use std::env::{current_exe, var, VarError};
use std::io::Error;

pub fn split_env_path() -> Result<Vec<String>, VarError> {
    Ok(var("PATH")?
        .split(':')
        .map(|r| r.to_string())
        .collect::<Vec<String>>())
}
// tested in command/mod.rs::test_command_from_paths

pub fn get_current_exe() -> Result<String, Error> {
    let err = Error::other("path is invalid");

    let path = current_exe()?
        .into_os_string()
        .into_string()
        .map_err(|_| err)?;

    Ok(path)
}
