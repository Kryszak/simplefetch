use std::{fs, io::Error};

pub(crate) struct FileReader {}

impl FileReader {
    pub fn read_file(path: &str) -> Result<String, Error> {
        return fs::read_to_string(path);
    }
}
