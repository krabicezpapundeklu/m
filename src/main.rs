use std::{error::Error, process::Stdio};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::{process::Command, spawn};

mod ansi_console;
mod mvn_output_handler;

#[macro_use]
extern crate lazy_static;

const QUIET_ARG: &str = "--m-quiet";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    ansi_console::setup();

    let mvn_path = which::which("mvn").expect("cannot find mvn");

    let (m_args, mvn_args): (Vec<_>, Vec<_>) =
        std::env::args().skip(1).partition(|arg| arg == QUIET_ARG);

    let mut mvn = Command::new(mvn_path)
        .args(mvn_args)
        .env("MAVEN_OPTS", "-Djansi.passthrough=true")
        .stdout(Stdio::piped())
        .spawn()
        .expect("cannot start mvn");

    let output = mvn.stdout.take().expect("no stdout");
    let mut reader = BufReader::new(output).split(b'\n');

    spawn(async move {
        let _ = mvn.wait().await;
    });

    let quiet = m_args.iter().any(|arg| arg == QUIET_ARG);
    let mut output_handler = mvn_output_handler::MvnOutputHandler::new(quiet);

    while let Some(line) = reader.next_segment().await? {
        output_handler.handle_line(&String::from_utf8_lossy(&line));
    }

    Ok(())
}
