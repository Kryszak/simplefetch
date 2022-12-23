use crate::formatters::{Formatter, IconFormatter, LabelFormatter};
use crate::providers::{
    OsInformation, OsInstallationDate, OsKernelVersion, OsName, OsShell, OsTerminal, OsWm,
};
use crate::settings::{ArgParser, FormattingStyle};

mod environment;
mod files;
mod formatters;
mod providers;
mod settings;

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
