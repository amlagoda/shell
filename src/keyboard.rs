pub mod keyboard {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    pub fn handle_key(
        mut input: String,
        event: &KeyEvent,
    ) -> (String, Option<String>, bool, bool, bool) {
        let mut to_print: Option<String> = None;
        let mut is_enter = false;
        let mut is_exit = false;
        let mut is_backspace = false;

        match event.code {
            KeyCode::Enter => is_enter = true,

            KeyCode::Backspace => {
                if input.len() > 0 {
                    input.pop();
                    is_backspace = true;
                }
            }

            KeyCode::Tab => {
                if input == "ech" {
                    input.push_str("o ");
                    to_print = Some("o ".to_string());
                }

                if input == "exi" {
                    input.push_str("t ");
                    to_print = Some("t ".to_string());
                }
            }

            KeyCode::Char(r) => {
                let is_ctrl = event.modifiers == KeyModifiers::CONTROL;

                if r == 'c' && is_ctrl {
                    to_print = Some("^C".to_string());
                    is_exit = true;
                } else if r == 'j' && is_ctrl {
                    is_enter = true;
                } else {
                    input.push(r);
                    to_print = Some(r.to_string());
                }
            }

            _ => {}
        }

        (input, to_print, is_enter, is_exit, is_backspace)
    }
}
