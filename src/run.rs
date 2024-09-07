use std::{
    ffi::{OsStr, OsString},
    io::{Read, Write},
    process::{Command, ExitStatus, Stdio},
};

use tempfile::NamedTempFile;

use crate::{
    command::{CommandEx, ExitStatusEx},
    plan::{ContainerOptions, ImageOrBuild, Mission, Shell},
};

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
    let uid = format!("{}:{}", nix::unistd::getuid(), nix::unistd::getgid());

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

/// Temporary file for Docker's `--iidfile` argument.
struct IidFile(NamedTempFile);

impl IidFile {
    fn new() -> anyhow::Result<Self> {
        Ok(Self(NamedTempFile::new()?))
    }

    fn path(&self) -> &std::path::Path {
        self.0.path()
    }

    /// Read image ID from the file written by Docker.
    fn image(&self) -> anyhow::Result<String> {
        let mut image = String::new();
        std::fs::File::open(self.0.path())?.read_to_string(&mut image)?;
        Ok(image)
    }
}

fn docker_build(context: &str) -> anyhow::Result<(IidFile, Command)> {
    let iidfile = IidFile::new()?;
    let mut command = Command::new("docker");
    command.args([
        OsStr::new("build"),
        OsStr::new(context),
        OsStr::new("--iidfile"),
        iidfile.path().as_os_str(),
    ]);
    Ok((iidfile, command))
}

/// Return image tag, building it if necessary.
fn image(image_or_build: &ImageOrBuild) -> anyhow::Result<String> {
    Ok(match image_or_build {
        ImageOrBuild::Image { image } => image.to_owned(),
        ImageOrBuild::Build { build: context } => {
            // https://docs.docker.com/reference/cli/docker/buildx/build/
            let (iidfile, mut command) = docker_build(context)?;
            command.spawn()?.wait()?.exit_ok_()?;
            iidfile.image()?
        }
        ImageOrBuild::Recipe { recipe } => {
            let (iidfile, mut command) = docker_build(".")?;
            let mut docker = command.args(["-f", "-"]).stdin(Stdio::piped()).spawn()?;

            // Pipe the Dockerfile content through.
            docker
                .stdin
                .take()
                .ok_or(anyhow::anyhow!("cannot access Docker stdin handle"))?
                .write_all(recipe.as_bytes())?;

            docker.wait()?.exit_ok_()?;
            iidfile.image()?
        }
    })
}

/// Create docker run command with common arguments.
fn docker_run(container_options: &ContainerOptions) -> anyhow::Result<Command> {
    // https://docs.docker.com/reference/cli/docker/container/run/
    let mut command = Command::new("docker");
    command
        .arg("run")
        .arg("--rm")
        .args(current_dir_as_volume()?)
        .args(docker_sock_as_volume()?); //.args(current_user()?);
    if container_options.forward_user {
        command.args(current_user()?);
    }
    Ok(command)
}

pub fn execute(mission: Mission) -> Result<ExitStatus, anyhow::Error> {
    let image = image(&mission.container_options.image_or_build)?;

    let mut docker = docker_run(&mission.container_options)?
        // Needed for pipes to work.
        .arg("--interactive")
        .stdin(Stdio::piped())
        .arg(&image)
        .debug()
        .spawn()?;

    docker
        .stdin
        .take()
        .ok_or(anyhow::anyhow!("cannot access Docker stdin handle"))?
        .write_all(mission.script.as_bytes())?;

    Ok(docker.wait()?)
}

pub fn shell(shell: Shell) -> Result<ExitStatus, anyhow::Error> {
    let image = image(&shell.container_options.image_or_build)?;

    // https://docs.docker.com/reference/cli/docker/container/run/
    let mut docker = docker_run(&shell.container_options)?
        .arg("--interactive")
        .arg("--tty")
        .arg(&image)
        .debug()
        .spawn()?;

    Ok(docker.wait()?)
}
