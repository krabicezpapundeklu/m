use duct::cmd;
use notify_rust::Notification;
use std::io::{BufRead, BufReader};

mod console;
mod mvn_output_handler;

#[macro_use]
extern crate lazy_static;

fn main() {
    console::set_title("Running...");

    let mvn_path = which::which("mvn").expect("cannot find mvn");

    let mut quiet = false;

    let mvn_args: Vec<_> = std::env::args()
        .skip(1)
        .filter(|arg| match arg.as_str() {
            "--m-quiet" => {
                quiet = true;
                false
            }
            _ => true,
        })
        .collect();

    let mvn = cmd(mvn_path, &mvn_args)
        .env("MAVEN_OPTS", "-Djansi.passthrough=true")
        .stderr_to_stdout();

    let mut output_handler = mvn_output_handler::MvnOutputHandler::new(quiet);
    let mut success = true;

    let lines = BufReader::new(mvn.reader().unwrap()).split(b'\n');

    for line in lines {
        if let Ok(line) = line {
            output_handler.handle_line(&String::from_utf8_lossy(&line))
        } else {
            success = false;
            break;
        }
    }

    let build_summary = if success && output_handler.success() {
        "Build succeeded \u{1f60e}"
    } else {
        "Build failed \u{1f622}"
    };

    Notification::new().summary(build_summary).show().unwrap();
}
