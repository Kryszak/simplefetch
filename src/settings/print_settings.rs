use argparse::FromCommandLine;

pub struct PrintSettings {
    pub label_padding: usize,
    pub style: FormattingStyle,
}

impl PrintSettings {
    pub fn new(label_padding: usize, style: FormattingStyle) -> Self {
        PrintSettings {
            label_padding,
            style,
        }
    }
}

pub enum FormattingStyle {
    Left,
    Right,
    Icon
}

impl FromCommandLine for FormattingStyle {
    fn from_argument(s: &str) -> Result<Self, String> {
        match s {
            "right" => Ok(FormattingStyle::Right),
            "left" => Ok(FormattingStyle::Left),
            "icon" => Ok(FormattingStyle::Icon),
            _ => Err(String::from("Allowed values: left, right, icon")),
        }
    }
}
