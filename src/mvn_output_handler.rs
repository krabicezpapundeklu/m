use crate::console;
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
            let title = format!("{} {}", progress, project);

            if self.quiet {
                self.print(&title, false);
            }

            console::set_title(&title);
        }

        if !self.quiet {
            println!("{}", line);
            return;
        }

        if let State::Summary = self.state {
            println!("{}", line);
            return;
        }

        if let Some(error) = self.match_error(&line) {
            self.print(error, true);
        } else if let Some(step) = self.match_step(&line) {
            self.print(&format!("  {}", step), false);
            self.state = State::Step;
        } else if self.match_summary(&line) {
            self.print("", false);
            self.state = State::SummaryFirstLine;
        } else if let State::SummaryFirstLine = self.state {
            self.state = State::Summary;
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

    fn print(&mut self, text: &str, is_error: bool) {
        if let State::Step = self.state {
            console::delete_last_line();
            self.state = State::Normal;
        }

        if is_error {
            console::print_error(text);
        } else {
            println!("{}", text);
        }
    }
}
