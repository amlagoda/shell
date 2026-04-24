// use crate::complete::complete_input;
// use crate::keyboard::handler::HandledKey;
// use crossterm::event::{KeyCode, KeyEvent};

// pub fn handle(
//     handled_key: &mut HandledKey,
//     commands: &Vec<&str>,
//     bin_paths: &Vec<&str>,
//     current_dir: &str,
//     previous_key: &Option<&KeyEvent>,
// ) {
//     handled_key.to_print = Some("\x07".to_string());

//     let r = complete_input(handled_key.input.as_str(), commands, bin_paths, current_dir);

//     if let Some(completion) = r {
//         if let Some(r) = completion.get_variants() {
//             if let Some(f) = previous_key {
//                 if f.code == KeyCode::Tab {
//                     handled_key.hint = Some(r.join("  "));
//                     handled_key.to_print = None;
//                 }
//             }
//         } else {
//             let selected = completion.get_selected().unwrap();

//             handled_key.input.push_str(selected);
//             handled_key.to_print = Some(selected.to_string());
//         }
//     }
// }
