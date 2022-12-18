mod environment;
mod files;
mod formatters;
mod providers;
mod settings;

use crate::formatters::{Formatter, LabelFormatter};
use crate::providers::{OsInformation, OsInstallationDate, OsName, OsWm};
use settings::ArgParser;

fn main() {
    let settings = ArgParser::parse_settings();
    let formatter = LabelFormatter::new(settings.label_padding, settings.style);
    let formatting_clojure = |(label, info)| formatter.format_information(label, info);

    vec![
        OsName::get().map(formatting_clojure),
        OsInstallationDate::get().map(formatting_clojure),
        OsWm::get().map(formatting_clojure),
    ]
    .into_iter()
    .flatten()
    .for_each(|info| println!("{}", info));
}
