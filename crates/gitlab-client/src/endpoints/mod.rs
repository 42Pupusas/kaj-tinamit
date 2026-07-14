//! API endpoint methods, grouped by resource. Each submodule defines an
//! extension trait implemented for [`GitlabClient`](crate::GitlabClient).
//!
//! All endpoints here are **read-only** (GET) — no create/update/delete.

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
mod milestones;
mod notes;
mod projects;
mod releases;
mod repository;
mod resource_events;
mod runners;
mod search;
mod snippets;
mod todos;
mod users;
mod wikis;

pub use boards::BoardEndpoints;
pub use ci::{JobEndpoints, PipelineEndpoints, PipelineQuery};
pub use commits::CommitEndpoints;
pub use deploy::{DeployKeyEndpoints, DeployTokenEndpoints};
pub use deployments::{DeploymentEndpoints, EnvironmentEndpoints};
pub use epics::EpicEndpoints;
pub use events::EventEndpoints;
pub use groups::{GroupEndpoints, MemberEndpoints};
pub use issue_links::IssueLinkEndpoints;
pub use issues::{IssueEndpoints, IssueQuery, IssueScope, IssueStateFilter};
pub use iterations::IterationEndpoints;
pub use labels::LabelEndpoints;
pub use merge_requests::{MergeRequestEndpoints, MergeRequestQuery};
pub use milestones::MilestoneEndpoints;
pub use notes::NoteEndpoints;
pub use projects::ProjectEndpoints;
pub use releases::ReleaseEndpoints;
pub use repository::RepositoryEndpoints;
pub use resource_events::ResourceEventEndpoints;
pub use runners::{RunnerEndpoints, RunnerQuery};
pub use search::SearchEndpoints;
pub use snippets::{ProjectSnippetEndpoints, SnippetEndpoints};
pub use todos::TodoEndpoints;
pub use users::UserEndpoints;
pub use wikis::WikiEndpoints;
