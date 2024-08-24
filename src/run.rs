use std::{
    ffi::{OsStr, OsString},
    io::Write,
    process::{Command, ExitStatus, Stdio},
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

/// Use current user inside the container.
fn current_user() -> anyhow::Result<Vec<OsString>> {
    let uid = format!(
        "{}:{}",
        nix::unistd::getuid().to_string(),
        nix::unistd::getgid().to_string()
    );

    let passwd = OsString::from("/etc/passwd");
    let group = OsString::from("/etc/group");

    let mut args = vec![
        OsString::from("--user"),
        OsString::from(uid),
        OsString::from("--volume"),
        volume_value(passwd.as_os_str(), passwd.as_os_str()),
        OsString::from("--volume"),
        volume_value(group.as_os_str(), group.as_os_str()),
    ];

    for group in nix::unistd::getgroups()? {
        args.extend([
            OsString::from("--group-add"),
            OsString::from(group.to_string()),
        ])
    }

    Ok(args)
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
            Command::new("docker")
                .args(["build", &context, "--tag", &image])
                .spawn()?
                .wait()?
                .success()
                .then_some(image)
                .ok_or(anyhow::anyhow!("failed building the image"))?
        }
        ImageOrBuild::Recipe { recipe } => {
            let image = random_image_tag();
            let mut docker = Command::new("docker")
                .args(["build", ".", "--tag", &image])
                .args(["-f", "-"])
                .stdin(Stdio::piped())
                .spawn()?;

            // Pipe the Dockerfile content through.
            docker
                .stdin
                .take()
                .ok_or(anyhow::anyhow!("cannot access Docker stdin handle"))?
                .write_all(recipe.as_bytes())?;

            docker
                .wait()?
                .success()
                .then_some(image)
                .ok_or(anyhow::anyhow!("failed building the image"))?
        }
    })
}

/// Create docker run command with common arguments.
fn docker_run() -> anyhow::Result<Command> {
    // https://docs.docker.com/reference/cli/docker/container/run/
    let mut command = std::process::Command::new("docker");
    command
        .arg("run")
        .arg("--rm")
        .args(current_dir_as_volume()?)
        .args(docker_sock_as_volume()?)
        ;//.args(current_user()?);
    Ok(command)
}

pub fn execute(mission: Mission) -> Result<ExitStatus, anyhow::Error> {
    let image = image(mission.image_or_build)?;

    let mut docker = docker_run()?
        // Needed for pipes to work.
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
    let mut docker = docker_run()?
        .arg("--interactive")
        .arg("--tty")
        .arg(&image)
        .spawn()?;

    Ok(docker.wait()?)
}
