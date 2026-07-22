//! Repository endpoints: the **Repository**, **Branches**, **Tags**, and
//! **Repository files** API categories.
//!
//! Reads cover trees/blobs/branches/tags/files/blame/compare; writes create
//! and delete branches and tags, and create/update/delete repository files.

use json_bourne::{FromJson, ToJson};
use kaj_tinamit::{
    BlameRange, Blob, Branch, Changelog, Contributor, FileMutationResult, RefCommit,
    RepositoryFile, Tag, TreeEntry,
};

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

/// Body for creating or updating a repository file. `branch`,
/// `content`, and `commit_message` are required; the rest are optional.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct CommitFile {
    pub branch: String,
    pub content: String,
    pub commit_message: String,
    /// Set on update to move the file from another branch's tip.
    #[bourne(skip_if_none)]
    pub start_branch: Option<String>,
    /// `text` (default) or `base64` — how `content` is encoded.
    #[bourne(skip_if_none)]
    pub encoding: Option<String>,
    #[bourne(skip_if_none)]
    pub author_email: Option<String>,
    #[bourne(skip_if_none)]
    pub author_name: Option<String>,
    /// Required by GitLab on update to guard against lost updates.
    #[bourne(skip_if_none)]
    pub last_commit_id: Option<String>,
}

impl CommitFile {
    #[must_use]
    pub fn new(
        branch: impl Into<String>,
        content: impl Into<String>,
        commit_message: impl Into<String>,
    ) -> Self {
        Self {
            branch: branch.into(),
            content: content.into(),
            commit_message: commit_message.into(),
            start_branch: None,
            encoding: None,
            author_email: None,
            author_name: None,
            last_commit_id: None,
        }
    }

    /// Send `content` as base64 (for binary files).
    #[must_use]
    pub fn base64(mut self) -> Self {
        self.encoding = Some("base64".to_string());
        self
    }

    #[must_use]
    pub fn with_author(mut self, name: impl Into<String>, email: impl Into<String>) -> Self {
        self.author_name = Some(name.into());
        self.author_email = Some(email.into());
        self
    }

    /// Set the expected last commit ID (optimistic concurrency on update).
    #[must_use]
    pub fn with_last_commit_id(mut self, last_commit_id: impl Into<String>) -> Self {
        self.last_commit_id = Some(last_commit_id.into());
        self
    }
}

/// Body for deleting a repository file.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
struct DeleteFileBody {
    branch: String,
    commit_message: String,
    #[bourne(skip_if_none)]
    author_email: Option<String>,
    #[bourne(skip_if_none)]
    author_name: Option<String>,
}

/// The subset of `repository/compare` we expose alongside the commit-diff
/// view in [`CommitEndpoints`](crate::CommitEndpoints): the full commit list.
#[derive(Debug, FromJson)]
#[bourne(deny_unknown_fields = false)]
struct CompareCommits {
    #[bourne(default)]
    commits: Vec<RefCommit>,
}

