//! Merge-request wire shapes, plus the notes/discussions/pipelines/
//! approvals objects that hang off them.

use json_bourne::{FromJson, ToJson};

/// Merge request state.
#[derive(Debug, Clone, PartialEq, Eq, FromJson, ToJson)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(rename_all = "lowercase")]
pub enum MergeRequestState {
    Opened,
    Closed,
    Locked,
    Merged,
}

impl AsRef<str> for MergeRequestState {
    fn as_ref(&self) -> &str {
        match self {
            Self::Opened => "opened",
            Self::Closed => "closed",
            Self::Locked => "locked",
            Self::Merged => "merged",
        }
    }
}

/// Merge status.
#[derive(Debug, Clone, PartialEq, Eq, FromJson, ToJson)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(rename_all = "snake_case")]
pub enum MergeStatus {
    CanBeMerged,
    CannotBeMerged,
    Unchecked,
    CannotBeMergedRecheck,
}

impl AsRef<str> for MergeStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::CanBeMerged => "can_be_merged",
            Self::CannotBeMerged => "cannot_be_merged",
            Self::Unchecked => "unchecked",
            Self::CannotBeMergedRecheck => "cannot_be_merged_recheck",
        }
    }
}

/// Merge request resource.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct MergeRequest {
    pub id: i64,
    pub iid: i64,
    pub title: String,
    pub description: Option<String>,
    pub state: MergeRequestState,
    pub created_at: String,
    pub updated_at: String,
    pub merged_at: Option<String>,
    pub closed_at: Option<String>,
    pub target_branch: String,
    pub source_branch: String,
    pub author: MergeRequestUser,
    pub assignee: Option<MergeRequestUser>,
    #[bourne(default)]
    pub assignees: Vec<MergeRequestUser>,
    #[bourne(default)]
    pub reviewers: Vec<MergeRequestUser>,
    #[bourne(default)]
    pub labels: Vec<String>,
    pub milestone: Option<MergeRequestMilestone>,
    #[bourne(default)]
    pub work_in_progress: bool,
    #[bourne(default)]
    pub draft: bool,
    pub merge_status: MergeStatus,
    pub sha: Option<String>,
    pub merge_commit_sha: Option<String>,
    pub web_url: String,
    #[bourne(default)]
    pub has_conflicts: bool,
    #[bourne(default)]
    pub blocking_discussions_resolved: bool,
}

#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct MergeRequestUser {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct MergeRequestMilestone {
    pub id: i64,
    pub iid: i64,
    pub title: String,
}

/// A discussion (thread of notes) on a merge request or issue.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct Discussion {
    pub id: String,
    pub individual_note: bool,
    pub notes: Vec<DiscussionNote>,
}

#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct DiscussionNote {
    pub id: i64,
    pub body: String,
    pub author: MergeRequestUser,
    pub created_at: String,
    pub updated_at: String,
    pub system: bool,
    pub noteable_type: String,
    pub resolvable: bool,
    pub resolved: Option<bool>,
}

/// Merge request changes/diffs.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct MergeRequestChanges {
    pub id: i64,
    pub iid: i64,
    pub changes: Vec<FileChange>,
    /// Number of changed files as reported by GitLab, e.g. `"12"` or,
    /// when the diff was truncated, `"100+"`.
    pub changes_count: String,
    /// GitLab sets this to `true` when the merge request is too large and
    /// the returned `changes` are a truncated subset of the real diff.
    #[bourne(default)]
    pub overflow: bool,
}

#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct FileChange {
    pub old_path: String,
    pub new_path: String,
    pub a_mode: Option<String>,
    pub b_mode: Option<String>,
    pub new_file: bool,
    pub renamed_file: bool,
    pub deleted_file: bool,
    pub diff: String,
}

/// Pipeline status.
#[derive(Debug, Clone, PartialEq, Eq, FromJson, ToJson)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(rename_all = "lowercase")]
pub enum PipelineStatus {
    Created,
    WaitingForResource,
    Preparing,
    Pending,
    Running,
    Success,
    Failed,
    Canceled,
    Skipped,
    Manual,
    Scheduled,
}

impl AsRef<str> for PipelineStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Created => "created",
            Self::WaitingForResource => "waiting_for_resource",
            Self::Preparing => "preparing",
            Self::Pending => "pending",
            Self::Running => "running",
            Self::Success => "success",
            Self::Failed => "failed",
            Self::Canceled => "canceled",
            Self::Skipped => "skipped",
            Self::Manual => "manual",
            Self::Scheduled => "scheduled",
        }
    }
}

/// Pipeline information.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct Pipeline {
    pub id: i64,
    pub iid: i64,
    pub status: PipelineStatus,
    pub ref_name: String,
    pub sha: String,
    pub web_url: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Merge request approvals.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct MergeRequestApprovals {
    pub approvals_required: i32,
    pub approvals_left: i32,
    pub approved_by: Vec<Approval>,
}

#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct Approval {
    pub user: MergeRequestUser,
}

/// Noteable type for notes/comments.
#[derive(Debug, Clone, PartialEq, Eq, FromJson, ToJson)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
pub enum NoteableType {
    Issue,
    MergeRequest,
    Snippet,
    Commit,
}

impl AsRef<str> for NoteableType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Issue => "Issue",
            Self::MergeRequest => "MergeRequest",
            Self::Snippet => "Snippet",
            Self::Commit => "Commit",
        }
    }
}

/// Note/comment on various objects.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct Note {
    pub id: i64,
    pub body: String,
    pub author: MergeRequestUser,
    pub created_at: String,
    pub updated_at: String,
    pub system: bool,
    pub noteable_type: NoteableType,
    pub noteable_id: i64,
}
