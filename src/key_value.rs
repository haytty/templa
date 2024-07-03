use std::collections::HashMap;
use std::fs;
use std::path::Path;
use yaml_rust2::{Yaml, YamlLoader};
use crate::error::Error;

#[derive(Debug)]
pub struct KeyValue(HashMap<String, String>);

impl KeyValue {
    pub fn empty() -> Self {
        Self(HashMap::new())
    }

    pub fn from_vec_string(key_value_args: &Vec<String>) -> Self {
        let mut map = HashMap::new();

        for key_value in key_value_args {
            let key_values: Vec<&str> = key_value.split("=").collect();
            let key = key_values[0];
            let value = key_values[1];
            map.insert(key.to_string(), value.to_string());
        }

        Self(map)
    }

    pub fn from_config<T: AsRef<Path>>(path: T) -> Result<Self, Error> {
        let contents = fs::read_to_string(path).map_err(|_| Error::FileLoadError)?;
        let docs: Vec<Yaml> = YamlLoader::load_from_str(&contents).map_err(|_| Error::YamlParseError)?;

        let mut map = HashMap::new();
        for doc in docs {
            match doc {
                Yaml::Hash(hash) => {
                    for (key, value) in hash {
                        match (key, value) {
                            (Yaml::String(key), Yaml::String(value)) => {
                                map.insert(key, value);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(Self(map))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn merge(&mut self, other: &Self) {
        self.0.extend(other.0.clone())
    }

    pub fn materialize(&self) -> HashMap<String, String> {
        self.0.clone()
    }
}