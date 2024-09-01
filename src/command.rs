/// Extensions for `std::process`.
use std::process::Command;

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
    fn exit_ok(&self) -> anyhow::Result<()>;
}

impl ExitStatusEx for std::process::ExitStatus {
    fn exit_ok(&self) -> anyhow::Result<()> {
        if self.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("command failed: {:?}", self))
        }
    }
}
