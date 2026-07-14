//! Commit endpoints.
//!
//! Reads fetch commits, diffs, comparisons, and statuses; writes create a
//! commit from a set of file actions, cherry-pick, and revert.

use gitlab_model::{CommitDiff, CommitStatus, CommitWithDiffs, GitlabCommit};
use json_bourne::{FromJson, ToJson};

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

/// One file action within a [`CreateCommit`] payload.
///
/// `action` is `create`, `update`, `delete`, `move`, or `chmod`. Use the
/// constructors for the common cases.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct CommitAction {
    pub action: String,
    pub file_path: String,
    #[bourne(skip_if_none)]
    pub previous_path: Option<String>,
    #[bourne(skip_if_none)]
    pub content: Option<String>,
    /// `text` (default) or `base64`.
    #[bourne(skip_if_none)]
    pub encoding: Option<String>,
    #[bourne(skip_if_none)]
    pub execute_filemode: Option<bool>,
}

impl CommitAction {
    /// Create a new file with `content`.
    #[must_use]
    pub fn create(file_path: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            action: "create".to_string(),
            file_path: file_path.into(),
            previous_path: None,
            content: Some(content.into()),
            encoding: None,
            execute_filemode: None,
        }
    }

    /// Replace an existing file's `content`.
    #[must_use]
    pub fn update(file_path: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            action: "update".to_string(),
            file_path: file_path.into(),
            previous_path: None,
            content: Some(content.into()),
            encoding: None,
            execute_filemode: None,
        }
    }

    /// Delete a file.
    #[must_use]
    pub fn delete(file_path: impl Into<String>) -> Self {
        Self {
            action: "delete".to_string(),
            file_path: file_path.into(),
            previous_path: None,
            content: None,
            encoding: None,
            execute_filemode: None,
        }
    }

    /// Move a file from `previous_path` to `file_path`, optionally rewriting
    /// its `content`.
    #[must_use]
    pub fn move_file(previous_path: impl Into<String>, file_path: impl Into<String>) -> Self {
        Self {
            action: "move".to_string(),
            file_path: file_path.into(),
            previous_path: Some(previous_path.into()),
            content: None,
            encoding: None,
            execute_filemode: None,
        }
    }
}

/// Body for creating a commit from a set of file actions
/// (`POST /projects/:id/repository/commits`).
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct CreateCommit {
    pub branch: String,
    pub commit_message: String,
    pub actions: Vec<CommitAction>,
    #[bourne(skip_if_none)]
    pub start_branch: Option<String>,
    #[bourne(skip_if_none)]
    pub author_email: Option<String>,
    #[bourne(skip_if_none)]
    pub author_name: Option<String>,
}

impl CreateCommit {
    #[must_use]
    pub fn new(
        branch: impl Into<String>,
        commit_message: impl Into<String>,
        actions: Vec<CommitAction>,
    ) -> Self {
        Self {
            branch: branch.into(),
            commit_message: commit_message.into(),
            actions,
            start_branch: None,
            author_email: None,
            author_name: None,
        }
    }

    /// Create `branch` from `start_branch` if it doesn't yet exist.
    #[must_use]
    pub fn with_start_branch(mut self, start_branch: impl Into<String>) -> Self {
        self.start_branch = Some(start_branch.into());
        self
    }

    #[must_use]
    pub fn with_author(mut self, name: impl Into<String>, email: impl Into<String>) -> Self {
        self.author_name = Some(name.into());
        self.author_email = Some(email.into());
        self
    }
}

/// Body for cherry-picking or reverting a commit onto a branch.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
struct BranchBody {
    branch: String,
}

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

    // --- Writes ---

    /// Create a commit on `branch` from a set of file actions. Returns the
    /// created commit.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_commit(
        &self,
        project_id: i64,
        commit: &CreateCommit,
    ) -> Result<CommitWithDiffs, Error>;

    /// Cherry-pick a commit (`sha`) onto `branch`. Returns the new commit.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn cherry_pick_commit(
        &self,
        project_id: i64,
        sha: &str,
        branch: &str,
    ) -> Result<CommitWithDiffs, Error>;

    /// Revert a commit (`sha`) on `branch`. Returns the new commit.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn revert_commit(
        &self,
        project_id: i64,
        sha: &str,
        branch: &str,
    ) -> Result<CommitWithDiffs, Error>;
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

    fn create_commit(
        &self,
        project_id: i64,
        commit: &CreateCommit,
    ) -> Result<CommitWithDiffs, Error> {
        self.post(
            &format!("api/v4/projects/{project_id}/repository/commits"),
            commit,
        )
    }

    fn cherry_pick_commit(
        &self,
        project_id: i64,
        sha: &str,
        branch: &str,
    ) -> Result<CommitWithDiffs, Error> {
        self.post(
            &format!("api/v4/projects/{project_id}/repository/commits/{sha}/cherry_pick"),
            &BranchBody {
                branch: branch.to_string(),
            },
        )
    }

    fn revert_commit(
        &self,
        project_id: i64,
        sha: &str,
        branch: &str,
    ) -> Result<CommitWithDiffs, Error> {
        self.post(
            &format!("api/v4/projects/{project_id}/repository/commits/{sha}/revert"),
            &BranchBody {
                branch: branch.to_string(),
            },
        )
    }
}
