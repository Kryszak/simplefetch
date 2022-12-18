use std::{fs, io::Error};

pub(crate) struct FileReader {}

impl FileReader {
    pub fn read_file(path: &str) -> Result<String, Error> {
        return fs::read_to_string(path);
    }
}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::env;
    use std::io::Write;

    use super::FileReader;

    #[test]
    fn should_read_file_content() {
        // given
        let file_content = "test";
        let file_path = format!("{}/temp.txt", env::temp_dir().to_str().unwrap());
        File::create(&file_path).unwrap().write_all(file_content.as_bytes()).unwrap();

        // when
        let read_content = FileReader::read_file(&file_path).unwrap();

        // then
        assert_eq!(String::from(file_content), read_content);
    }
}