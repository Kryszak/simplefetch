pub trait OsInformation {
    fn get() -> Option<String>;
    fn label() -> String;
}
