use std::{
    ffi::{OsStr, OsString},
    io::Write,
    process::{ExitStatus, Stdio},
};

use crate::plan::{ImageOrBuild, Mission, Shell};

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

/// Mount Docker's socket, letting containers use host's daemon.
fn docker_sock_as_volume() -> anyhow::Result<Vec<OsString>> {
    let docker_sock = OsStr::new("/var/run/docker.sock");
    Ok(vec![
        OsString::from("--volume"),
        volume_value(docker_sock, docker_sock),
    ])
}

fn random_image_tag() -> String {
    // Prepending it with dummy repository prevents Docker to look for it on Docker Hub. It is also
    // a bit harder to accidentally push it there.
    format!("cio.local/{}", uuid::Uuid::new_v4())
}

/// Return image tag, building it if necessary.
fn image(image_or_build: ImageOrBuild) -> anyhow::Result<String> {
    Ok(match image_or_build {
        ImageOrBuild::Image { image } => image,
        ImageOrBuild::Build { build: context } => {
            let image = random_image_tag();
            std::process::Command::new("docker")
                .args(["build", &context, "--tag", &image])
                .spawn()?
                .wait()?
                .success()
                .then_some(image)
                .ok_or(anyhow::anyhow!("failed building the image"))?
        }
    })
}

pub fn execute(mission: Mission) -> Result<ExitStatus, anyhow::Error> {
    let image = image(mission.image_or_build)?;

    // https://docs.docker.com/reference/cli/docker/container/run/
    let mut docker = std::process::Command::new("docker")
        .arg("run")
        .arg("--rm")
        .args(current_dir_as_volume()?)
        .args(docker_sock_as_volume()?)
        // Script will be piped via stdin.
        .arg("--interactive")
        .stdin(Stdio::piped())
        .arg(&image)
        .spawn()?;

    docker
        .stdin
        .take()
        .ok_or(anyhow::anyhow!("cannot access Docker stdin handle"))?
        .write_all(mission.script.as_bytes())?;

    Ok(docker.wait()?)
}

pub fn shell(shell: Shell) -> Result<ExitStatus, anyhow::Error> {
    let image = image(shell.image_or_build)?;

    // https://docs.docker.com/reference/cli/docker/container/run/
    let mut docker = std::process::Command::new("docker")
        .arg("run")
        .arg("--rm")
        .args(current_dir_as_volume()?)
        .args(docker_sock_as_volume()?)
        .arg("--interactive")
        .arg("--tty")
        .arg(&image)
        .spawn()?;

    Ok(docker.wait()?)
}
