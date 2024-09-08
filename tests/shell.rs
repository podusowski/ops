pub mod utils;

use std::process::Command;

use assert_cmd::assert::OutputAssertExt;
use predicates::str::contains;
use utils::{Workspace, PROGRAM};

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
