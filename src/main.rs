mod run;

use std::collections::HashMap;

use anyhow::Context;
use run::run_in_docker;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Mission {
    image: String,
    script: String,
}

#[derive(Debug, Deserialize)]
pub struct Recipe {
    missions: HashMap<String, Mission>,
}

impl Recipe {
    fn from_file(path: &str) -> anyhow::Result<Self> {
        Ok(serde_yaml::from_reader(std::fs::File::open(path)?)?)
    }
}

fn main() -> anyhow::Result<()> {
    let recipe = Recipe::from_file("cio.yaml").with_context(|| "could not load cio.yaml")?;
    println!("{:#?}", recipe);

    for (name, mission) in recipe.missions {
        println!("Launching '{}'", name);
        let status = run_in_docker(mission)?;

        if !status.success() {
            eprintln!("Task failed with status: {:?}", status.code());
        }
    }

    Ok(())
}
