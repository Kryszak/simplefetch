use jiff::Timestamp;

use crate::providers::OsInformation;
use std::fs;

pub struct OsInstallationDate {}

impl OsInformation for OsInstallationDate {
    fn get() -> Option<(String, String, String)> {
        fs::metadata("/")
            .and_then(|m| m.created())
            .ok()
            .and_then(|created| Timestamp::try_from(created).ok())
            .map(|date| {
                (
                    String::from("\u{1F680}"),
                    String::from("Installed"),
                    date.strftime("%Y-%m-%d %H:%M:%S %:z").to_string(),
                )
            })
    }
}
