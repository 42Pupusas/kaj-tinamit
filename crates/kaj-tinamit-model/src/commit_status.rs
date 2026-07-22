//! Wire shapes for the **Commit statuses** API category — the per-commit
//! CI/external-check rollup (`GET /projects/:id/repository/commits/:sha/statuses`).

use json_bourne::{FromJson, ToJson};

use crate::Id;
use crate::ci::{CiStatus, CiUser};

/// A proptest strategy yielding `Option<f64>` restricted to finite values.
///
/// Coverage is always a finite percentage; excluding NaN/inf keeps the JSON
/// round-trip well-defined (those have no JSON representation).
#[cfg(feature = "proptest")]
fn finite_opt_f64() -> impl proptest::strategy::Strategy<Value = Option<f64>> {
    proptest::option::of(-1e12f64..1e12f64)
}

/// A single commit status entry: one CI job or external check reported
/// against a commit SHA.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct CommitStatus {
    pub id: Id,
    pub sha: Option<String>,
    #[bourne(rename = "ref")]
    pub ref_name: Option<String>,
    pub status: CiStatus,
    pub name: Option<String>,
    pub target_url: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    #[bourne(default)]
    pub allow_failure: bool,
    #[cfg_attr(feature = "proptest", proptest(strategy = "finite_opt_f64()"))]
    pub coverage: Option<f64>,
    pub pipeline_id: Option<Id>,
    pub author: Option<CiUser>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_commit_status() {
        let json = r#"{
            "id": 91,
            "sha": "18f3e63d",
            "ref": "main",
            "status": "success",
            "name": "bundler:audit",
            "target_url": "https://example.com/jobs/91",
            "allow_failure": true,
            "coverage": 98.29
        }"#;
        let s: CommitStatus = parse_str(json).unwrap();
        assert_eq!(s.id, 91);
        assert_eq!(s.status, CiStatus::Success);
        assert!(s.allow_failure);
    }
}
