use crossterm::event::KeyEvent;

struct State {
    previous_key: Option<KeyEvent>,
}

impl State {
    pub fn new(previous_key: Option<KeyEvent>) -> State {
        State { previous_key }
    }

    pub fn previous_key(&self) -> Option<KeyEvent> {
        self.previous_key
    }

    pub fn set_previous_key(&mut self, key: KeyEvent) {
        self.previous_key = Some(key)
    }
}
