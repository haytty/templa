#[derive(Debug)]
pub enum Error {
    FileLoadError,
    YamlParseError,
    EmptyKeyValueError,
}