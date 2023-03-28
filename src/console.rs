use crossterm::{
    cursor::MoveToPreviousLine,
    execute,
    style::{PrintStyledContent, Stylize},
    terminal::{Clear, ClearType, SetTitle},
};

use std::io::stdout;

pub fn delete_last_line() {
    execute!(
        stdout(),
        MoveToPreviousLine(0),
        Clear(ClearType::CurrentLine)
    )
    .unwrap();
}

pub fn print_error(error: &str) {
    execute!(stdout(), PrintStyledContent(error.red())).unwrap();
}

pub fn set_title(title: &str) {
    execute!(stdout(), SetTitle(title)).unwrap();
}
