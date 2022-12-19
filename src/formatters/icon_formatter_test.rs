#[cfg(test)]
mod icon_formatter_test {
    use crate::formatters::{IconFormatter, Formatter};


    #[test]
    fn should_format_with_icon() {
        // given
        let formatter = IconFormatter::new();
        
        // when
        let result = formatter.format_information(String::from("\u{1F528}"), String::new(), String::from("info"));

        // then
        assert_eq!(String::from("\u{1b}[1;96m🔨:\u{1b}[0m info"), result);
    }
}