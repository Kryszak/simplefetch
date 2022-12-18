pub trait Formatter {
    fn format_information(&self, section_name: String, information: String) -> String;
}
