use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn to_action(key: &KeyEvent) -> Option<TerminalAction> {
    let action = match key.code {
        KeyCode::Enter => TerminalAction::Run,
        KeyCode::Backspace => TerminalAction::InputSub,
        KeyCode::Tab => TerminalAction::InputComplete,
        KeyCode::Char(r) => from_char(r, key.modifiers == KeyModifiers::CONTROL),
        KeyCode::Up => TerminalAction::HistoryNext,
        KeyCode::Down => TerminalAction::HistoryPrev,
        _ => return None,
    };

    Some(action)
}

fn from_char(symbol: char, is_ctrl: bool) -> TerminalAction {
    if symbol == 'c' && is_ctrl {
        TerminalAction::Exit
    } else if symbol == 'j' && is_ctrl {
        TerminalAction::Run
    } else {
        TerminalAction::InputAdd(symbol)
    }
}

pub enum TerminalAction {
    Run,
    Exit,
    HistoryNext,
    HistoryPrev,
    InputAdd(char),
    InputSub,
    InputComplete,
}
