mod keyboard;
mod terminal;

use self::keyboard::Keyboard;
use self::terminal::Terminal;

pub struct State {
    keyboard: Keyboard,
    terminal: Terminal,
}

impl State {
    pub fn new() -> State {
        State {
            keyboard: Keyboard::new(),
            terminal: Terminal::new(),
        }
    }

    pub fn keyboard(&mut self) -> &mut Keyboard {
        &mut self.keyboard
    }

    pub fn terminal(&mut self) -> &mut Terminal {
        &mut self.terminal
    }
}
