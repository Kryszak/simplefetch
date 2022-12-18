use std::env::VarError;

pub struct EnvironmentInformationExtractor {}

impl EnvironmentInformationExtractor {
    pub fn get_variable(name: &str) -> Result<String, VarError> {
        std::env::var(name)
    }
}
