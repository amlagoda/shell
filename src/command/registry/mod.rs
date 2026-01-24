pub mod cd;
pub mod echo;
pub mod pwd;
pub mod r#type;
pub mod yes;

pub enum Builtin {
    Cd,
    Echo,
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

    pub fn is_blocking(&self) -> bool {
        match self {
            Builtin::Yes => true,
            _ => false,
        }
    }

    fn list() -> Vec<Builtin> {
        vec![
            Builtin::Cd,
            Builtin::Echo,
            Builtin::Exit,
            Builtin::Pwd,
            Builtin::Type,
            Builtin::Yes,
        ]
    }
}

impl ToString for Builtin {
    fn to_string(&self) -> String {
        match self {
            Builtin::Cd => String::from("cd"),
            Builtin::Echo => String::from("echo"),
            Builtin::Exit => String::from("exit"),
            Builtin::Pwd => String::from("pwd"),
            Builtin::Type => String::from("type"),
            Builtin::Yes => String::from("yes"),
        }
    }
}
