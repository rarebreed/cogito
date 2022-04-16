//! Module containing definitions of test meta data
//!

use chrono::{DateTime, Duration, Utc};
use serde::{Serialize, Deserialize};

/// Path for where logs are stored
#[derive(Serialize, Deserialize, Debug)]
pub enum LogPath {
    /// For logs stored locally on a system
    Local(String),
    /// For logs stored in a cloud provider (eg, s3)
    Cloud {
        /// Provider type (eg, s3, gcp)
        /// FIXME: make this an enum
        provider: String,
        /// The full path to the log
        uri: String
    }
}

/// Status of an execution (ie, a TestRun)
#[derive(Serialize, Deserialize, Debug)]
pub enum RunStatus {
    /// Passing execution (ie no asserts failed and no panics)
    Pass,
    /// Test did not pass assertion(s)
    Fail,
    /// Test was not run
    Skipped,
    /// test failed, but not due to assertion
    Error,
}

/// Information about the result of an execution of some kind (eg a TestRun)
pub struct RunResult {
    /// Status of the run
    pub status: RunStatus,
    /// Name of the run (eg, fully qualified name of a test)
    pub name: String,
    /// Start of the execution
    pub start_time: DateTime<Utc>,
    /// End of the execution
    pub end_time: DateTime<Utc>,
    /// Time between start and end
    pub duration: Duration,
    /// If the run did not Pass, what caused it
    pub failure: Option<String>,
    /// Path to logs of the execution
    pub log: LogPath,
}
