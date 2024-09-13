use std::{
    fs::{create_dir, remove_dir_all, File},
    io::Write,
    path::PathBuf,
};

pub struct Workspace(pub PathBuf);

impl Workspace {
    pub fn new(ops_yaml: &str) -> Self {
        let dir: PathBuf =
            PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(uuid::Uuid::new_v4().to_string());
        create_dir(&dir).unwrap();
        Self(dir).with_ops_yaml(ops_yaml)
    }

    pub fn with_ops_yaml(self, ops_yaml: &str) -> Self {
        File::create(self.0.join("Ops.yaml"))
            .unwrap()
            .write_all(ops_yaml.as_bytes())
            .unwrap();
        self
    }

    pub fn with_dockerfile(self, dockerfile: &str) -> Self {
        File::create(self.0.join("Dockerfile"))
            .unwrap()
            .write_all(dockerfile.as_bytes())
            .unwrap();
        self
    }
}

impl Drop for Workspace {
    fn drop(&mut self) {
        remove_dir_all(&self.0).expect("could not remove workspace");
    }
}

pub static PROGRAM: &'static str = env!("CARGO_BIN_EXE_ops");
