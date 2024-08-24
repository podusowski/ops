use anyhow::Context;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ImageOrBuild {
    Image { image: String },
    Build { build: String },
}

#[derive(Debug, Deserialize)]
pub struct Mission {
    #[serde(flatten)]
    pub image_or_build: ImageOrBuild,
    pub script: String,
}

#[derive(Debug, Deserialize)]
pub struct Shell {
    #[serde(flatten)]
    pub image_or_build: ImageOrBuild,
}

#[derive(Debug, Deserialize)]
pub struct Plan {
    pub missions: HashMap<String, Mission>,
    pub shell: Option<Shell>,
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
