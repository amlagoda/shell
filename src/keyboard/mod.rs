mod handle;

use crate::keyboard::handle::handle_arrow;
use crate::keyboard::handle::handle_backspace;
use crate::keyboard::handle::handle_char;
use crate::keyboard::handle::handle_enter;
use crate::keyboard::handle::handle_tab;

use crate::env::get_current_dir;
use crate::history::Log;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key(
    input: &str,
    key: &KeyEvent,
    previous_key: &Option<&KeyEvent>,
    commands: &Vec<&str>,
    bin_paths: &Vec<&str>,
    log: &mut Log,
    has_user_typing: bool,
) -> HandledKey {
    let mut handled_key = HandledKey::new(input.to_string(), has_user_typing);

    match key.code {
        KeyCode::Enter => handle_enter(&mut handled_key),
        KeyCode::Backspace => handle_backspace(&mut handled_key),
        KeyCode::Tab => handle_tab(
            &mut handled_key,
            commands,
            bin_paths,
            get_current_dir().as_str(),
            previous_key,
        ),
        KeyCode::Char(r) => handle_char(&mut handled_key, &key, r),
        KeyCode::Up | KeyCode::Down => handle_arrow(&mut handled_key, log, &key),
        _ => {}
    }

    handled_key
}

pub struct HandledKey {
    input: String,
    to_print: Option<String>,
    hint: Option<String>,
    backspace_len: Option<usize>,
    is_enter: bool,
    is_exit: bool,
    has_user_typing: bool,
}

impl HandledKey {
    fn new(input: String, has_user_typing: bool) -> HandledKey {
        HandledKey {
            input,
            to_print: None,
            hint: None,
            backspace_len: None,
            is_enter: false,
            is_exit: false,
            has_user_typing,
        }
    }

    pub fn get_input(&self) -> &str {
        self.input.as_str()
    }

    pub fn get_to_print(&self) -> Option<&str> {
        self.to_print.as_deref()
    }

    pub fn get_hint(&self) -> Option<&str> {
        self.hint.as_deref()
    }

    pub fn get_backspace_len(&self) -> Option<usize> {
        self.backspace_len
    }

    pub fn is_enter(&self) -> bool {
        self.is_enter
    }

    pub fn is_exit(&self) -> bool {
        self.is_exit
    }

    pub fn has_user_typing(&self) -> bool {
        self.has_user_typing
    }
}
