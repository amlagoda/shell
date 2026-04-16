mod input;

use self::input::Input;

pub struct Terminal {
    input: Input,
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            input: Input::new(),
        }
    }

    pub fn input(&mut self) -> &mut Input {
        &mut self.input
    }
}
