#[cfg(test)]
mod file_information_extractor_test {
    use crate::files::FileInformationExtractor;
    use regex::Regex;
    use std::{env, fs::File, io::Write};

    #[test]
    fn should_extract_information_from_file() {
        // given
        let file_content = "NAME=\"Arch Linux\"
        PRETTY_NAME=\"Arch Linux\"";
        let file_path = format!("{}/os-release", env::temp_dir().to_str().unwrap());
        File::create(&file_path)
            .unwrap()
            .write_all(file_content.as_bytes())
            .unwrap();

        let capture_group = "os_name";
        let regex =
            Regex::new(format!("PRETTY_NAME=\"(?P<{}>.*)\"", capture_group).as_str()).unwrap();

        // when
        let result = FileInformationExtractor::get_information(&file_path, regex, capture_group);

        // then
        assert!(result.is_some());
        assert_eq!(String::from("Arch Linux"), result.unwrap());
    }

    #[test]
    fn should_return_none_if_information_is_not_found_in_file() {
        // given
        let file_content = "ID=arch
        BUILD_ID=rolling
        ANSI_COLOR=\"38;2;23;147;209\"";
        let file_path = format!("{}/os-release-2", env::temp_dir().to_str().unwrap());
        File::create(&file_path)
            .unwrap()
            .write_all(file_content.as_bytes())
            .unwrap();

        let capture_group = "os_name";
        let regex =
            Regex::new(format!("PRETTY_NAME=\"(?P<{}>.*)\"", capture_group).as_str()).unwrap();

        // when
        let result = FileInformationExtractor::get_information(
            "i/do/not/exist/release",
            regex,
            capture_group,
        );

        // then
        assert!(result.is_none());
    }

    #[test]
    fn should_return_none_if_file_does_not_exist() {
        // given
        let capture_group = "os_name";
        let regex =
            Regex::new(format!("PRETTY_NAME=\"(?P<{}>.*)\"", capture_group).as_str()).unwrap();

        // when
        let result = FileInformationExtractor::get_information(
            "i/do/not/exist/release",
            regex,
            capture_group,
        );

        // then
        assert!(result.is_none());
    }
}
