//! # kaj-tinamit-client
//!
//! A blocking, type-safe client for the GitLab REST API v4, built on the
//! in-house stack: HTTP via [`kaj-tinamit-http`](kaj_tinamit_http), wire shapes via
//! [`kaj-tinamit`](kaj_tinamit), JSON via `json-bourne`. No tokio, no
//! reqwest, no serde.
//!
//! ## Quick start
//!
//! ```no_run
//! use kaj_tinamit_client::GitlabClient;
//! use kaj_tinamit_client::prelude::*; // endpoint extension traits
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
    AcceptMergeRequest, AwardEmojiEndpoints, BoardEndpoints, CommitAction, CommitEndpoints,
    CommitFile, CommitStatusUpdate, CreateCommit, CreateDeployKey, CreateDeployToken,
    CreateDeployment, CreateEnvironment, CreateEpic, CreateIssue, CreateLabel, CreateMergeRequest,
    CreateMilestone, CreatePipelineSchedule, CreateProject, CreateRelease, CreateSnippet,
    CreateWikiPage, DeployKeyEndpoints, DeployTokenEndpoints, DeploymentEndpoints,
    EnvironmentEndpoints, EpicEndpoints, EventEndpoints, GroupEndpoints, IssueEndpoints,
    IssueLinkEndpoints, IssueQuery, IssueScope, IssueStateFilter, IterationEndpoints, JobEndpoints,
    LabelEndpoints, LinkType, MemberEndpoints, MergeRequestEndpoints, MergeRequestQuery,
    MetadataEndpoints, MilestoneEndpoints, NoteEndpoints, PipelineEndpoints, PipelineInput,
    PipelineQuery, PipelineScheduleEndpoints, ProjectEndpoints, ProjectSnippetEndpoints,
    ProtectedEndpoints, ReleaseEndpoints, RepositoryEndpoints, ResourceEventEndpoints,
    RunnerEndpoints, RunnerQuery, SearchEndpoints, SnippetEndpoints, SnippetFileInput,
    TodoEndpoints, UpdateDeployment, UpdateEnvironment, UpdateEpic, UpdateIssue, UpdateLabel,
    UpdateMergeRequest, UpdateMilestone, UpdatePipelineSchedule, UpdateProject, UpdateRelease,
    UpdateRunner, UpdateSnippet, UpdateWikiPage, UserEndpoints, WikiEndpoints,
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

// Re-export the wire shapes so callers need only depend on kaj-tinamit-client.
pub use kaj_tinamit as model;

// AccessLevel is an input to several write endpoints (members, protections),
// so surface it at the crate root for convenience.
pub use kaj_tinamit::AccessLevel;
