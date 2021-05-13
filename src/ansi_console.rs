pub fn delete_last_line() {
    print!("\x1b[A\x1b[2K\r");
}

pub fn print_error(error: &str) {
    println!("\x1b[31m{}\x1b[0m", error);
}

pub fn set_title(title: &str) {
    print!("\x1b]2;{}\x07", title);
}

#[cfg(windows)]
pub fn setup() {
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::winbase::STD_OUTPUT_HANDLE;
    use winapi::um::{
        consoleapi::{GetConsoleMode, SetConsoleMode},
        handleapi::INVALID_HANDLE_VALUE,
        wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING,
    };

    unsafe {
        let console = GetStdHandle(STD_OUTPUT_HANDLE);

        if console == INVALID_HANDLE_VALUE {
            return;
        }

        let mut mode = 0u32;

        if GetConsoleMode(console, &mut mode) == 0 {
            return;
        }

        SetConsoleMode(console, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
    }
}

#[cfg(not(windows))]
pub fn setup() {}
