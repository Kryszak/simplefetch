use jiff::Zoned;

use crate::providers::OsInformation;
use std::fs;

pub struct OsInstallationDate {}

impl OsInformation for OsInstallationDate {
    fn get() -> Option<(String, String, String)> {
        fs::metadata("/")
            .and_then(|m| m.created())
            .map(|created| Zoned::try_from(created).unwrap())
            .map(|date| date.strftime("%Y-%m-%d %H:%M:%S %:z").to_string())
            .ok()
            .map(|info| (String::from("\u{1F680}"), String::from("Installed"), info))
    }
}
