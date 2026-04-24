// mod handler;

// use crate::env::get_current_dir;
// use crate::history::Log;
// use crate::keyboard::handler::HandledKey;
// use crossterm::event::{KeyCode, KeyEvent};

// use crate::keyboard::handler::handle_arrow;
// use crate::keyboard::handler::handle_backspace;
// use crate::keyboard::handler::handle_char;
// use crate::keyboard::handler::handle_enter;
// use crate::keyboard::handler::handle_tab;

// pub fn handle_key(
//     input: &str,
//     key: &KeyEvent,
//     previous_key: &Option<&KeyEvent>,
//     commands: &Vec<&str>,
//     bin_paths: &Vec<&str>,
//     log: &mut Log,
//     has_user_typing: bool,
// ) -> HandledKey {
//     let mut handled_key = HandledKey::new(input.to_string(), has_user_typing);

//     match key.code {
//         KeyCode::Enter => handle_enter(&mut handled_key),
//         KeyCode::Backspace => handle_backspace(&mut handled_key),
//         KeyCode::Tab => handle_tab(
//             &mut handled_key,
//             commands,
//             bin_paths,
//             get_current_dir().as_str(),
//             previous_key,
//         ),
//         KeyCode::Char(r) => handle_char(&mut handled_key, &key, r),
//         KeyCode::Up | KeyCode::Down => handle_arrow(&mut handled_key, log, &key),
//         _ => {}
//     }

//     handled_key
// }
