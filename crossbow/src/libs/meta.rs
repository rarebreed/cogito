//! Module containing definitions of test meta data

use chrono::{DateTime, Duration, Utc};
use serde::{Serialize, Deserialize};

/// Different states for a test run
#[derive(Serialize, Deserialize, Debug)]
pub enum RunStatus {
    /// Test passed all assertions
    Pass,
    /// Test failed one or more assertions
    Fail,    
    /// Test was not executed
    Skipped,
    /// test failed, but not due to assertion
    Error,
}

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


/// Represents relevant data for a test run
#[derive(Serialize, Deserialize, Debug)]
pub struct RunResult {
    /// The status of the test
    pub status: RunStatus,
    /// Name of the test
    pub name: String,
    /// When the test was executed
    pub start_time: DateTime<Utc>,
    /// When the test completed
    pub end_time: DateTime<Utc>,
    /// Message of test failure
    pub failure: Option<String>,
    /// Path for log of test
    pub log: LogPath,
}

impl RunResult {
    /// Returns the duration the test ran
    pub fn duration(&self) -> Duration {
        self.end_time.signed_duration_since(self.start_time)
    }
}