/// Read endpoints for repository contents, branches, tags, and files.
pub trait RepositoryEndpoints {
    /// List a repository tree (files and directories).
    ///
    /// `path` scopes to a subdirectory (`None` = repo root); `ref_name`
    /// selects a branch/tag/commit (`None` = default branch); when
    /// `recursive` is set, subdirectories are walked.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn repository_tree(
        &self,
        project_id: i64,
        path: Option<&str>,
        ref_name: Option<&str>,
        recursive: bool,
    ) -> Result<Vec<TreeEntry>, Error>;

    /// Retrieve a blob's metadata and Base64-encoded content by SHA.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn blob(&self, project_id: i64, sha: &str) -> Result<Blob, Error>;

    /// Retrieve raw blob content by SHA.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn blob_raw(&self, project_id: i64, sha: &str) -> Result<String, Error>;

    /// List repository contributors.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn contributors(&self, project_id: i64) -> Result<Vec<Contributor>, Error>;

    /// Get the merge base (common ancestor) commit of two or more refs.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn merge_base(&self, project_id: i64, refs: &[&str]) -> Result<RefCommit, Error>;

    /// List the commits between two refs (the `commits` half of compare).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn compare_ref_commits(
        &self,
        project_id: i64,
        from: &str,
        to: &str,
    ) -> Result<Vec<RefCommit>, Error>;

    /// Generate changelog data for a version (without committing it).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn changelog(&self, project_id: i64, version: &str) -> Result<Changelog, Error>;

    // ---- Branches ----

    /// List all repository branches.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn branches(&self, project_id: i64) -> Result<Vec<Branch>, Error>;

    /// Search branches whose name contains `search`.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn search_branches(&self, project_id: i64, search: &str) -> Result<Vec<Branch>, Error>;

    /// Retrieve a single branch by name.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn branch(&self, project_id: i64, branch: &str) -> Result<Branch, Error>;

    // ---- Tags ----

    /// List all repository tags.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn tags(&self, project_id: i64) -> Result<Vec<Tag>, Error>;

    /// Retrieve a single tag by name.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn tag(&self, project_id: i64, tag_name: &str) -> Result<Tag, Error>;

    // ---- Repository files ----

    /// Retrieve a file's metadata and Base64-encoded content at `ref_name`.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn file(
        &self,
        project_id: i64,
        file_path: &str,
        ref_name: &str,
    ) -> Result<RepositoryFile, Error>;

    /// Retrieve a file's raw content at `ref_name`.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn file_raw(&self, project_id: i64, file_path: &str, ref_name: &str) -> Result<String, Error>;

    /// Retrieve blame ranges for a file at `ref_name`.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn file_blame(
        &self,
        project_id: i64,
        file_path: &str,
        ref_name: &str,
    ) -> Result<Vec<BlameRange>, Error>;

    // ---- Writes: branches ----

    /// Create a branch `branch` from `ref_name` (branch/tag/commit). Returns
    /// the new branch.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_branch(&self, project_id: i64, branch: &str, ref_name: &str)
    -> Result<Branch, Error>;

    /// Delete a branch by name.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_branch(&self, project_id: i64, branch: &str) -> Result<(), Error>;

    /// Delete all merged branches (bulk cleanup). GitLab performs this
    /// asynchronously and returns `202 Accepted`.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_merged_branches(&self, project_id: i64) -> Result<(), Error>;

    // ---- Writes: tags ----

    /// Create a tag `tag_name` at `ref_name`, with an optional annotation
    /// message. Returns the new tag.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_tag(
        &self,
        project_id: i64,
        tag_name: &str,
        ref_name: &str,
        message: Option<&str>,
    ) -> Result<Tag, Error>;

    /// Delete a tag by name.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_tag(&self, project_id: i64, tag_name: &str) -> Result<(), Error>;

    // ---- Writes: repository files ----

    /// Create a new repository file. Returns the file path and branch.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_file(
        &self,
        project_id: i64,
        file_path: &str,
        commit: &CommitFile,
    ) -> Result<FileMutationResult, Error>;

    /// Update an existing repository file. Returns the file path and branch.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn update_file(
        &self,
        project_id: i64,
        file_path: &str,
        commit: &CommitFile,
    ) -> Result<FileMutationResult, Error>;

    /// Delete a repository file on `branch` with `commit_message`.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_file(
        &self,
        project_id: i64,
        file_path: &str,
        branch: &str,
        commit_message: &str,
    ) -> Result<(), Error>;
}

impl RepositoryEndpoints for GitlabClient {
    fn repository_tree(
        &self,
        project_id: i64,
        path: Option<&str>,
        ref_name: Option<&str>,
        recursive: bool,
    ) -> Result<Vec<TreeEntry>, Error> {
        let mut query = Vec::new();
        if let Some(p) = path {
            query.push(format!("path={}", p.percent_encode()));
        }
        if let Some(r) = ref_name {
            query.push(format!("ref={}", r.percent_encode()));
        }
        if recursive {
            query.push("recursive=true".to_string());
        }
        let mut url = format!("api/v4/projects/{project_id}/repository/tree");
        if !query.is_empty() {
            url.push('?');
            url.push_str(&query.join("&"));
        }
        self.get_paginated(&url)
    }

