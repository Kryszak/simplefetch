use colored::{ColoredString, Colorize};

use crate::settings::print_settings::FormattingStyle;

use super::Formatter;

pub struct LabelFormatter {
    label_padding: usize,
    style: FormattingStyle,
}

impl LabelFormatter {
    pub fn new(label_padding: usize, style: FormattingStyle) -> Self {
        LabelFormatter {
            label_padding,
            style,
        }
    }

    fn get_padded_section_name(&self, section_name: String) -> ColoredString {
        match self.style {
            FormattingStyle::Left => format!(
                "{section_name:<padding_length$}:",
                section_name = section_name,
                padding_length = self.label_padding
            ),
            FormattingStyle::Right => format!(
                "{section_name:>padding_length$}:",
                section_name = section_name,
                padding_length = self.label_padding
            ),
        }
        .bright_cyan()
        .bold()
    }
}

impl Formatter for LabelFormatter {
    fn format_information(&self, section_name: String, information: String) -> String {
        format!(
            "{} {}",
            self.get_padded_section_name(section_name),
            information
        )
    }
}
