use crate::ansi_console;
use regex::Regex;

lazy_static! {
    static ref ERROR_PATTERN: Regex = Regex::new(r"\[.+?ERROR.+?\]\s*(.+)").unwrap();
    static ref PROJECT_PATTERN: Regex = Regex::new(r"Building (.+?)\s+(\[\d+/\d+\])").unwrap();
    static ref STEP_PATTERN: Regex = Regex::new("--- (.+) @.+---").unwrap();
    static ref SUMMARY_PATTERN: Regex = Regex::new("Reactor Summary for .+:").unwrap();
}

enum State {
    Normal,
    Step,
    SummaryFirstLine,
    Summary,
}

pub struct MvnOutputHandler {
    quiet: bool,
    state: State,
}

impl MvnOutputHandler {
    pub fn new(quiet: bool) -> Self {
        MvnOutputHandler {
            quiet,
            state: State::Normal,
        }
    }

    pub fn handle_line(&mut self, line: &str) {
        if let Some((project, progress)) = self.match_project(&line) {
            ansi_console::set_title(&format!("{} {}", progress, project));

            if self.quiet {
                println!("{} {}", progress, project);
            }
        }

        if !self.quiet {
            println!("{}", line);
            return;
        }

        match self.state {
            State::Step => {
                ansi_console::delete_last_line();
                self.state = State::Normal;
            }
            State::SummaryFirstLine => {
                self.state = State::Summary;
                return;
            }
            State::Summary => {
                println!("{}", line);
                return;
            }
            _ => {}
        }

        if let Some(error) = self.match_error(&line) {
            ansi_console::print_error(&error);
        } else if let Some(step) = self.match_step(&line) {
            self.state = State::Step;
            println!("  {}", step);
        } else if self.match_summary(&line) {
            self.state = State::SummaryFirstLine;
            println!();
        }
    }

    fn match_error<'a>(&self, line: &'a str) -> Option<&'a str> {
        ERROR_PATTERN
            .captures(line)
            .map(|c| c.get(1).unwrap().as_str())
    }

    fn match_project<'a>(&self, line: &'a str) -> Option<(&'a str, &'a str)> {
        PROJECT_PATTERN
            .captures(line)
            .map(|c| (c.get(1).unwrap().as_str(), c.get(2).unwrap().as_str()))
    }

    fn match_step<'a>(&self, line: &'a str) -> Option<&'a str> {
        STEP_PATTERN
            .captures(line)
            .map(|c| c.get(1).unwrap().as_str())
    }

    fn match_summary(&self, line: &str) -> bool {
        SUMMARY_PATTERN.is_match(line)
    }
}
