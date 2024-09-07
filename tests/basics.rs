use std::{io::Write, os::unix::fs::MetadataExt, path::PathBuf, process::Command};

use assert_cmd::assert::OutputAssertExt;
use predicates::str::contains;

struct Workspace(pub PathBuf);

impl Workspace {
    fn new(ops_yaml: &str) -> Self {
        let dir: PathBuf =
            PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(uuid::Uuid::new_v4().to_string());
        std::fs::create_dir(&dir).unwrap();
        std::fs::File::create(dir.join("cio.yaml"))
            .unwrap()
            .write_all(ops_yaml.as_bytes())
            .unwrap();
        Self(dir)
    }

    fn with_dockerfile(self, dockerfile: &str) -> Self {
        std::fs::File::create(self.0.join("Dockerfile"))
            .unwrap()
            .write_all(dockerfile.as_bytes())
            .unwrap();
        self
    }
}

impl Drop for Workspace {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}

static PROGRAM: &'static str = env!("CARGO_BIN_EXE_ops");

#[test]
fn hello_world() {
    let workspace = Workspace::new(
        "
        missions:
            hello-world:
                image: busybox
                script: echo hello world",
    );

    Command::new(PROGRAM)
        .arg("execute")
        .current_dir(&workspace.0)
        .assert()
        .success();
}

#[test]
fn failing_mission() {
    let workspace = Workspace::new(
        "
        missions:
            hello-world:
                image: busybox
                script: false",
    );

    Command::new(PROGRAM)
        .arg("execute")
        .current_dir(&workspace.0)
        .assert()
        .failure();
}

#[test]
fn one_mission_successful_but_other_fails() {
    // All missions have to be successful for whole thing to be too.
    let workspace = Workspace::new(
        "
        missions:
            success:
                image: busybox
                script: true
            failure:
                image: busybox
                script: false",
    );

    Command::new(PROGRAM)
        .arg("execute")
        .current_dir(&workspace.0)
        .assert()
        .failure();
}

#[test]
fn execute_only_when_matches_pattern() {
    let workspace = Workspace::new(
        "
        missions:
            success:
                image: busybox
                script: true
            failure:
                image: busybox
                script: false",
    );

    Command::new(PROGRAM)
        .args(["execute", "success"])
        .current_dir(&workspace.0)
        .assert()
        .success();
}

#[test]
fn docker_build() {
    let workspace = Workspace::new(
        "
        missions:
            hello-world:
                build: .
                script: true",
    )
    .with_dockerfile("FROM busybox");

    Command::new(PROGRAM)
        .arg("execute")
        .current_dir(&workspace.0)
        .assert()
        .success();
}

#[test]
fn docker_build_from_recipe() {
    let workspace = Workspace::new(
        "
        missions:
            hello-world:
                recipe: FROM busybox
                script: true",
    );

    Command::new(PROGRAM)
        .arg("execute")
        .current_dir(&workspace.0)
        .assert()
        .success();
}

#[test]
fn forwarding_user() {
    let workspace = Workspace::new(
        "
        missions:
            hello-world:
                image: busybox
                forward_user: True
                script: touch foo",
    );

    Command::new(PROGRAM)
        .arg("execute")
        .current_dir(&workspace.0)
        .assert()
        .success();

    // This might be false positive if the test is run as root.
    let metadata = std::fs::metadata(workspace.0.join("foo")).unwrap();
    let current_uid = nix::unistd::Uid::current().as_raw();
    assert_eq!(metadata.uid(), current_uid);
}

#[test]
fn shell_accepts_args() {
    let workspace = Workspace::new(
        "
        shell:
            image: busybox",
    );

    Command::new(PROGRAM)
        .args(["shell", "echo", "hello", "world"])
        .current_dir(&workspace.0)
        .assert()
        .stdout(contains("hello world"))
        .success();
}