    fn blob(&self, project_id: i64, sha: &str) -> Result<Blob, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/repository/blobs/{sha}"
        ))
    }

    fn blob_raw(&self, project_id: i64, sha: &str) -> Result<String, Error> {
        self.get_raw(&format!(
            "api/v4/projects/{project_id}/repository/blobs/{sha}/raw"
        ))
    }

    fn contributors(&self, project_id: i64) -> Result<Vec<Contributor>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/repository/contributors"
        ))
    }

    fn merge_base(&self, project_id: i64, refs: &[&str]) -> Result<RefCommit, Error> {
        let query = refs
            .iter()
            .map(|r| format!("refs[]={}", r.percent_encode()))
            .collect::<Vec<_>>()
            .join("&");
        self.get(&format!(
            "api/v4/projects/{project_id}/repository/merge_base?{query}"
        ))
    }

    fn compare_ref_commits(
        &self,
        project_id: i64,
        from: &str,
        to: &str,
    ) -> Result<Vec<RefCommit>, Error> {
        let result: CompareCommits = self.get(&format!(
            "api/v4/projects/{project_id}/repository/compare?from={}&to={}",
            from.percent_encode(),
            to.percent_encode()
        ))?;
        Ok(result.commits)
    }

    fn changelog(&self, project_id: i64, version: &str) -> Result<Changelog, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/repository/changelog?version={}",
            version.percent_encode()
        ))
    }

    fn branches(&self, project_id: i64) -> Result<Vec<Branch>, Error> {
        self.get_paginated(&format!("api/v4/projects/{project_id}/repository/branches"))
    }

    fn search_branches(&self, project_id: i64, search: &str) -> Result<Vec<Branch>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/repository/branches?search={}",
            search.percent_encode()
        ))
    }

    fn branch(&self, project_id: i64, branch: &str) -> Result<Branch, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/repository/branches/{}",
            branch.percent_encode()
        ))
    }

    fn tags(&self, project_id: i64) -> Result<Vec<Tag>, Error> {
        self.get_paginated(&format!("api/v4/projects/{project_id}/repository/tags"))
    }

    fn tag(&self, project_id: i64, tag_name: &str) -> Result<Tag, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/repository/tags/{}",
            tag_name.percent_encode()
        ))
    }

    fn file(
        &self,
        project_id: i64,
        file_path: &str,
        ref_name: &str,
    ) -> Result<RepositoryFile, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/repository/files/{}?ref={}",
            file_path.percent_encode(),
            ref_name.percent_encode()
        ))
    }

    fn file_raw(&self, project_id: i64, file_path: &str, ref_name: &str) -> Result<String, Error> {
        self.get_raw(&format!(
            "api/v4/projects/{project_id}/repository/files/{}/raw?ref={}",
            file_path.percent_encode(),
            ref_name.percent_encode()
        ))
    }

    fn file_blame(
        &self,
        project_id: i64,
        file_path: &str,
        ref_name: &str,
    ) -> Result<Vec<BlameRange>, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/repository/files/{}/blame?ref={}",
            file_path.percent_encode(),
            ref_name.percent_encode()
        ))
    }

    fn create_branch(
        &self,
        project_id: i64,
        branch: &str,
        ref_name: &str,
    ) -> Result<Branch, Error> {
        // Branch + ref go in the query string per GitLab's API.
        self.post_no_body(&format!(
            "api/v4/projects/{project_id}/repository/branches?branch={}&ref={}",
            branch.percent_encode(),
            ref_name.percent_encode()
        ))
    }

    fn delete_branch(&self, project_id: i64, branch: &str) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/projects/{project_id}/repository/branches/{}",
            branch.percent_encode()
        ))
    }

    fn delete_merged_branches(&self, project_id: i64) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/projects/{project_id}/repository/merged_branches"
        ))
    }

    fn create_tag(
        &self,
        project_id: i64,
        tag_name: &str,
        ref_name: &str,
        message: Option<&str>,
    ) -> Result<Tag, Error> {
        let mut url = format!(
            "api/v4/projects/{project_id}/repository/tags?tag_name={}&ref={}",
            tag_name.percent_encode(),
            ref_name.percent_encode()
        );
        if let Some(message) = message {
            url.push_str(&format!("&message={}", message.percent_encode()));
        }
        self.post_no_body(&url)
    }

    fn delete_tag(&self, project_id: i64, tag_name: &str) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/projects/{project_id}/repository/tags/{}",
            tag_name.percent_encode()
        ))
    }

    fn create_file(
        &self,
        project_id: i64,
        file_path: &str,
        commit: &CommitFile,
    ) -> Result<FileMutationResult, Error> {
        self.post(
            &format!(
                "api/v4/projects/{project_id}/repository/files/{}",
                file_path.percent_encode()
            ),
            commit,
        )
    }

    fn update_file(
        &self,
        project_id: i64,
        file_path: &str,
        commit: &CommitFile,
    ) -> Result<FileMutationResult, Error> {
        self.put(
            &format!(
                "api/v4/projects/{project_id}/repository/files/{}",
                file_path.percent_encode()
            ),
            commit,
        )
    }

    fn delete_file(
        &self,
        project_id: i64,
        file_path: &str,
        branch: &str,
        commit_message: &str,
    ) -> Result<(), Error> {
        self.delete_with_body(
            &format!(
                "api/v4/projects/{project_id}/repository/files/{}",
                file_path.percent_encode()
            ),
            &DeleteFileBody {
                branch: branch.to_string(),
                commit_message: commit_message.to_string(),
                author_email: None,
                author_name: None,
            },
        )
    }
}
