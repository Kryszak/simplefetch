use super::print_settings::{FormattingStyle::Right, PrintSettings};
use argparse::{ArgumentParser, Parse, Store};

pub struct ArgParser {}

impl ArgParser {
    pub fn parse_settings() -> PrintSettings {
        let mut settings = PrintSettings::new(9, Right);
        {
            let mut ap = ArgumentParser::new();
            ap.set_description("Simple Linux system information fetcher.");
            ap.refer(&mut settings.label_padding).add_option(
                &["--label-padding"],
                Store,
                "Padding of section labels",
            );
            ap.refer(&mut settings.style).add_option(
                &["--style"],
                Parse,
                "Style of display. Available: left,right",
            );
            ap.parse_args_or_exit();
        }
        settings
    }
}
