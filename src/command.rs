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
