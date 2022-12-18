#[cfg(test)]
mod label_formatter_test {
    use colored::Colorize;

    use crate::formatters::{Formatter, LabelFormatter};
    use crate::settings::FormattingStyle::{Left, Right};

    #[test]
    fn should_format_with_padding_to_right() {
        // given
        let formatter = LabelFormatter::new(3, Right);

        // when
        let result = formatter.format_information(String::from("OS"), String::from("Arch Linux"));

        // then
        assert_eq!(
            format!("{} Arch Linux", " OS:".bright_cyan().bold().to_string()),
            result
        );
    }

    #[test]
    fn should_format_with_padding_to_left() {
        // given
        let formatter = LabelFormatter::new(3, Left);

        // when
        let result = formatter.format_information(String::from("OS"), String::from("Arch Linux"));

        // then
        assert_eq!(
            format!("{} Arch Linux", "OS :".bright_cyan().bold().to_string()),
            result
        );
    }
}
