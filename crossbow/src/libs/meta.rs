//! Module containing definitions of test meta data
//!

use chrono::{DateTime, Duration, Utc};

/// new type for an S3 URI
pub struct S3Path(String);

/// Different states for a test run
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

/// Represents relevant data for a test run
pub struct RunResult {
    /// The status of the test
    pub status: RunStatus,
    /// Name of the test
    pub name: String,
    /// When the test was executed
    pub start_time: DateTime<Utc>,
    /// When the test completed
    pub end_time: DateTime<Utc>,
    /// Time test took to run
    pub duration: Duration,
    /// Message of test failure
    pub failure: Option<String>,
    /// Where test result is stored
    pub log: S3Path,
}
