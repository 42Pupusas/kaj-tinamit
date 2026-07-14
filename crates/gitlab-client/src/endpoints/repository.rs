//! Repository read endpoints: the **Repository**, **Branches**, **Tags**,
//! and **Repository files** API categories.
//!
//! Covers every read-only (GET) primitive in those categories. Write
//! operations (create branch/tag/file, delete, changelog commit) are
//! intentionally omitted.

use gitlab_model::{
    BlameRange, Blob, Branch, Changelog, Contributor, RefCommit, RepositoryFile, Tag, TreeEntry,
};
use json_bourne::FromJson;

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

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
    fn file_raw(&self, project_id: i64, file_path: &str, ref_name: &str)
    -> Result<String, Error>;

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
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/repository/branches"
        ))
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

    fn file_raw(
        &self,
        project_id: i64,
        file_path: &str,
        ref_name: &str,
    ) -> Result<String, Error> {
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
}
