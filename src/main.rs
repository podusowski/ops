mod run;

use anyhow::Context;
use run::run_in_docker;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Recipe {
    image: String,
    script: String,
}

impl Recipe {
    fn from_file(path: &str) -> anyhow::Result<Self> {
        Ok(serde_yaml::from_reader(std::fs::File::open(path)?)?)
    }
}

fn main() -> anyhow::Result<()> {
    let recipe = Recipe::from_file("cio.yaml").with_context(|| "could not load cio.yaml")?;
    let status = run_in_docker(recipe)?;

    if !status.success() {
        eprintln!("Task failed with status: {:?}", status.code());
    }

    Ok(())
}
