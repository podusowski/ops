use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Recipe {
    image: String,
    command: String,
}

impl Recipe {
    fn from_file(path: &str) -> anyhow::Result<Self> {
        Ok(serde_yaml::from_reader(std::fs::File::open(path)?)?)
    }
}

fn main() -> anyhow::Result<()> {
    let recipe = Recipe::from_file("cio.yaml").with_context(|| "could not load cio.yaml")?;

    let docker = std::process::Command::new("docker")
        .args(["run"])
        .spawn()?
        .wait()?;

    println!("{:?}", docker);

    Ok(())
}
