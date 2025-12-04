use crate::command::CommandResult;
use std::io::Error;

pub fn run_command(args: &Vec<&str>) -> Result<CommandResult, Error> {
    let join = args.to_vec().join(" ").to_string();

    Ok(CommandResult::new(None, Some(join)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_command() -> Result<(), Error> {
        let r = vec!["foo", "bar"];
        assert_eq!("foo bar", run_command(&r)?.output().unwrap());

        Ok(())
    }
}
