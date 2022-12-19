use crate::environment::EnvironmentInformationExtractor;

use super::OsInformation;

pub struct OsTerminal {}

impl OsInformation for OsTerminal {
    fn get() -> Option<(String, String)> {
        EnvironmentInformationExtractor::get_variable("TERM_PROGRAM")
            .ok()
            .map(|info| (String::from("Terminal"), info))
    }
}