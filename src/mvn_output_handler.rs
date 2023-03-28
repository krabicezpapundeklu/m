use crate::console;
use regex::Regex;

lazy_static! {
    static ref ERROR_PATTERN: Regex = Regex::new(r"\[.+?ERROR.+?\]\s*(.+)").unwrap();
    static ref PROJECT_PATTERN: Regex =
        Regex::new(r"\[.*?INFO.*?\].*Building (.+?)\s*(\[\d+/\d+\])?").unwrap();
    static ref STEP_PATTERN: Regex = Regex::new("--- (.+) @.+---").unwrap();
    static ref STATUS_PATTERN: Regex =
        Regex::new(r"\[.*?INFO.*?\].*BUILD (FAILURE|SUCCESS)").unwrap();
    static ref SUMMARY_PATTERN: Regex = Regex::new("Reactor Summary for (.+):").unwrap();
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
    project_name: Option<String>,
}

impl MvnOutputHandler {
    pub fn new(quiet: bool) -> Self {
        MvnOutputHandler {
            quiet,
            state: State::Normal,
            success: true,
            project_name: None,
        }
    }

    pub fn handle_line(&mut self, line: &str) {
        if let Some(error) = match_error(line) {
            self.print_if_quiet(&format!("{error}\n"), true);
        } else if let Some(step) = match_step(line) {
            self.print_if_quiet(&format!("  {step}\n"), false);
            self.state = State::Step;
        } else if let Some((project, progress)) = match_project(line) {
            let title = format!("{progress} {project}\n");

            self.print_if_quiet(&title, false);
            self.project_name = Some(project.to_string());

            console::set_title(&title);
        } else if let Some(success) = match_status(line) {
            if !matches!(self.state, State::Summary) {
                self.print_if_quiet("\n", false);
            }

            self.state = State::Summary;
            self.success = success;
        } else if let Some(project) = match_summary(line) {
            self.print_if_quiet("\n", false);
            self.state = State::Summary;
            self.project_name = Some(project.to_string());
        }

        if !self.quiet || matches!(self.state, State::Summary) {
            println!("{line}");
        }
    }

    pub fn project_name(&self) -> Option<&str> {
        self.project_name.as_deref()
    }

    pub fn success(&self) -> bool {
        self.success
    }

    fn print_if_quiet(&mut self, text: &str, is_error: bool) {
        if !self.quiet {
            return;
        }

        if let State::Step = self.state {
            console::delete_last_line();
            self.state = State::Normal;
        }

        if is_error {
            console::print_error(text);
        } else {
            print!("{text}");
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

fn match_summary(line: &str) -> Option<&str> {
    SUMMARY_PATTERN
        .captures(line)
        .map(|c| c.get(1).unwrap().as_str())
}
