use crate::console;
use regex::Regex;

lazy_static! {
    static ref ERROR_PATTERN: Regex = Regex::new(r"\[.+?ERROR.+?\]\s*(.+)").unwrap();
    static ref PROJECT_PATTERN: Regex =
        Regex::new(r"\[.*?INFO.*?\].*Building (.+?)\s*(\[\d+/\d+\])?").unwrap();
    static ref STEP_PATTERN: Regex = Regex::new("--- (.+) @.+---").unwrap();
    static ref STATUS_PATTERN: Regex =
        Regex::new(r"\[.*?INFO.*?\].*BUILD (FAILURE|SUCCESS)").unwrap();
    static ref SUMMARY_PATTERN: Regex = Regex::new("Reactor Summary for .+:").unwrap();
}

enum State {
    Normal,
    Step,
    Summary,
}

pub struct MvnOutputHandler {
    quiet: bool,
    state: State,
    success: bool,
}

impl MvnOutputHandler {
    pub fn new(quiet: bool) -> Self {
        MvnOutputHandler {
            quiet,
            state: State::Normal,
            success: true,
        }
    }

    pub fn handle_line(&mut self, line: &str) {
        if let Some((project, progress)) = match_project(&line) {
            let title = format!("{} {}\n", progress, project);

            if self.quiet {
                self.print(&title, false);
            }

            console::set_title(&title);
        } else if let Some(success) = match_status(&line) {
            if self.quiet && !matches!(self.state, State::Summary) {
                self.print("\n", false);
                self.state = State::Summary;
            }

            self.success = success;
        }

        if !self.quiet || matches!(self.state, State::Summary) {
            println!("{}", line);
            return;
        }

        if let Some(error) = match_error(&line) {
            self.print(&format!("{}\n", error), true);
        } else if let Some(step) = match_step(&line) {
            self.print(&format!("  {}\n", step), false);
            self.state = State::Step;
        } else if match_summary(&line) {
            self.print("\n", false);
            println!("{}", line);
            self.state = State::Summary;
        }
    }

    pub fn success(&self) -> bool {
        self.success
    }

    fn print(&mut self, text: &str, is_error: bool) {
        if let State::Step = self.state {
            console::delete_last_line();
            self.state = State::Normal;
        }

        if is_error {
            console::print_error(text);
        } else {
            print!("{}", text);
        }
    }
}

fn match_error(line: &str) -> Option<&str> {
    ERROR_PATTERN
        .captures(line)
        .map(|c| c.get(1).unwrap().as_str())
}

fn match_project(line: &str) -> Option<(&str, &str)> {
    PROJECT_PATTERN.captures(line).map(|c| {
        (
            c.get(1).unwrap().as_str(),
            c.get(2).map_or("[1/1]", |m| m.as_str()),
        )
    })
}

fn match_status(line: &str) -> Option<bool> {
    STATUS_PATTERN
        .captures(line)
        .map(|c| c.get(1).unwrap().as_str() == "SUCCESS")
}

fn match_step(line: &str) -> Option<&str> {
    STEP_PATTERN
        .captures(line)
        .map(|c| c.get(1).unwrap().as_str())
}

fn match_summary(line: &str) -> bool {
    SUMMARY_PATTERN.is_match(line)
}
