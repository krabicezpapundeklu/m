use duct::cmd;
use notify_rust::Notification;
use std::io::{BufRead, BufReader};

mod ansi_console;
mod mvn_output_handler;

#[macro_use]
extern crate lazy_static;

const QUIET_ARG: &str = "--m-quiet";

fn main() {
    ansi_console::setup();

    let mvn_path = which::which("mvn").expect("cannot find mvn");

    let (m_args, mvn_args): (Vec<_>, Vec<_>) =
        std::env::args().skip(1).partition(|arg| arg == QUIET_ARG);

    let mvn = cmd(mvn_path, &mvn_args)
        .env("MAVEN_OPTS", "-Djansi.passthrough=true")
        .stderr_to_stdout();

    let quiet = m_args.iter().any(|arg| arg == QUIET_ARG);
    let mut output_handler = mvn_output_handler::MvnOutputHandler::new(quiet);

    let mut build_summary = "Build succeeded 😎";
    let lines = BufReader::new(mvn.reader().unwrap()).split(b'\n');

    for line in lines {
        match line {
            Ok(line) => output_handler.handle_line(&String::from_utf8_lossy(&line)),
            Err(_) => {
                build_summary = "Build failed 😢";
                break;
            }
        }
    }

    Notification::new().summary(build_summary).show().unwrap();
}
