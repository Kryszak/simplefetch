use crate::environment::EnvironmentInformationExtractor;

use super::OsInformation;

pub struct OsShell {}

impl OsShell {
    fn basename(path: String) -> Option<String> {
        path.split("/").last().map(|s| String::from(s))
    }
}

impl OsInformation for OsShell {
    fn get() -> Option<(String, String)> {
        EnvironmentInformationExtractor::get_variable("SHELL")
            .ok()
            .and_then(|shell_path| OsShell::basename(shell_path))
            .map(|info| (String::from("Shell"), info))
    }
}
