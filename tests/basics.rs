use std::{io::Write, path::PathBuf, process::Command};

struct TemporaryWorkspace(pub PathBuf);

impl TemporaryWorkspace {
    fn new(ops_yaml: &str) -> Self {
        let dir: PathBuf = format!("/var/tmp/{}", uuid::Uuid::new_v4()).into();
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::File::create(dir.join("cio.yaml"))
            .unwrap()
            .write_all(ops_yaml.as_bytes())
            .unwrap();
        Self(dir)
    }
}

impl Drop for TemporaryWorkspace {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn hello_world() {
    let workspace = TemporaryWorkspace::new(
        "
        missions:
        hello-world:
            image: busybox
            script: echo hello world",
    );

    let program = env!("CARGO_BIN_EXE_cio");

    let success = Command::new(program)
        .arg("execute")
        .current_dir(&workspace.0)
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success();

    assert!(success);
}

#[test]
fn failing_mission() {
    let program = env!("CARGO_BIN_EXE_cio");
    let workspaces = std::path::Path::new(file!())
        .parent()
        .unwrap()
        .join("workspaces");

    let success = Command::new(program)
        .arg("execute")
        .current_dir(workspaces.join("failing_mission"))
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success();

    assert!(!success);
}

#[test]
fn docker_build() {
    let program = env!("CARGO_BIN_EXE_cio");
    let workspaces = std::path::Path::new(file!())
        .parent()
        .unwrap()
        .join("workspaces");

    let success = Command::new(program)
        .arg("execute")
        .current_dir(workspaces.join("docker_build"))
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success();

    assert!(success);
}
