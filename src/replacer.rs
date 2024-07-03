use std::fs;
use std::path::Path;
use regex::Regex;
use crate::key_value::KeyValue;
use crate::error::Error;

pub struct Replacer {
    content: String,
    key_value: KeyValue,
}

impl Replacer {
    pub fn new(content: String, key_value: KeyValue) -> Self {
        Self {
            content,
            key_value,
        }
    }
    pub fn from_file<T: AsRef<Path>>(path: T, key_value: KeyValue) -> Result<Self, Error> {
        let content = fs::read_to_string(path).map_err(|_| Error::FileLoadError)?;
        let replacer = Replacer::new(content, key_value);
        Ok(replacer)
    }

    pub fn replace(&self) -> String {
        let mut replaced_content = self.content.clone();
        for (key, value) in self.key_value.materialize() {
            let regex_str = format!(r"\{{\{{\s*{}\s*\}}\}}", regex::escape(&key));
            let re = Regex::new(&regex_str).unwrap();
            replaced_content = re.replace_all(&replaced_content, value).into_owned();
        }

        replaced_content
    }
}