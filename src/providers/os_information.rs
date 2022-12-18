pub trait OsInformation {
    fn get() -> Option<(String, String)>;
}
