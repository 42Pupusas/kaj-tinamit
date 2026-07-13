//! Milestone read endpoints.

use gitlab_model::Milestone;

use crate::client::GitlabClient;
use crate::error::Error;

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
}
