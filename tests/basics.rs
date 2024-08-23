use std::process::Command;

#[test]
fn hello_world() {
    let program = env!("CARGO_BIN_EXE_cio");
    let workspaces = std::path::Path::new(file!())
        .parent()
        .unwrap()
        .join("workspaces");

    let success = Command::new(program)
        .arg("execute")
        .current_dir(workspaces.join("hello_world"))
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
