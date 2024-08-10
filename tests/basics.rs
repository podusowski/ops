use std::process::Command;

#[test]
fn hello_world() {
    let program = env!("CARGO_BIN_EXE_cio");
    let workspaces = std::path::Path::new(file!())
        .parent()
        .unwrap()
        .join("workspaces");

    let success = Command::new(program)
        .current_dir(workspaces.join("hello_world"))
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success();

    assert!(success);
}
