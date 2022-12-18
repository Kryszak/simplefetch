#[cfg(test)]
mod environment_information_extractor_test {
    use crate::environment::EnvironmentInformationExtractor;

    #[test]
    fn should_return_value_from_env_variable() {
        // given
        let variable_name = "TEST_VAR_FOR_FETCHER";
        let variable_value = "I am the value";
        std::env::set_var(variable_name, variable_value);

        // when
        let result = EnvironmentInformationExtractor::get_variable(variable_name);

        // then
        assert!(result.is_ok());
        assert_eq!(variable_value, result.unwrap());
    }

    #[test]
    fn should_return_error_if_variable_is_not_set() {
        // given
        let variable_name = "I_AM_NOT_PRESENT";

        // when
        let result = EnvironmentInformationExtractor::get_variable(variable_name);

        // then
        assert!(result.is_err());
    }
}
