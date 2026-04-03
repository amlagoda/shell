use crate::complete::complete_input;
use crate::env::get_current_dir;
use crate::history::Log;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

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
        KeyCode::Enter => {
            handled_key.is_enter = true;
            handled_key.has_user_typing = false;
        }

        KeyCode::Backspace => {
            if !handled_key.input.is_empty() {
                handled_key.input.pop();
                handled_key.backspace_len = Some(1);
            }
            // not else
            if handled_key.input.is_empty() {
                handled_key.has_user_typing = false;
            }
        }

        KeyCode::Tab => {
            handled_key.to_print = Some("\x07".to_string());

            let r = complete_input(
                handled_key.input.as_str(),
                commands,
                bin_paths,
                get_current_dir().as_str(),
            );

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

        KeyCode::Char(r) => {
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

        KeyCode::Up | KeyCode::Down => {
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
