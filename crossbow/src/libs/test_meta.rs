//! Data types representing tests, alarms and observations
//!

use crate::libs::meta::RunResult;

/// A TestSuite is a collection of TestRuns
pub struct TestSuiteResult {}

/// Execution of a test
pub struct TestRun {
    /// The RunResult
    pub run: RunResult,
}

/// A grouping of tests
pub struct TestGroup {}
