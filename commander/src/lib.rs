//! This module will handle the abstraction of running commands, either locally or remotely on a machine.

#[cfg(feature = "async")]
pub mod async_cmd;
#[cfg(feature = "sync")]
pub mod sync_cmd;
