mod cd;
mod echo;
mod history;
mod pwd;
mod r#type;
mod yes;

pub use cd::run_command as cd;
pub use echo::run_command as echo;
pub use history::run_command as history;
pub use pwd::run_command as pwd;
pub use r#type::run_command as r#type;
pub use yes::run_command as yes;

use std::fmt::{Display, Error, Formatter};

pub enum Builtin {
    Cd,
    Echo,
    History,
    Exit,
    Pwd,
    Type,
    Yes,
}

impl Builtin {
    pub fn to_builtin(command: &str) -> Option<Builtin> {
        Builtin::list()
            .into_iter()
            .find(|r| r.to_string() == command)
    }

    pub fn list_as_strings() -> Vec<String> {
        Builtin::list()
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<String>>()
    }

    // is_blocking - commands that block the program (like "yes")
    pub fn is_blocking(&self) -> bool {
        matches!(self, Builtin::Yes)
    }

    pub fn is_exit(&self) -> bool {
        matches!(self, Builtin::Exit)
    }

    fn list() -> Vec<Builtin> {
        vec![
            Builtin::Cd,
            Builtin::Echo,
            Builtin::History,
            Builtin::Exit,
            Builtin::Pwd,
            Builtin::Type,
            Builtin::Yes,
        ]
    }
}

impl Display for Builtin {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        let to_write = match self {
            Builtin::Cd => "cd",
            Builtin::Echo => "echo",
            Builtin::History => "history",
            Builtin::Exit => "exit",
            Builtin::Pwd => "pwd",
            Builtin::Type => "type",
            Builtin::Yes => "yes",
        };

        write!(formatter, "{}", to_write)?;

        Ok(())
    }
}
