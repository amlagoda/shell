use std::env::{args, /*current_exe,*/ var};
use std::io::Error;

pub fn split_env_path() -> Result<Vec<String>, Error> {
    Ok(var("PATH")
        .map_err(|e| Error::other(e.to_string()))?
        .split(':')
        .map(|r| r.to_string())
        .collect::<Vec<String>>())
}
// tested in command/mod.rs::test_command_from_paths

pub fn get_args() -> Vec<String> {
    args().into_iter().skip(1).collect::<Vec<String>>()
}

// pub fn get_current_exe() -> Result<String, Error> {
//     let err = Error::other("path is invalid");

//     let path = current_exe()?
//         .into_os_string()
//         .into_string()
//         .map_err(|_| err)?;

//     Ok(path)
// }
