use std::env::{var, VarError};

pub fn split_env_path() -> Result<Vec<String>, VarError> {
    Ok(var("PATH")?
        .split(':')
        .map(|r| r.to_string())
        .collect::<Vec<String>>())
}

// tested in command/mod.rs::test_command_from_paths
