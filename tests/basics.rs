pub mod utils;

use std::{os::unix::fs::MetadataExt, process::Command};

use assert_cmd::assert::OutputAssertExt;
use predicates::str::contains;
use utils::{Workspace, PROGRAM};

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
        .stdout(contains("hello world"))
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
