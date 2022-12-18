use crate::files::FileInformationExtractor;
use crate::providers::OsInformation;
use regex::Regex;

pub(crate) struct OsName {}

impl OsInformation for OsName {
    fn get() -> Option<String> {
        let capture_group_name = "os_name";
        let os_name_regex =
            Regex::new(format!("PRETTY_NAME=\"(?P<{}>.*)\"", capture_group_name).as_str()).unwrap();

        FileInformationExtractor::get_information(
            "/etc/os-release",
            os_name_regex,
            capture_group_name,
        )
    }

    fn label() -> String {
        String::from("OS")
    }
}
