use anyhow::Context;
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
    /// Load the plan from a YAML file.
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        Self::_from_file(path).with_context(|| format!("could not load '{}'", path))
    }

    fn _from_file(path: &str) -> anyhow::Result<Self> {
        Ok(serde_yaml::from_reader(std::fs::File::open(path)?)?)
    }
}
