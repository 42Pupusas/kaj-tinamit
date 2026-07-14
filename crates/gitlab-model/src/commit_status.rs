//! Wire shapes for the **Commit statuses** API category — the per-commit
//! CI/external-check rollup (`GET /projects/:id/repository/commits/:sha/statuses`).

use json_bourne::{FromJson, ToJson};

use crate::ci::{CiStatus, CiUser};

/// A single commit status entry: one CI job or external check reported
/// against a commit SHA.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct CommitStatus {
    pub id: i64,
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
    pub coverage: Option<f64>,
    pub pipeline_id: Option<i64>,
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
