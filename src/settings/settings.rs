use argparse::FromCommandLine;

pub struct Settings {
    pub label_padding: usize,
    pub style: FormattingStyle,
}

impl Settings {
    pub fn new(label_padding: usize, style: FormattingStyle) -> Self {
        return Settings {
            label_padding,
            style,
        };
    }
}

pub enum FormattingStyle {
    Left,
    Right,
}

impl FromCommandLine for FormattingStyle {
    fn from_argument(s: &str) -> Result<Self, String> {
        match s {
            "right" => Ok(FormattingStyle::Right),
            "left" => Ok(FormattingStyle::Left),
            _ => Err(String::from("Allowed values: left, right")),
        }
    }
}
