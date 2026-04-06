use crossterm::event::KeyEvent;

struct HandledKey {
    actions: Option<Vec<InteractiveAction>>,
    previous: Option<KeyEvent>,
    current: KeyEvent,
}

impl HandledKey {
    fn new(previous: Option<KeyEvent>, current: KeyEvent) -> HandledKey {
        HandledKey {
            previous,
            current,
            actions: None,
        }
    }

    fn iter(&self) -> impl Iterator<Item = &InteractiveAction> {
        self.actions.iter().flatten()
    }
}

enum InteractiveAction {
    PrintInput(String),
    PrintOutput(String),
    SaveInput(String),
    RemoveInput(usize),
    Exit,
}
