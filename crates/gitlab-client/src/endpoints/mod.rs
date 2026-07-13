//! API endpoint methods, grouped by resource. Each submodule defines an
//! extension trait implemented for [`GitlabClient`](crate::GitlabClient).
//!
//! All endpoints here are **read-only** (GET) — no create/update/delete.

mod commits;
mod events;
mod issues;
mod merge_requests;
mod milestones;
mod projects;
mod users;
mod wikis;

pub use commits::CommitEndpoints;
pub use events::EventEndpoints;
pub use issues::{IssueEndpoints, IssueQuery, IssueScope, IssueStateFilter};
pub use merge_requests::{MergeRequestEndpoints, MergeRequestQuery};
pub use milestones::MilestoneEndpoints;
pub use projects::ProjectEndpoints;
pub use users::UserEndpoints;
pub use wikis::WikiEndpoints;
