use crate::environment::EnvironmentInformationExtractor;

use super::OsInformation;

pub struct OsWm {}

impl OsInformation for OsWm {
    fn get() -> Option<(String, String)> {
        EnvironmentInformationExtractor::get_variable("DESKTOP_SESSION")
            .ok()
            .map(|info| (String::from("WM"), info))
    }
}
