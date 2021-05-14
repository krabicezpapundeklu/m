use crossterm::{
    cursor, execute,
    style::{self, Color},
    terminal::{self, ClearType},
};

use std::io::stdout;

pub fn delete_last_line() {
    let _ = execute!(
        stdout(),
        cursor::MoveToPreviousLine(0),
        terminal::Clear(ClearType::CurrentLine)
    );
}

pub fn print_error(error: &str) {
    let _ = execute!(stdout(), style::SetForegroundColor(Color::Red));
    println!("{}", error);
    let _ = execute!(stdout(), style::SetForegroundColor(Color::Reset));
}

pub fn set_title(title: &str) {
    let _ = execute!(stdout(), terminal::SetTitle(title));
}
