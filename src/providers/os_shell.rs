use crate::environment::EnvironmentInformationExtractor;

use super::OsInformation;

pub struct OsShell {}

impl OsShell {
    fn basename(path: String) -> Option<String> {
        path.split('/').last().map(String::from)
    }
}

impl OsInformation for OsShell {
    fn get() -> Option<(String, String, String)> {
        EnvironmentInformationExtractor::get_variable("SHELL")
            .ok()
            .and_then(OsShell::basename)
            .map(|info| (String::from("\u{1F41A}"), String::from("Shell"), info))
    }
}
