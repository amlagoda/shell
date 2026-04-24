// mod arrow;
// mod backspace;
// mod char;
// mod enter;
// mod tab;
// mod test;

// pub use arrow::handle as handle_arrow;
// pub use backspace::handle as handle_backspace;
// pub use char::handle as handle_char;
// pub use enter::handle as handle_enter;
// pub use tab::handle as handle_tab;

// pub struct HandledKey {
//     input: String,
//     to_print: Option<String>,
//     hint: Option<String>,
//     backspace_len: Option<usize>,
//     is_enter: bool,
//     is_exit: bool,
//     has_user_typing: bool,
// }

// impl HandledKey {
//     pub fn new(input: String, has_user_typing: bool) -> HandledKey {
//         HandledKey {
//             input,
//             to_print: None,
//             hint: None,
//             backspace_len: None,
//             is_enter: false,
//             is_exit: false,
//             has_user_typing,
//         }
//     }

//     pub fn input(&self) -> &str {
//         self.input.as_str()
//     }

//     pub fn get_to_print(&self) -> Option<&str> {
//         self.to_print.as_deref()
//     }

//     pub fn get_hint(&self) -> Option<&str> {
//         self.hint.as_deref()
//     }

//     pub fn get_backspace_len(&self) -> Option<usize> {
//         self.backspace_len
//     }

//     pub fn is_enter(&self) -> bool {
//         self.is_enter
//     }

//     pub fn is_exit(&self) -> bool {
//         self.is_exit
//     }

//     pub fn has_user_typing(&self) -> bool {
//         self.has_user_typing
//     }
// }
