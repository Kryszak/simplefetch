use super::settings::{FormattingStyle::Right, Settings};
use argparse::{ArgumentParser, Parse, Store};

pub struct ArgParser {}

impl ArgParser {
    pub fn parse_settings() -> Settings {
        let mut settings = Settings::new(10, Right);
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

        return settings;
    }
}
