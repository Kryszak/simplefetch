pub trait Formatter {
    fn format_information(&self, icon: String, label: String, information: String) -> String;
}
