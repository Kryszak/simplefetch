use crate::files::FileReader;
use regex::Regex;

pub struct FileInformationExtractor {}

impl FileInformationExtractor {
    pub fn get_information(file_path: &str, regex: Regex, capture_group: &str) -> Option<String> {
        return FileReader::read_file(file_path)
            .ok()
            .as_deref()
            .and_then(|content| regex.captures(content))
            .and_then(|captures| captures.name(capture_group))
            .map(|matched| String::from(matched.as_str()));
    }
}
