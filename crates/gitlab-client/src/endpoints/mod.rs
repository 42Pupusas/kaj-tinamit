//! API endpoint methods, grouped by resource. Each submodule defines an
//! extension trait implemented for [`GitlabClient`](crate::GitlabClient).
//!
//! All endpoints here are **read-only** (GET) — no create/update/delete.

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
pub use commits::CommitEndpoints;
pub use deploy::{DeployKeyEndpoints, DeployTokenEndpoints};
pub use deployments::{DeploymentEndpoints, EnvironmentEndpoints};
pub use epics::EpicEndpoints;
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
pub use pipeline_schedules::PipelineScheduleEndpoints;
pub use projects::ProjectEndpoints;
pub use protected::ProtectedEndpoints;
pub use releases::ReleaseEndpoints;
pub use repository::RepositoryEndpoints;
pub use resource_events::ResourceEventEndpoints;
pub use runners::{RunnerEndpoints, RunnerQuery};
pub use search::SearchEndpoints;
pub use snippets::{ProjectSnippetEndpoints, SnippetEndpoints};
pub use todos::TodoEndpoints;
pub use users::UserEndpoints;
pub use wikis::{CreateWikiPage, UpdateWikiPage, WikiEndpoints};
