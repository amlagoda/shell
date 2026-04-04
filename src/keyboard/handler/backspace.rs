use crate::keyboard::handler::HandledKey;

pub fn handle(handled_key: &mut HandledKey) {
    if !handled_key.input.is_empty() {
        handled_key.input.pop();
        handled_key.backspace_len = Some(1);
    }
    // not else
    if handled_key.input.is_empty() {
        handled_key.has_user_typing = false;
    }
}
