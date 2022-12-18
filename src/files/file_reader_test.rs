#[cfg(test)]
mod file_reader_test {
    use std::env;
    use std::fs::File;
    use std::io::Write;

    use crate::files::FileReader;

    #[test]
    fn should_read_file_content() {
        // given
        let file_content = "test";
        let file_path = format!("{}/temp.txt", env::temp_dir().to_str().unwrap());
        File::create(&file_path)
            .unwrap()
            .write_all(file_content.as_bytes())
            .unwrap();

        // when
        let result = FileReader::read_file(&file_path);

        // then
        assert!(result.is_ok());
        assert_eq!(String::from(file_content), result.unwrap());
    }

    #[test]
    fn should_return_error_if_file_does_not_exist() {
        // given
        let file_path = "/i/definitely/do/not/exist/file.txt";

        // when
        let result = FileReader::read_file(file_path);

        // then
        assert!(result.is_err());
    }
}
