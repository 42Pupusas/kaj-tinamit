//! Issue-link read endpoints: the **Issue links** API category — typed
//! dependencies between issues.
//!
//! All read-only (GET). Create/delete of links are intentionally omitted.

use gitlab_model::IssueLink;

use crate::client::GitlabClient;
use crate::error::Error;

/// Read endpoints for issue links (dependencies).
pub trait IssueLinkEndpoints {
    /// List all issues linked to the given issue, with their link type
    /// (`relates_to`, `blocks`, `is_blocked_by`).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn issue_links(&self, project_id: i64, issue_iid: i64) -> Result<Vec<IssueLink>, Error>;
}

impl IssueLinkEndpoints for GitlabClient {
    fn issue_links(&self, project_id: i64, issue_iid: i64) -> Result<Vec<IssueLink>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/issues/{issue_iid}/links"
        ))
    }
}
