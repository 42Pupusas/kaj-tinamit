//! API endpoint methods, grouped by resource. Each submodule defines an
//! extension trait implemented for [`GitlabClient`](crate::GitlabClient).
//!
//! All endpoints here are **read-only** (GET) — no create/update/delete.

mod ci;
mod commits;
mod deployments;
mod events;
mod groups;
mod issues;
mod labels;
mod merge_requests;
mod milestones;
mod projects;
mod releases;
mod repository;
mod runners;
mod snippets;
mod users;
mod wikis;

pub use ci::{JobEndpoints, PipelineEndpoints, PipelineQuery};
pub use commits::CommitEndpoints;
pub use deployments::{DeploymentEndpoints, EnvironmentEndpoints};
pub use events::EventEndpoints;
pub use groups::{GroupEndpoints, MemberEndpoints};
pub use issues::{IssueEndpoints, IssueQuery, IssueScope, IssueStateFilter};
pub use labels::LabelEndpoints;
pub use merge_requests::{MergeRequestEndpoints, MergeRequestQuery};
pub use milestones::MilestoneEndpoints;
pub use projects::ProjectEndpoints;
pub use releases::ReleaseEndpoints;
pub use repository::RepositoryEndpoints;
pub use runners::{RunnerEndpoints, RunnerQuery};
pub use snippets::{ProjectSnippetEndpoints, SnippetEndpoints};
pub use users::UserEndpoints;
pub use wikis::WikiEndpoints;
