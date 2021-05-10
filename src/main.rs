use regex::bytes::Regex;
use std::{error::Error, process::Stdio};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::{process::Command, spawn};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    setup_ansi_console();

    let mvn_path = which::which("mvn").expect("cannot find mvn");

    let mut mvn = Command::new(mvn_path)
        .args(std::env::args().skip(1))
        .env("MAVEN_OPTS", "-Djansi.passthrough=true")
        .stdout(Stdio::piped())
        .spawn()
        .expect("cannot start mvn");

    let output = mvn.stdout.take().expect("no stdout");
    let mut reader = BufReader::new(output).split(b'\n');

    spawn(async move {
        let _ = mvn.wait().await;
    });

    let pattern = Regex::new(r"Building (.+)\s+\[(\d+)/(\d+)\]")?;

    while let Some(line) = reader.next_segment().await? {
        if let Some(captures) = pattern.captures(&line) {
            print!(
                "\x1b]2;[{}/{}] {}\x07",
                String::from_utf8_lossy(&captures[2]),
                String::from_utf8_lossy(&captures[3]),
                String::from_utf8_lossy(&captures[1])
            );
        }

        println!("{}", String::from_utf8_lossy(&line));
    }

    Ok(())
}

#[cfg(windows)]
fn setup_ansi_console() {
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
fn setup_ansi_console() {}
