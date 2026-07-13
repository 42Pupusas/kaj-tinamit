//! API endpoint methods, grouped by resource. Each submodule defines an
//! extension trait implemented for [`GitlabClient`](crate::GitlabClient).

mod issues;
mod projects;
mod users;

pub use issues::{IssueEndpoints, IssueQuery, IssueScope, IssueStateFilter};
pub use projects::ProjectEndpoints;
pub use users::UserEndpoints;
