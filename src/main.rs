mod files;
mod formatters;
mod providers;
mod settings;

use crate::formatters::{Formatter, LabelFormatter};
use crate::providers::{OsInformation, OsInstallationDate, OsName};
use settings::ArgParser;

fn main() {
    let settings = ArgParser::parse_settings();
    let formatter = LabelFormatter::new(settings.label_padding, settings.style);

    vec![
        OsName::get().map(|info| formatter.format_information(OsName::label(), info)),
        OsInstallationDate::get()
            .map(|info| formatter.format_information(OsInstallationDate::label(), info)),
    ]
    .into_iter()
    .flatten()
    .for_each(|info| println!("{}", info));
}
