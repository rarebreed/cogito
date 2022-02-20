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


#[derive(Serialize, Deserialize, Debug)]
pub enum RunStatus {
    Pass,
    Fail,    // Test did not pass assertion(s)
    Skipped, // Test was not run
    Error,   // test failed, but not due to assertion
}

pub struct RunResult {
    pub status: RunStatus,
    pub name: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub duration: Duration,
    pub failure: Option<String>,
    pub log: LogPath,
}
