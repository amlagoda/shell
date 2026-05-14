mod terminal;

use self::terminal::Terminal;
use crate::keyboard::TerminalAction;

pub struct State {
    terminal: Terminal,
    previous_action: Option<TerminalAction>,
}

impl State {
    pub fn new() -> State {
        State {
            terminal: Terminal::new(),
            previous_action: None,
        }
    }

    pub fn previous_action(&self) -> Option<&TerminalAction> {
        self.previous_action.as_ref()
    }

    pub fn set_previous_action(&mut self, action: TerminalAction) {
        self.previous_action = Some(action);
    }

    pub fn terminal(&mut self) -> &mut Terminal {
        &mut self.terminal
    }
}
