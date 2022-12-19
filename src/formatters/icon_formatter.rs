use colored::{ColoredString, Colorize};

use super::Formatter;

pub struct IconFormatter {}
impl IconFormatter {
    pub fn new() -> Self {
        IconFormatter {}
    }

    fn colorize_icon(&self, icon: String) -> ColoredString {
        format!("{}:", icon).bright_cyan().bold()
    }
}

impl Formatter for IconFormatter {
    fn format_information(&self, icon: String, _: String, information: String) -> String {
        format!("{} {}", self.colorize_icon(icon), information)
    }
}
