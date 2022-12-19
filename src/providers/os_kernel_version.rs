use regex::Regex;

use crate::files::FileInformationExtractor;

use super::OsInformation;

pub struct OsKernelVersion {}

impl OsInformation for OsKernelVersion {
    fn get() -> Option<(String, String, String)> {
        let capture_group_name = "kernel_version";
        let os_name_regex = Regex::new(
            format!("Linux version (?P<{}>[a-zA-Z0-9-.]+).*", capture_group_name).as_str(),
        )
        .unwrap();

        FileInformationExtractor::get_information(
            "/proc/version",
            os_name_regex,
            capture_group_name,
        )
        .map(|info| (String::from("\u{1F48E}"), String::from("Kernel"), info))
    }
}
