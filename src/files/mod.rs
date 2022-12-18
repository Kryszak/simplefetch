pub(crate) mod file_information_extractor;
pub(crate) mod file_reader;

pub(crate) use file_information_extractor::FileInformationExtractor;
pub(crate) use file_reader::FileReader;

#[cfg(test)]
mod file_reader_test;
#[cfg(test)]
mod file_information_extractor_test;