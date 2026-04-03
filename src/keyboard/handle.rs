use crate::complete::complete_input;
use crate::history::Log;
use crate::keyboard::HandledKey;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_enter(handled_key: &mut HandledKey) {
    handled_key.is_enter = true;
    handled_key.has_user_typing = false;
}

pub fn handle_backspace(handled_key: &mut HandledKey) {
    if !handled_key.input.is_empty() {
        handled_key.input.pop();
        handled_key.backspace_len = Some(1);
    }
    // not else
    if handled_key.input.is_empty() {
        handled_key.has_user_typing = false;
    }
}

pub fn handle_tab(
    handled_key: &mut HandledKey,
    commands: &Vec<&str>,
    bin_paths: &Vec<&str>,
    current_dir: &str,
    previous_key: &Option<&KeyEvent>,
) {
    handled_key.to_print = Some("\x07".to_string());

    let r = complete_input(handled_key.input.as_str(), commands, bin_paths, current_dir);

    if let Some(completion) = r {
        if let Some(r) = completion.get_variants() {
            if let Some(f) = previous_key {
                if f.code == KeyCode::Tab {
                    handled_key.hint = Some(r.join("  "));
                    handled_key.to_print = None;
                }
            }
        } else {
            let selected = completion.get_selected().unwrap();

            handled_key.input.push_str(selected);
            handled_key.to_print = Some(selected.to_string());
        }
    }
}

pub fn handle_char(handled_key: &mut HandledKey, key: &KeyEvent, r: char) {
    let is_ctrl = key.modifiers == KeyModifiers::CONTROL;

    if r == 'c' && is_ctrl {
        handled_key.to_print = Some("^C".to_string());
        handled_key.is_exit = true;
    } else if r == 'j' && is_ctrl {
        handled_key.is_enter = true;
        handled_key.has_user_typing = false;
    } else {
        handled_key.input.push(r);
        handled_key.to_print = Some(r.to_string());
        handled_key.has_user_typing = true;
    }
}

pub fn handle_arrow(handled_key: &mut HandledKey, log: &mut Log, key: &KeyEvent) {
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
