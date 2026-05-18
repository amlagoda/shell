mod terminal;

use self::terminal::Input;
use crate::history::History;
use crate::keyboard::TerminalAction;

pub struct State {
    input: Input,
    previous_action: Option<TerminalAction>,
    history: History,
}

impl State {
    pub fn new() -> State {
        State {
            input: Input::new(),
            previous_action: None,
            history: History::new(),
        }
    }

    pub fn previous_action(&self) -> Option<&TerminalAction> {
        self.previous_action.as_ref()
    }

    pub fn set_previous_action(&mut self, action: TerminalAction) {
        self.previous_action = Some(action);
    }

    pub fn input(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn history(&mut self) -> &mut History {
        &mut self.history
    }
}
