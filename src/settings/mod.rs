pub(crate) mod arg_parser;
pub(crate) mod settings;

pub(crate) use arg_parser::ArgParser;

#[cfg(test)]
pub(crate) use settings::FormattingStyle;