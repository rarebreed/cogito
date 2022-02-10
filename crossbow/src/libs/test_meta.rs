//! Data types representing tests, alarms and observations
//!

use crate::libs::meta::RunResult;

/// A TestSuite is a collection of TestRuns
pub struct TestSuiteResult {}

pub struct TestRun {
    pub run: RunResult,
}

pub struct TestGroup {}
