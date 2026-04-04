use crate::keyboard::handler::HandledKey;

pub fn handle(handled_key: &mut HandledKey) {
    handled_key.set_enter(true);
    handled_key.set_user_typing(false);
}
