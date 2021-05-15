use duct::cmd;
use notify_rust::Notification;
use std::io::{BufRead, BufReader};

mod console;
mod mvn_output_handler;

#[macro_use]
extern crate lazy_static;

const QUIET_ARG: &str = "--m-quiet";

fn main() {
    console::set_title("Running...");

    let mvn_path = which::which("mvn").expect("cannot find mvn");

    let (m_args, mvn_args): (Vec<_>, Vec<_>) =
        std::env::args().skip(1).partition(|arg| arg == QUIET_ARG);

    let mvn = cmd(mvn_path, &mvn_args)
        .env("MAVEN_OPTS", "-Djansi.passthrough=true")
        .stderr_to_stdout();

    let quiet = m_args.iter().any(|arg| arg == QUIET_ARG);
    let mut output_handler = mvn_output_handler::MvnOutputHandler::new(quiet);

    let mut success = true;
    let lines = BufReader::new(mvn.reader().unwrap()).split(b'\n');

    for line in lines {
        match line {
            Ok(line) => output_handler.handle_line(&String::from_utf8_lossy(&line)),
            Err(_) => {
                success = false;
                break;
            }
        }
    }

    let build_summary = if success && output_handler.success() {
        "Build succeeded 😎"
    } else {
        "Build failed 😢"
    };

    Notification::new().summary(build_summary).show().unwrap();
}
