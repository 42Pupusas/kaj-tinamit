//! Epic read endpoints: the **Epics** and **Epic issues** API categories
//! (group-level, premium tier).
//!
//! All read-only (GET). Create/update/delete and epic-issue assignment are
//! intentionally omitted.

use gitlab_model::{Epic, EpicIssue};

use crate::client::GitlabClient;
use crate::error::Error;

/// Read endpoints for group epics and their child issues.
///
/// Epics are addressed by their group-scoped **IID** in the sub-resource
/// paths, matching GitLab's REST surface.
pub trait EpicEndpoints {
    /// List a group's epics.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_epics(&self, group_id: i64) -> Result<Vec<Epic>, Error>;

    /// Retrieve a single epic by group ID and epic IID.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn epic(&self, group_id: i64, epic_iid: i64) -> Result<Epic, Error>;

    /// List the issues assigned to an epic.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn epic_issues(&self, group_id: i64, epic_iid: i64) -> Result<Vec<EpicIssue>, Error>;

    /// List the direct child epics of an epic.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn epic_children(&self, group_id: i64, epic_iid: i64) -> Result<Vec<Epic>, Error>;

    /// List epics linked (related) to an epic — the `related_epics` category,
    /// distinct from parent/child nesting.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn epic_related(&self, group_id: i64, epic_iid: i64) -> Result<Vec<Epic>, Error>;
}

impl EpicEndpoints for GitlabClient {
    fn group_epics(&self, group_id: i64) -> Result<Vec<Epic>, Error> {
        self.get_paginated(&format!("api/v4/groups/{group_id}/epics"))
    }

    fn epic(&self, group_id: i64, epic_iid: i64) -> Result<Epic, Error> {
        self.get(&format!("api/v4/groups/{group_id}/epics/{epic_iid}"))
    }

    fn epic_issues(&self, group_id: i64, epic_iid: i64) -> Result<Vec<EpicIssue>, Error> {
        self.get_paginated(&format!("api/v4/groups/{group_id}/epics/{epic_iid}/issues"))
    }

    fn epic_children(&self, group_id: i64, epic_iid: i64) -> Result<Vec<Epic>, Error> {
        self.get_paginated(&format!("api/v4/groups/{group_id}/epics/{epic_iid}/epics"))
    }

    fn epic_related(&self, group_id: i64, epic_iid: i64) -> Result<Vec<Epic>, Error> {
        self.get_paginated(&format!(
            "api/v4/groups/{group_id}/epics/{epic_iid}/related_epics"
        ))
    }
}
