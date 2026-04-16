mod input;

use crate::session::input::Input;
use crossterm::event::KeyEvent;

pub struct State {
    previous_key: Option<KeyEvent>,
    input: Input,
}

impl State {
    pub fn new() -> State {
        State {
            previous_key: None,
            input: Input::new(),
        }
    }

    pub fn previous_key(&self) -> Option<&KeyEvent> {
        self.previous_key.as_ref()
    }

    pub fn set_previous_key(&mut self, key: KeyEvent) {
        self.previous_key = Some(key)
    }

    pub fn input(&mut self) -> &mut Input {
        &mut self.input
    }
}
