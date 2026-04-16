use crossterm::event::KeyEvent;

pub struct Keyboard {
    previous_key: Option<KeyEvent>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard { previous_key: None }
    }

    pub fn previous_key(&self) -> Option<&KeyEvent> {
        self.previous_key.as_ref()
    }

    pub fn set_previous_key(&mut self, key: KeyEvent) {
        self.previous_key = Some(key)
    }
}
