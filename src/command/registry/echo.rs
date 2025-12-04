use crate::command::CommandResult;
use std::io::Error;

pub fn run_command(args: Option<&Vec<&str>>) -> Result<CommandResult, Error> {
    let join = args
        .ok_or(&vec![" "])
        .unwrap()
        .to_vec()
        .join(" ")
        .to_string();

    Ok(CommandResult::new(None, Some(join)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_command() -> Result<(), Error> {
        let r = vec!["foo", "bar"];
        assert_eq!("foo bar", run_command(Some(&r))?.output().unwrap());

        Ok(())
    }
}
