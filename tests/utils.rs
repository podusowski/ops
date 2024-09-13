use std::{io::Write, path::PathBuf};

pub struct Workspace(pub PathBuf);

impl Workspace {
    pub fn new(ops_yaml: &str) -> Self {
        let dir: PathBuf =
            PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(uuid::Uuid::new_v4().to_string());
        std::fs::create_dir(&dir).unwrap();
        std::fs::File::create(dir.join("Ops.yaml"))
            .unwrap()
            .write_all(ops_yaml.as_bytes())
            .unwrap();
        Self(dir)
    }

    pub fn with_dockerfile(self, dockerfile: &str) -> Self {
        std::fs::File::create(self.0.join("Dockerfile"))
            .unwrap()
            .write_all(dockerfile.as_bytes())
            .unwrap();
        self
    }
}

impl Drop for Workspace {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).expect("could not remove workspace");
    }
}

pub static PROGRAM: &'static str = env!("CARGO_BIN_EXE_ops");
