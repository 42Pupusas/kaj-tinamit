//! Commit-related wire shapes.

use json_bourne::{FromJson, ToJson};

/// A git commit from the GitLab API.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct GitlabCommit {
    pub id: String,
    pub short_id: String,
    pub title: String,
    pub message: String,
    pub author_name: String,
    pub author_email: String,
    pub authored_date: String,
    pub committer_name: String,
    pub committer_email: String,
    pub committed_date: String,
    pub created_at: String,
    #[bourne(default)]
    pub parent_ids: Vec<String>,
    pub web_url: String,
}

/// A single file diff from a commit or compare response.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct CommitDiff {
    pub old_path: String,
    pub new_path: String,
    pub a_mode: Option<String>,
    pub b_mode: Option<String>,
    pub new_file: bool,
    pub renamed_file: bool,
    pub deleted_file: bool,
    pub diff: String,
}

/// A commit including per-commit change statistics.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct CommitWithDiffs {
    pub id: String,
    pub short_id: String,
    pub title: String,
    pub message: String,
    pub author_name: String,
    pub author_email: String,
    pub authored_date: String,
    pub committer_name: String,
    pub committer_email: String,
    pub committed_date: String,
    #[bourne(default)]
    pub parent_ids: Vec<String>,
    pub web_url: String,
    #[bourne(default)]
    pub stats: CommitStats,
}

/// Commit statistics.
#[derive(Debug, FromJson, ToJson, Clone, Default)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct CommitStats {
    #[bourne(default)]
    pub additions: i32,
    #[bourne(default)]
    pub deletions: i32,
    #[bourne(default)]
    pub total: i32,
}
