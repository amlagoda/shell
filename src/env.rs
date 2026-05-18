use std::env::{args, current_dir, /*current_exe,*/ var};
use std::io::Error;

pub fn get_bin_paths() -> Result<Vec<String>, Error> {
    Ok(var("PATH")
        .map_err(|e| Error::other(e.to_string()))?
        .split(':')
        .map(|r| r.to_string())
        .collect::<Vec<String>>())
}
// tested in command/mod.rs::test_command_from_paths

pub fn get_args() -> Vec<String> {
    args().skip(1).collect::<Vec<String>>()
}

pub fn get_history_log_path() -> Option<String> {
    var("HISTFILE").ok()
}

pub fn get_current_dir() -> Result<String, Error> {
    // does not end with a slash
    let err = Error::other("invalid utf-8");
    let current_dir = current_dir()?.to_str().ok_or(err)?.to_string();

    Ok(current_dir)
}

// pub fn get_current_exe() -> Result<String, Error> {
//     let err = Error::other("path is invalid");

//     let path = current_exe()?
//         .into_os_string()
//         .into_string()
//         .map_err(|_| err)?;

//     Ok(path)
// }
