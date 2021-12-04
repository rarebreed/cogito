//! Module containing definitions of test meta data
//! 

use chrono::{DateTime, Utc, Duration};

pub struct S3Path(String);

pub enum RunStatus {
    Pass,    // Test failed
    Fail,    // Test did not pass assertion(s)
    Skipped, // Test was not run
    Error    // test failed, but not due to assertion
}

pub struct RunResult {
    pub status: RunStatus,
    pub name: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub duration: Duration,
    pub failure: Option<String>,
    pub log: S3Path
}