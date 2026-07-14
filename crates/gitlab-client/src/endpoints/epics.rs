//! Epic endpoints: the **Epics** and **Epic issues** API categories
//! (group-level, premium tier).
//!
//! Reads list/fetch epics and their children/issues; writes create, update,
//! and delete epics. (Premium-only — these 404 on non-premium instances.)

use gitlab_model::{Epic, EpicIssue};
use json_bourne::ToJson;

use crate::client::GitlabClient;
use crate::error::Error;

/// Body for creating an epic. `title` is required; the rest are omitted
/// when unset.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct CreateEpic {
    pub title: String,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    #[bourne(skip_if_none)]
    pub labels: Option<String>,
    #[bourne(skip_if_none)]
    pub confidential: Option<bool>,
    #[bourne(skip_if_none)]
    pub start_date_fixed: Option<String>,
    #[bourne(skip_if_none)]
    pub due_date_fixed: Option<String>,
}

impl CreateEpic {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            labels: None,
            confidential: None,
            start_date_fixed: None,
            due_date_fixed: None,
        }
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn with_labels(mut self, labels: &[&str]) -> Self {
        self.labels = Some(labels.join(","));
        self
    }

    #[must_use]
    pub fn confidential(mut self, confidential: bool) -> Self {
        self.confidential = Some(confidential);
        self
    }
}

/// Body for updating an epic. Every field optional; `state_event` drives
/// close/reopen.
#[derive(Debug, Clone, Default, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct UpdateEpic {
    #[bourne(skip_if_none)]
    pub title: Option<String>,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    #[bourne(skip_if_none)]
    pub labels: Option<String>,
    #[bourne(skip_if_none)]
    pub confidential: Option<bool>,
    #[bourne(skip_if_none)]
    pub state_event: Option<String>,
}

impl UpdateEpic {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn with_labels(mut self, labels: &[&str]) -> Self {
        self.labels = Some(labels.join(","));
        self
    }

    #[must_use]
    pub fn confidential(mut self, confidential: bool) -> Self {
        self.confidential = Some(confidential);
        self
    }

    /// Close the epic on update.
    #[must_use]
    pub fn close(mut self) -> Self {
        self.state_event = Some("close".to_string());
        self
    }

    /// Reopen the epic on update.
    #[must_use]
    pub fn reopen(mut self) -> Self {
        self.state_event = Some("reopen".to_string());
        self
    }
}

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

    // --- Writes ---

    /// Create an epic in a group. Returns the created epic.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_epic(&self, group_id: i64, epic: &CreateEpic) -> Result<Epic, Error>;

    /// Update an epic (by IID). Returns the updated epic.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn update_epic(&self, group_id: i64, epic_iid: i64, update: &UpdateEpic)
    -> Result<Epic, Error>;

    /// Delete an epic (by IID).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_epic(&self, group_id: i64, epic_iid: i64) -> Result<(), Error>;
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

    fn create_epic(&self, group_id: i64, epic: &CreateEpic) -> Result<Epic, Error> {
        self.post(&format!("api/v4/groups/{group_id}/epics"), epic)
    }

    fn update_epic(
        &self,
        group_id: i64,
        epic_iid: i64,
        update: &UpdateEpic,
    ) -> Result<Epic, Error> {
        self.put(
            &format!("api/v4/groups/{group_id}/epics/{epic_iid}"),
            update,
        )
    }

    fn delete_epic(&self, group_id: i64, epic_iid: i64) -> Result<(), Error> {
        self.delete(&format!("api/v4/groups/{group_id}/epics/{epic_iid}"))
    }
}
