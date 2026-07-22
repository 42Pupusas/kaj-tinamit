//! Milestone endpoints (project- and group-scoped).
//!
//! Reads list/fetch milestones; writes create, update, and delete them.

use json_bourne::ToJson;
use kaj_tinamit::Milestone;

use crate::client::GitlabClient;
use crate::error::Error;

/// Body for creating a milestone. `title` is required; the rest are omitted
/// when unset.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct CreateMilestone {
    pub title: String,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    #[bourne(skip_if_none)]
    pub due_date: Option<String>,
    #[bourne(skip_if_none)]
    pub start_date: Option<String>,
}

impl CreateMilestone {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            due_date: None,
            start_date: None,
        }
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn with_due_date(mut self, due_date: impl Into<String>) -> Self {
        self.due_date = Some(due_date.into());
        self
    }

    #[must_use]
    pub fn with_start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }
}

/// Body for updating a milestone. Every field optional; `state_event`
/// drives activate/close (`"activate"` / `"close"`).
#[derive(Debug, Clone, Default, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct UpdateMilestone {
    #[bourne(skip_if_none)]
    pub title: Option<String>,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    #[bourne(skip_if_none)]
    pub due_date: Option<String>,
    #[bourne(skip_if_none)]
    pub start_date: Option<String>,
    #[bourne(skip_if_none)]
    pub state_event: Option<String>,
}

impl UpdateMilestone {
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
    pub fn with_due_date(mut self, due_date: impl Into<String>) -> Self {
        self.due_date = Some(due_date.into());
        self
    }

    #[must_use]
    pub fn with_start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }

    /// Close the milestone on update.
    #[must_use]
    pub fn close(mut self) -> Self {
        self.state_event = Some("close".to_string());
        self
    }

    /// Activate (reopen) the milestone on update.
    #[must_use]
    pub fn activate(mut self) -> Self {
        self.state_event = Some("activate".to_string());
        self
    }
}

/// Milestone read endpoints (project- and group-scoped).
pub trait MilestoneEndpoints {
    /// Fetch all milestones for a project.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_milestones(&self, project_id: i32) -> Result<Vec<Milestone>, Error>;

    /// Fetch a single project milestone by ID.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_milestone(&self, project_id: i32, milestone_id: i32) -> Result<Milestone, Error>;

    /// Fetch all milestones for a group.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_milestones(&self, group_id: i32) -> Result<Vec<Milestone>, Error>;

    /// Fetch a single group milestone by ID.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_milestone(&self, group_id: i32, milestone_id: i32) -> Result<Milestone, Error>;

    // --- Writes ---

    /// Create a project milestone. Returns the created milestone.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_project_milestone(
        &self,
        project_id: i32,
        milestone: &CreateMilestone,
    ) -> Result<Milestone, Error>;

    /// Update a project milestone. Returns the updated milestone.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn update_project_milestone(
        &self,
        project_id: i32,
        milestone_id: i32,
        update: &UpdateMilestone,
    ) -> Result<Milestone, Error>;

    /// Delete a project milestone.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_project_milestone(&self, project_id: i32, milestone_id: i32) -> Result<(), Error>;

    /// Create a group milestone. Returns the created milestone.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_group_milestone(
        &self,
        group_id: i32,
        milestone: &CreateMilestone,
    ) -> Result<Milestone, Error>;

    /// Update a group milestone. Returns the updated milestone.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn update_group_milestone(
        &self,
        group_id: i32,
        milestone_id: i32,
        update: &UpdateMilestone,
    ) -> Result<Milestone, Error>;

    /// Delete a group milestone.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_group_milestone(&self, group_id: i32, milestone_id: i32) -> Result<(), Error>;
}

impl MilestoneEndpoints for GitlabClient {
    fn project_milestones(&self, project_id: i32) -> Result<Vec<Milestone>, Error> {
        self.get_paginated(&format!("api/v4/projects/{project_id}/milestones"))
    }

    fn project_milestone(&self, project_id: i32, milestone_id: i32) -> Result<Milestone, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/milestones/{milestone_id}"
        ))
    }

    fn group_milestones(&self, group_id: i32) -> Result<Vec<Milestone>, Error> {
        self.get_paginated(&format!("api/v4/groups/{group_id}/milestones"))
    }

    fn group_milestone(&self, group_id: i32, milestone_id: i32) -> Result<Milestone, Error> {
        self.get(&format!(
            "api/v4/groups/{group_id}/milestones/{milestone_id}"
        ))
    }

    fn create_project_milestone(
        &self,
        project_id: i32,
        milestone: &CreateMilestone,
    ) -> Result<Milestone, Error> {
        self.post(
            &format!("api/v4/projects/{project_id}/milestones"),
            milestone,
        )
    }

    fn update_project_milestone(
        &self,
        project_id: i32,
        milestone_id: i32,
        update: &UpdateMilestone,
    ) -> Result<Milestone, Error> {
        self.put(
            &format!("api/v4/projects/{project_id}/milestones/{milestone_id}"),
            update,
        )
    }

    fn delete_project_milestone(&self, project_id: i32, milestone_id: i32) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/projects/{project_id}/milestones/{milestone_id}"
        ))
    }

    fn create_group_milestone(
        &self,
        group_id: i32,
        milestone: &CreateMilestone,
    ) -> Result<Milestone, Error> {
        self.post(&format!("api/v4/groups/{group_id}/milestones"), milestone)
    }

    fn update_group_milestone(
        &self,
        group_id: i32,
        milestone_id: i32,
        update: &UpdateMilestone,
    ) -> Result<Milestone, Error> {
        self.put(
            &format!("api/v4/groups/{group_id}/milestones/{milestone_id}"),
            update,
        )
    }

    fn delete_group_milestone(&self, group_id: i32, milestone_id: i32) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/groups/{group_id}/milestones/{milestone_id}"
        ))
    }
}
