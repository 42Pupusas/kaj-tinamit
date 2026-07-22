//! Issue-link endpoints: the **Issue links** API category — typed
//! dependencies between issues.
//!
//! Reads list an issue's links; writes create and delete them.

use json_bourne::ToJson;
use kaj_tinamit::{IssueLink, IssueLinkResult, IssueLinkType};

use crate::client::GitlabClient;
use crate::error::Error;

/// Body for creating an issue link
/// (`POST /projects/:id/issues/:iid/links`).
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
struct CreateIssueLinkBody {
    target_project_id: i64,
    target_issue_iid: i64,
    #[bourne(skip_if_none)]
    link_type: Option<IssueLinkType>,
}

/// The relationship a new link expresses, from the source issue's side.
///
/// Re-exported convenience alias so callers need not reach into the model
/// crate for the common case.
pub use kaj_tinamit::IssueLinkType as LinkType;

/// Read endpoints for issue links (dependencies).
pub trait IssueLinkEndpoints {
    /// List all issues linked to the given issue, with their link type
    /// (`relates_to`, `blocks`, `is_blocked_by`).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn issue_links(&self, project_id: i64, issue_iid: i64) -> Result<Vec<IssueLink>, Error>;

    // --- Writes ---

    /// Link the given issue to a target issue with an optional relationship
    /// type (defaults to `relates_to` on GitLab). Returns the source/target
    /// issues and the link type.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_issue_link(
        &self,
        project_id: i64,
        issue_iid: i64,
        target_project_id: i64,
        target_issue_iid: i64,
        link_type: Option<IssueLinkType>,
    ) -> Result<IssueLinkResult, Error>;

    /// Delete an issue link by its link row ID (`issue_link_id`). Returns
    /// the two issues that were unlinked.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_issue_link(
        &self,
        project_id: i64,
        issue_iid: i64,
        issue_link_id: i64,
    ) -> Result<IssueLinkResult, Error>;
}

impl IssueLinkEndpoints for GitlabClient {
    fn issue_links(&self, project_id: i64, issue_iid: i64) -> Result<Vec<IssueLink>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/issues/{issue_iid}/links"
        ))
    }

    fn create_issue_link(
        &self,
        project_id: i64,
        issue_iid: i64,
        target_project_id: i64,
        target_issue_iid: i64,
        link_type: Option<IssueLinkType>,
    ) -> Result<IssueLinkResult, Error> {
        self.post(
            &format!("api/v4/projects/{project_id}/issues/{issue_iid}/links"),
            &CreateIssueLinkBody {
                target_project_id,
                target_issue_iid,
                link_type,
            },
        )
    }

    fn delete_issue_link(
        &self,
        project_id: i64,
        issue_iid: i64,
        issue_link_id: i64,
    ) -> Result<IssueLinkResult, Error> {
        // DELETE here returns the unlinked issues, so parse the body.
        self.delete_with_response(&format!(
            "api/v4/projects/{project_id}/issues/{issue_iid}/links/{issue_link_id}"
        ))
    }
}
