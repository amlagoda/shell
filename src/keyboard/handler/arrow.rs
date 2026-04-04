use crate::history::Log;
use crate::keyboard::handler::HandledKey;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle(handled_key: &mut HandledKey, log: &mut Log, key: &KeyEvent) {
    if !handled_key.has_user_typing {
        if !handled_key.input.is_empty() {
            handled_key.backspace_len = Some(handled_key.input.len());
        }

        let command = if key.code == KeyCode::Up {
            log.next()
        } else {
            log.prev()
        };

        if let Some(command) = command {
            handled_key.input = command;
            handled_key.to_print = Some(handled_key.input.clone());
        } else {
            handled_key.to_print = Some(format!("{}\x07", handled_key.input));
        }
    }
}
