//! API endpoint methods, grouped by resource. Each submodule defines an
//! extension trait implemented for [`GitlabClient`](crate::GitlabClient).
//!
//! Most categories expose both read (`GET`) and write (`POST`/`PUT`/
//! `DELETE`) endpoints; write bodies are the small `Create*`/`Update*`
//! builder structs re-exported from the crate root.

mod award_emoji;
mod boards;
mod ci;
mod commits;
mod deploy;
mod deployments;
mod epics;
mod events;
mod groups;
mod issue_links;
mod issues;
mod iterations;
mod labels;
mod merge_requests;
mod metadata;
mod milestones;
mod notes;
mod pipeline_schedules;
mod projects;
mod protected;
mod releases;
mod repository;
mod resource_events;
mod runners;
mod search;
mod snippets;
mod todos;
mod users;
mod wikis;

pub use award_emoji::AwardEmojiEndpoints;
pub use boards::BoardEndpoints;
pub use ci::{JobEndpoints, PipelineEndpoints, PipelineInput, PipelineQuery};
pub use commits::{CommitAction, CommitEndpoints, CommitStatusUpdate, CreateCommit};
pub use deploy::{CreateDeployKey, CreateDeployToken, DeployKeyEndpoints, DeployTokenEndpoints};
pub use deployments::{
    CreateDeployment, CreateEnvironment, DeploymentEndpoints, EnvironmentEndpoints,
    UpdateDeployment, UpdateEnvironment,
};
pub use epics::{CreateEpic, EpicEndpoints, UpdateEpic};
pub use events::EventEndpoints;
pub use groups::{GroupEndpoints, MemberEndpoints};
pub use issue_links::{IssueLinkEndpoints, LinkType};
pub use issues::{
    CreateIssue, IssueEndpoints, IssueQuery, IssueScope, IssueStateFilter, UpdateIssue,
};
pub use iterations::IterationEndpoints;
pub use labels::{CreateLabel, LabelEndpoints, UpdateLabel};
pub use merge_requests::{
    AcceptMergeRequest, CreateMergeRequest, MergeRequestEndpoints, MergeRequestQuery,
    UpdateMergeRequest,
};
pub use metadata::MetadataEndpoints;
pub use milestones::{CreateMilestone, MilestoneEndpoints, UpdateMilestone};
pub use notes::NoteEndpoints;
pub use pipeline_schedules::{
    CreatePipelineSchedule, PipelineScheduleEndpoints, UpdatePipelineSchedule,
};
pub use projects::{CreateProject, ProjectEndpoints, UpdateProject};
pub use protected::ProtectedEndpoints;
pub use releases::{CreateRelease, ReleaseEndpoints, UpdateRelease};
pub use repository::{CommitFile, RepositoryEndpoints};
pub use resource_events::ResourceEventEndpoints;
pub use runners::{RunnerEndpoints, RunnerQuery, UpdateRunner};
pub use search::SearchEndpoints;
pub use snippets::{
    CreateSnippet, ProjectSnippetEndpoints, SnippetEndpoints, SnippetFileInput, UpdateSnippet,
};
pub use todos::TodoEndpoints;
pub use users::UserEndpoints;
pub use wikis::{CreateWikiPage, UpdateWikiPage, WikiEndpoints};
