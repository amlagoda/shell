use std::env::{var, VarError};

pub fn split_env_path() -> Result<Vec<String>, VarError> {
    Ok(var("PATH")?
        .split(':')
        .map(|r| r.to_string())
        .collect::<Vec<String>>())
}
// tested in command/mod.rs::test_command_from_paths

pub fn get_env_builtin() -> Result<String, VarError> {
    Ok(var(get_env_builtin_name())?)
}

pub fn get_env_builtin_name() -> String {
    String::from("YOUR_PROGRAM_BUILTIN")
}
