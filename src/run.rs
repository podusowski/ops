use std::{
    ffi::{OsStr, OsString},
    io::Write,
    process::{ExitStatus, Stdio},
};

use crate::plan::Mission;

/// Format a value for Docker's `--volume` argument.
fn volume_value(source: &OsStr, destination: &OsStr) -> OsString {
    let mut value = OsString::with_capacity(source.len() + destination.len() + 1);
    value.push(source);
    value.push(":");
    value.push(destination);
    value
}

/// Mount current directory as volume with the same path.
fn current_dir_as_volume() -> anyhow::Result<Vec<OsString>> {
    let current_dir = std::env::current_dir()?;
    Ok(vec![
        OsString::from("--volume"),
        volume_value(current_dir.as_os_str(), current_dir.as_os_str()),
        OsString::from("--workdir"),
        current_dir.as_os_str().to_owned(),
    ])
}

pub fn run_in_docker(mission: Mission) -> Result<ExitStatus, anyhow::Error> {
    // https://docs.docker.com/reference/cli/docker/container/run/
    let mut docker = std::process::Command::new("docker")
        .arg("run")
        .arg("--rm")
        .args(current_dir_as_volume()?)
        // Script will be piped via stdin.
        .arg("--interactive")
        .stdin(Stdio::piped())
        .arg(&mission.image)
        .spawn()?;

    docker
        .stdin
        .take()
        .ok_or(anyhow::anyhow!("cannot access Docker stdin handle"))?
        .write_all(mission.script.as_bytes())?;

    Ok(docker.wait()?)
}
