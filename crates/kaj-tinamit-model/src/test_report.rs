//! Wire shapes for a pipeline's unit test reports
//! (`GET /projects/:id/pipelines/:pipeline_id/test_report` and
//! `.../test_report_summary`).

use json_bourne::{FromJson, ToJson};

/// One test case inside a [`TestSuite`].
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct TestCase {
    /// `success`, `failed`, `skipped`, or `error`.
    pub status: Option<String>,
    pub name: Option<String>,
    pub classname: Option<String>,
    pub file: Option<String>,
    #[cfg_attr(
        feature = "proptest",
        proptest(strategy = "proptest::option::of(-1e12f64..1e12f64)")
    )]
    pub execution_time: Option<f64>,
    pub system_output: Option<String>,
    pub stack_trace: Option<String>,
}

/// A test suite with its cases, as reported by one or more jobs.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct TestSuite {
    pub name: Option<String>,
    #[cfg_attr(
        feature = "proptest",
        proptest(strategy = "proptest::option::of(-1e12f64..1e12f64)")
    )]
    pub total_time: Option<f64>,
    #[bourne(default)]
    pub total_count: i64,
    #[bourne(default)]
    pub success_count: i64,
    #[bourne(default)]
    pub failed_count: i64,
    #[bourne(default)]
    pub skipped_count: i64,
    #[bourne(default)]
    pub error_count: i64,
    pub suite_error: Option<String>,
    #[bourne(default)]
    pub build_ids: Vec<i64>,
    #[bourne(default)]
    #[cfg_attr(
        feature = "proptest",
        proptest(
            strategy = "proptest::collection::vec(proptest::prelude::any::<TestCase>(), 0..4)"
        )
    )]
    pub test_cases: Vec<TestCase>,
}

impl TestSuite {
    /// The cases that failed or errored.
    pub fn broken_cases(&self) -> impl Iterator<Item = &TestCase> {
        self.test_cases
            .iter()
            .filter(|c| matches!(c.status.as_deref(), Some("failed" | "error")))
    }
}

/// The full test report of a pipeline.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct TestReport {
    #[cfg_attr(
        feature = "proptest",
        proptest(strategy = "proptest::option::of(-1e12f64..1e12f64)")
    )]
    pub total_time: Option<f64>,
    #[bourne(default)]
    pub total_count: i64,
    #[bourne(default)]
    pub success_count: i64,
    #[bourne(default)]
    pub failed_count: i64,
    #[bourne(default)]
    pub skipped_count: i64,
    #[bourne(default)]
    pub error_count: i64,
    #[bourne(default)]
    #[cfg_attr(
        feature = "proptest",
        proptest(
            strategy = "proptest::collection::vec(proptest::prelude::any::<TestSuite>(), 0..4)"
        )
    )]
    pub test_suites: Vec<TestSuite>,
}

/// The `total` block of a [`TestReportSummary`].
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct TestTotals {
    #[cfg_attr(
        feature = "proptest",
        proptest(strategy = "proptest::option::of(-1e12f64..1e12f64)")
    )]
    pub time: Option<f64>,
    #[bourne(default)]
    pub count: i64,
    #[bourne(default)]
    pub success: i64,
    #[bourne(default)]
    pub failed: i64,
    #[bourne(default)]
    pub skipped: i64,
    #[bourne(default)]
    pub error: i64,
    pub suite_error: Option<String>,
}

/// Per-suite counts without the individual cases.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct TestReportSummary {
    pub total: TestTotals,
    #[bourne(default)]
    #[cfg_attr(
        feature = "proptest",
        proptest(
            strategy = "proptest::collection::vec(proptest::prelude::any::<TestSuite>(), 0..4)"
        )
    )]
    pub test_suites: Vec<TestSuite>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_report_and_find_failures() {
        let json = r#"{
            "total_time": 5,
            "total_count": 2,
            "success_count": 1,
            "failed_count": 1,
            "skipped_count": 0,
            "error_count": 0,
            "test_suites": [{
                "name": "Secure",
                "total_time": 5,
                "total_count": 2,
                "success_count": 1,
                "failed_count": 1,
                "skipped_count": 0,
                "error_count": 0,
                "suite_error": null,
                "test_cases": [
                    {"status": "success", "name": "ok", "classname": "a", "execution_time": 1.5},
                    {"status": "failed", "name": "boom", "classname": "a", "execution_time": 3.5,
                     "stack_trace": "assertion failed"}
                ]
            }]
        }"#;
        let r: TestReport = parse_str(json).unwrap();
        assert_eq!(r.failed_count, 1);
        let broken: Vec<_> = r.test_suites[0].broken_cases().collect();
        assert_eq!(broken.len(), 1);
        assert_eq!(broken[0].name.as_deref(), Some("boom"));
    }

    #[test]
    fn parse_summary() {
        let json = r#"{
            "total": {"time": 1904, "count": 3363, "success": 3351, "failed": 0,
                      "skipped": 12, "error": 0, "suite_error": null},
            "test_suites": [{"name": "test", "total_time": 1904, "total_count": 3363,
                             "success_count": 3351, "failed_count": 0, "skipped_count": 12,
                             "error_count": 0, "build_ids": [66004], "suite_error": null}]
        }"#;
        let s: TestReportSummary = parse_str(json).unwrap();
        assert_eq!(s.total.count, 3363);
        assert_eq!(s.test_suites[0].build_ids, vec![66004]);
    }
}
