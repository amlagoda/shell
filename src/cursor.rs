use crossterm::cursor::MoveLeft;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use std::fs::File;
use std::io::Error;

pub fn move_left(target: &mut File, len: u16) -> Result<(), Error> {
    execute!(target, MoveLeft(len), Clear(ClearType::UntilNewLine))
}
