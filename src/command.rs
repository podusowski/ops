/// Extensions for `std::process`.
use std::{io::Write, process::Command};

pub trait CommandEx {
    /// Print details about this command to `debug!`.
    fn debug(&mut self) -> &mut Self;
}

impl CommandEx for Command {
    fn debug(&mut self) -> &mut Self {
        log::debug!("{:?}", self.get_args());
        self
    }
}

/// Placeholder until `exit_ok` is stabilized.
/// https://github.com/rust-lang/rust/issues/84908
pub trait ExitStatusEx {
    fn exit_ok_(&self) -> anyhow::Result<()>;
}

impl ExitStatusEx for std::process::ExitStatus {
    fn exit_ok_(&self) -> anyhow::Result<()> {
        if self.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("command failed: {:?}", self))
        }
    }
}

pub trait ChildEx {
    /// Write data to the process' stdin.
    fn write_to_stdin(&mut self, data: &[u8]) -> anyhow::Result<()>;
}

impl ChildEx for std::process::Child {
    fn write_to_stdin(&mut self, data: &[u8]) -> anyhow::Result<()> {
        self.stdin
            .take()
            .ok_or(anyhow::anyhow!("cannot access process' stdin handle"))?
            .write_all(data)?;
        Ok(())
    }
}
