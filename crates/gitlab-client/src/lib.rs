//! # gitlab-client
//!
//! A blocking, type-safe client for the GitLab REST API v4, built on the
//! in-house stack: HTTP via [`gitlab-http`](gitlab_http), wire shapes via
//! [`gitlab-model`](gitlab_model), JSON via `json-bourne`. No tokio, no
//! reqwest, no serde.
//!
//! ## Quick start
//!
//! ```no_run
//! use gitlab_client::GitlabClient;
//! use gitlab_client::prelude::*; // endpoint extension traits
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Reads GITLAB_URL and GITLAB_PAT from the environment.
//! let client = GitlabClient::from_env()?;
//! let me = client.current_user()?;
//! println!("authenticated as {}", me.username);
//! # Ok(())
//! # }
//! ```

mod client;
mod encode;
mod endpoints;
mod error;
mod pagination;

pub use client::GitlabClient;
pub use endpoints::{
    AcceptMergeRequest, AwardEmojiEndpoints, BoardEndpoints, CommitEndpoints, CreateIssue,
    CreateLabel, CreateMergeRequest, CreateMilestone, CreateWikiPage, DeployKeyEndpoints,
    DeployTokenEndpoints, DeploymentEndpoints, EnvironmentEndpoints, EpicEndpoints, EventEndpoints,
    GroupEndpoints, IssueEndpoints, IssueLinkEndpoints, IssueQuery, IssueScope, IssueStateFilter,
    IterationEndpoints, JobEndpoints, LabelEndpoints, LinkType, MemberEndpoints,
    MergeRequestEndpoints, MergeRequestQuery, MetadataEndpoints, MilestoneEndpoints, NoteEndpoints,
    PipelineEndpoints, PipelineInput, PipelineQuery, PipelineScheduleEndpoints, ProjectEndpoints,
    ProjectSnippetEndpoints, ProtectedEndpoints, ReleaseEndpoints, RepositoryEndpoints,
    ResourceEventEndpoints, RunnerEndpoints, RunnerQuery, SearchEndpoints, SnippetEndpoints,
    TodoEndpoints, UpdateIssue, UpdateLabel, UpdateMergeRequest, UpdateMilestone, UpdateWikiPage,
    UserEndpoints, WikiEndpoints,
};
pub use error::{ConfigError, Error, HttpMethod, Resource};
pub use pagination::PaginationConfig;

/// Bring every endpoint extension trait into scope in one `use`.
pub mod prelude {
    pub use crate::endpoints::{
        AwardEmojiEndpoints, BoardEndpoints, CommitEndpoints, DeployKeyEndpoints,
        DeployTokenEndpoints, DeploymentEndpoints, EnvironmentEndpoints, EpicEndpoints,
        EventEndpoints, GroupEndpoints, IssueEndpoints, IssueLinkEndpoints, IterationEndpoints,
        JobEndpoints, LabelEndpoints, MemberEndpoints, MergeRequestEndpoints, MetadataEndpoints,
        MilestoneEndpoints, NoteEndpoints, PipelineEndpoints, PipelineScheduleEndpoints,
        ProjectEndpoints, ProjectSnippetEndpoints, ProtectedEndpoints, ReleaseEndpoints,
        RepositoryEndpoints, ResourceEventEndpoints, RunnerEndpoints, SearchEndpoints,
        SnippetEndpoints, TodoEndpoints, UserEndpoints, WikiEndpoints,
    };
}

// Re-export the wire shapes so callers need only depend on gitlab-client.
pub use gitlab_model as model;
