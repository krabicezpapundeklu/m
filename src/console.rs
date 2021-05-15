use crossterm::{
    cursor::MoveToPreviousLine,
    execute,
    style::{Colorize, PrintStyledContent},
    terminal::{Clear, ClearType, SetTitle},
};

use std::io::stdout;

pub fn delete_last_line() {
    let _ = execute!(
        stdout(),
        MoveToPreviousLine(0),
        Clear(ClearType::CurrentLine)
    );
}

pub fn print_error(error: &str) {
    let _ = execute!(stdout(), PrintStyledContent(error.red()));
}

pub fn set_title(title: &str) {
    let _ = execute!(stdout(), SetTitle(title));
}
