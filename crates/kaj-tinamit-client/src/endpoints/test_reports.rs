//! Pipeline unit test report endpoints.

use kaj_tinamit::{TestReport, TestReportSummary};

use crate::client::GitlabClient;
use crate::error::Error;

/// Test reports gathered from a pipeline's `artifacts:reports:junit`.
pub trait TestReportEndpoints {
    /// The full report: every suite and every case, with stack traces of
    /// the failures.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn pipeline_test_report(&self, project_id: i64, pipeline_id: i64) -> Result<TestReport, Error>;

    /// Counts per suite, without the individual cases. Much smaller than
    /// the full report on large test suites.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn pipeline_test_report_summary(
        &self,
        project_id: i64,
        pipeline_id: i64,
    ) -> Result<TestReportSummary, Error>;
}

impl TestReportEndpoints for GitlabClient {
    fn pipeline_test_report(&self, project_id: i64, pipeline_id: i64) -> Result<TestReport, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/pipelines/{pipeline_id}/test_report"
        ))
    }

    fn pipeline_test_report_summary(
        &self,
        project_id: i64,
        pipeline_id: i64,
    ) -> Result<TestReportSummary, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/pipelines/{pipeline_id}/test_report_summary"
        ))
    }
}
