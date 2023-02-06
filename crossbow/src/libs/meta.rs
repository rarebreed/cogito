//! Definitions of test results and metadata

use std::collections::HashMap;

use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};

/// Test Case data
#[derive(Serialize, Deserialize, Debug)]
pub struct TestCase {
    /// The full unambiguous name of the test method
    pub qualified_name: String,
    /// Any parent group this test ran under
    pub parent: Option<String>,
    /// Metadata for the test
    pub metadata: Option<TestMetaData>,
    /// What ran the test
    pub executor: Executor,
}

/// Information about the entity that executes a test
#[derive(Serialize, Deserialize, Debug)]
pub struct Executor {
    /// A name for the executor, like a jenkins job name
    pub name: String,
    /// Eg Jenkins, Travis, Airflow, local, etc
    pub executor_type: String,
    /// An identifier like a jenkins build number
    pub id: String,
}

/// Data about the TestCase, such as dependencies
#[derive(Serialize, Deserialize, Debug)]
pub struct TestMetaData {
    /// system dependencies like microservices, etl pipelines
    pub system_deps: HashMap<String, String>,
    /// local dependencies such as utilities or files
    pub local_deps: HashMap<String, String>,
    /// Things which must exist before test runs, such as state value
    pub preconditions: HashMap<String, String>,
    /// Things which must be true after test runs
    pub postconditions: HashMap<String, String>,
}

/// SystemDependencies are external infrastructure dependencies that your project relies on.
///
/// For example, microservices, storage, other data jobs
pub struct SystemDependencies {}

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
        uri: String,
    },
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
    /// Path for log of test
    pub log: LogPath,
    /// Error details
    pub error_details: Option<String>,
    /// stack trace
    pub stack_trace: Option<Vec<String>>,
}

impl RunResult {
    /// Returns the duration the test ran
    pub fn duration(&self) -> chrono::Duration {
        self.end_time.signed_duration_since(self.start_time)
    }
}
