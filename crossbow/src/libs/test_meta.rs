//! Data types representing tests, alarms and observations
//!

use crate::libs::meta::RunResult;

/// A TestSuite is a collection of TestRuns
pub struct TestSuiteResult {}

/// Data for an execution of a collection of tests
pub struct TestRun {
    /// The result overall of all the tests that ran
    pub run: RunResult,
}

/// A collection of tests
pub struct TestGroup {}
