use crate::providers::OsInformation;
use chrono::{DateTime, Local};
use std::fs;

pub struct OsInstallationDate {}

impl OsInformation for OsInstallationDate {
    fn get() -> Option<(String, String)> {
        fs::metadata("/")
            .and_then(|m| m.created())
            .map(|created| -> DateTime<Local> { DateTime::from(created) })
            .map(|date| format!("{}", date))
            .ok()
            .map(|info| (String::from("Installed"), info))
    }
}
