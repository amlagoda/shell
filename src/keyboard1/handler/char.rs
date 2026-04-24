use crate::keyboard::handler::HandledKey;
use crossterm::event::{KeyEvent, KeyModifiers};

pub fn handle(handled_key: &mut HandledKey, key: &KeyEvent, r: char) {
    let is_ctrl = key.modifiers == KeyModifiers::CONTROL;

    if r == 'c' && is_ctrl {
        handled_key.to_print = Some("^C".to_string());
        handled_key.is_exit = true;
    } else if r == 'j' && is_ctrl {
        handled_key.is_enter = true;
        handled_key.has_user_typing = false;
    } else {
        // handled_key.input.push(r);
        // handled_key.to_print = Some(r.to_string());
        // handled_key.has_user_typing = true;
    }
}
