mod environment;
mod files;
mod formatters;
mod providers;
mod settings;

use crate::formatters::{Formatter, LabelFormatter};
use crate::providers::{OsInformation, OsInstallationDate, OsName, OsWm};
use formatters::IconFormatter;
use providers::{OsKernelVersion, OsShell, OsTerminal};
use settings::{ArgParser, FormattingStyle};

fn main() {
    let settings = ArgParser::parse_settings();
    let formatter: Box<dyn Formatter> = match settings.style {
        FormattingStyle::Left | FormattingStyle::Right => {
            Box::new(LabelFormatter::new(settings.label_padding, settings.style))
        }
        FormattingStyle::Icon => Box::new(IconFormatter::new()),
    };

    let formatting_clojure = |(icon, label, info)| formatter.format_information(icon, label, info);

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
