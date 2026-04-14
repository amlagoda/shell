use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn to_action(key: &KeyEvent) -> Option<TerminalAction> {
    match key.code {
        KeyCode::Enter => Some(TerminalAction::Run),
        KeyCode::Backspace => Some(TerminalAction::InputSub),
        KeyCode::Tab => Some(TerminalAction::InputComplete),
        KeyCode::Char(r) => from_char(r, key.modifiers == KeyModifiers::CONTROL),
        KeyCode::Up => Some(TerminalAction::HistoryNext),
        KeyCode::Down => Some(TerminalAction::HistoryPrev),
        _ => None,
    }
}

fn from_char(symbol: char, is_ctrl: bool) -> Option<TerminalAction> {
    let action = if symbol == 'c' && is_ctrl {
        TerminalAction::Exit
    } else if symbol == 'j' && is_ctrl {
        TerminalAction::Run
    } else {
        TerminalAction::InputAdd(symbol)
    };

    Some(action)
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
