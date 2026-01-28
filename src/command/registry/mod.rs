pub mod cd;
pub mod echo;
pub mod pwd;
// pub mod tee;
pub mod r#type;
pub mod yes;

// the output of the commands must not start or end \r\n
pub enum Builtin {
    Cd,
    Echo,
    Exit,
    Pwd,
    // Tee,
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
        match self {
            Builtin::Yes => true,
            // Builtin::Tee => true,
            _ => false,
        }
    }

    pub fn is_exit(&self) -> bool {
        match self {
            Builtin::Exit => true,
            _ => false,
        }
    }

    fn list() -> Vec<Builtin> {
        vec![
            Builtin::Cd,
            Builtin::Echo,
            Builtin::Exit,
            Builtin::Pwd,
            // Builtin::Tee,
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
            // Builtin::Tee => String::from("tee"),
            Builtin::Type => String::from("type"),
            Builtin::Yes => String::from("yes"),
        }
    }
}

pub struct PrintFact {
    stdout: bool,
    stderr: bool,
}

impl PrintFact {
    pub fn new(stdout: bool, stderr: bool) -> PrintFact {
        PrintFact { stdout, stderr }
    }

    pub fn is_stdout(&self) -> bool {
        self.stdout
    }

    pub fn is_stderr(&self) -> bool {
        self.stderr
    }

    pub fn is_any(&self) -> bool {
        self.is_stdout() || self.is_stderr()
    }
}
