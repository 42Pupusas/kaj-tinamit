//! Commit read endpoints.

use gitlab_model::{CommitDiff, CommitStatus, CommitWithDiffs, GitlabCommit};
use json_bourne::FromJson;

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

/// The subset of GitLab's `repository/compare` response we care about: the
/// list of file diffs between two refs.
#[derive(Debug, FromJson)]
#[bourne(deny_unknown_fields = false)]
struct CompareResult {
    #[bourne(default)]
    diffs: Vec<CommitDiff>,
}

/// Commit read endpoints.
pub trait CommitEndpoints {
    /// Get a single commit by project ID and commit SHA.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn commit(&self, project_id: i64, sha: &str) -> Result<CommitWithDiffs, Error>;

    /// Get the diffs for a commit.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn commit_diffs(&self, project_id: i64, sha: &str) -> Result<Vec<CommitDiff>, Error>;

    /// List commits for a project, optionally from a specific branch/tag.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn commits(&self, project_id: i64, ref_name: Option<&str>) -> Result<Vec<GitlabCommit>, Error>;

    /// Compare two refs and get the resulting file diffs.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn compare_commits(
        &self,
        project_id: i64,
        from: &str,
        to: &str,
    ) -> Result<Vec<CommitDiff>, Error>;

    /// List the CI/external statuses reported against a commit (the per-commit
    /// pipeline/check rollup).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn commit_statuses(&self, project_id: i64, sha: &str) -> Result<Vec<CommitStatus>, Error>;
}

impl CommitEndpoints for GitlabClient {
    fn commit(&self, project_id: i64, sha: &str) -> Result<CommitWithDiffs, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/repository/commits/{sha}"
        ))
    }

    fn commit_diffs(&self, project_id: i64, sha: &str) -> Result<Vec<CommitDiff>, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/repository/commits/{sha}/diff"
        ))
    }

    fn commits(&self, project_id: i64, ref_name: Option<&str>) -> Result<Vec<GitlabCommit>, Error> {
        let mut url = format!("api/v4/projects/{project_id}/repository/commits");
        if let Some(ref_name) = ref_name {
            url.push_str(&format!("?ref_name={}", ref_name.percent_encode()));
        }
        self.get_paginated(&url)
    }

    fn compare_commits(
        &self,
        project_id: i64,
        from: &str,
        to: &str,
    ) -> Result<Vec<CommitDiff>, Error> {
        let result: CompareResult = self.get(&format!(
            "api/v4/projects/{project_id}/repository/compare?from={}&to={}",
            from.percent_encode(),
            to.percent_encode()
        ))?;
        Ok(result.diffs)
    }

    fn commit_statuses(&self, project_id: i64, sha: &str) -> Result<Vec<CommitStatus>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/repository/commits/{sha}/statuses"
        ))
    }
}
