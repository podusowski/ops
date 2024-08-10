use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Mission {
    pub image: String,
    pub script: String,
}

#[derive(Debug, Deserialize)]
pub struct Plan {
    pub missions: HashMap<String, Mission>,
}

impl Plan {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        Ok(serde_yaml::from_reader(std::fs::File::open(path)?)?)
    }
}
