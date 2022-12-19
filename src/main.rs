mod environment;
mod files;
mod formatters;
mod providers;
mod settings;

use crate::formatters::{Formatter, LabelFormatter};
use crate::providers::{OsInformation, OsInstallationDate, OsName, OsWm};
use providers::{OsKernelVersion, OsShell, OsTerminal};
use settings::ArgParser;

fn main() {
    let settings = ArgParser::parse_settings();
    let formatter = LabelFormatter::new(settings.label_padding, settings.style);
    let formatting_clojure = |(label, info)| formatter.format_information(label, info);

    [
        OsName::get(),
        OsKernelVersion::get(),
        OsInstallationDate::get(),
        OsWm::get(),
        OsShell::get(),
        OsTerminal::get(),
    ]
    .into_iter()
    .flatten()
    .map(formatting_clojure)
    .for_each(|info| println!("{}", info));
}
