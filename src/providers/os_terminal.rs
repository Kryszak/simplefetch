use crate::environment::EnvironmentInformationExtractor;

use super::OsInformation;

pub struct OsTerminal {}

impl OsInformation for OsTerminal {
    fn get() -> Option<(String, String, String)> {
        EnvironmentInformationExtractor::get_variable("TERM_PROGRAM")
            .ok()
            .map(|info| (String::from("\u{1F528}"), String::from("Terminal"), info))
    }
}
