use std::{
    ffi::{OsStr, OsString},
    io::Write,
    process::{ExitStatus, Stdio},
};

use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Recipe {
    image: String,
    script: String,
}

impl Recipe {
    fn from_file(path: &str) -> anyhow::Result<Self> {
        Ok(serde_yaml::from_reader(std::fs::File::open(path)?)?)
    }
}

/// Format a value for Docker's `--volume` argument.
fn volume_value(source: &OsStr, destination: &OsStr) -> OsString {
    let mut value = OsString::with_capacity(source.len() + destination.len() + 1);
    value.push(source);
    value.push(":");
    value.push(destination);
    value
}

fn run_in_docker(recipe: Recipe) -> Result<ExitStatus, anyhow::Error> {
    let current_dir = std::env::current_dir()?;

    // https://docs.docker.com/reference/cli/docker/container/run/
    let mut docker = std::process::Command::new("docker")
        .arg("run")
        .arg("--rm")
        // Mount current directory with the same path.
        .args([
            OsStr::new("--volume"),
            &volume_value(current_dir.as_os_str(), current_dir.as_os_str()),
        ])
        .args([OsStr::new("--workdir"), current_dir.as_os_str()])
        // Script will be piped via stdin.
        .arg("--interactive")
        .stdin(Stdio::piped())
        .arg(&recipe.image)
        .spawn()?;

    docker
        .stdin
        .take()
        .ok_or(anyhow::anyhow!("cannot access Docker stdin handle"))?
        .write_all(recipe.script.as_bytes())?;

    Ok(docker.wait()?)
}

fn main() -> anyhow::Result<()> {
    let recipe = Recipe::from_file("cio.yaml").with_context(|| "could not load cio.yaml")?;
    let status = run_in_docker(recipe)?;

    if !status.success() {
        eprintln!("Task failed with status: {:?}", status.code());
    }

    Ok(())
}
