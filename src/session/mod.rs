mod terminal;

use self::terminal::Input;
use crate::history::History;
use crate::keyboard::TerminalAction;

#[derive(Default)]
pub struct State {
    input: Input,
    previous_action: Option<TerminalAction>,
    history: History,
}

impl State {
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
