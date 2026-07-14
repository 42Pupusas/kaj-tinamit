//! Endpoints for the **Runners** API category.
//!
//! Reads list/fetch runners, jobs, and managers; writes update/pause/delete
//! runners, manage project assignments, and reset authentication tokens.

use gitlab_model::{Job, Runner, RunnerAuthToken, RunnerDetail, RunnerManager};
use json_bourne::ToJson;

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

/// Body for updating a runner's configuration. All fields optional; unset
/// fields are omitted so a partial update touches only what you set.
#[derive(Debug, Clone, Default, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct UpdateRunner {
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    #[bourne(skip_if_none)]
    pub active: Option<bool>,
    #[bourne(skip_if_none)]
    pub paused: Option<bool>,
    #[bourne(skip_if_none)]
    pub tag_list: Option<Vec<String>>,
    #[bourne(skip_if_none)]
    pub run_untagged: Option<bool>,
    #[bourne(skip_if_none)]
    pub locked: Option<bool>,
    #[bourne(skip_if_none)]
    pub access_level: Option<String>,
    #[bourne(skip_if_none)]
    pub maximum_timeout: Option<i64>,
    #[bourne(skip_if_none)]
    pub maintenance_note: Option<String>,
}

impl UpdateRunner {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn with_active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    #[must_use]
    pub fn with_paused(mut self, paused: bool) -> Self {
        self.paused = Some(paused);
        self
    }

    #[must_use]
    pub fn with_tags(mut self, tags: &[&str]) -> Self {
        self.tag_list = Some(tags.iter().map(|t| (*t).to_string()).collect());
        self
    }

    #[must_use]
    pub fn with_run_untagged(mut self, run_untagged: bool) -> Self {
        self.run_untagged = Some(run_untagged);
        self
    }

    #[must_use]
    pub fn with_locked(mut self, locked: bool) -> Self {
        self.locked = Some(locked);
        self
    }

    /// Access level (`not_protected` or `ref_protected`).
    #[must_use]
    pub fn with_access_level(mut self, access_level: impl Into<String>) -> Self {
        self.access_level = Some(access_level.into());
        self
    }

    #[must_use]
    pub fn with_maximum_timeout(mut self, seconds: i64) -> Self {
        self.maximum_timeout = Some(seconds);
        self
    }

    #[must_use]
    pub fn with_maintenance_note(mut self, note: impl Into<String>) -> Self {
        self.maintenance_note = Some(note.into());
        self
    }
}

/// Filters for runner listings. Empty by default; values are percent-encoded.
#[derive(Debug, Default, Clone)]
pub struct RunnerQuery {
    params: Vec<(&'static str, String)>,
}

impl RunnerQuery {
    /// A new, empty query.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by runner type (`instance_type`, `group_type`,
    /// `project_type`).
    #[must_use]
    pub fn with_type(mut self, runner_type: &str) -> Self {
        self.params.push(("type", runner_type.to_string()));
        self
    }

    /// Filter by status (`online`, `offline`, `stale`, `never_contacted`).
    #[must_use]
    pub fn with_status(mut self, status: &str) -> Self {
        self.params.push(("status", status.to_string()));
        self
    }

    /// Filter to only paused (`true`) or active (`false`) runners.
    #[must_use]
    pub fn with_paused(mut self, paused: bool) -> Self {
        self.params.push(("paused", paused.to_string()));
        self
    }

    /// Filter by runner tags.
    #[must_use]
    pub fn with_tags(mut self, tags: &[&str]) -> Self {
        self.params.push(("tag_list", tags.join(",")));
        self
    }

    /// Render as a URL query string (without leading `?`); empty when no
    /// filters are set.
    fn to_query_string(&self) -> String {
        self.params
            .iter()
            .map(|(k, v)| format!("{k}={}", v.percent_encode()))
            .collect::<Vec<_>>()
            .join("&")
    }
}

/// Read endpoints for CI runners.
pub trait RunnerEndpoints {
    /// List all runners available to the authenticated user.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn runners(&self, query: &RunnerQuery) -> Result<Vec<Runner>, Error>;

    /// List all runners in the instance (admin/auditor only).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn all_runners(&self, query: &RunnerQuery) -> Result<Vec<Runner>, Error>;

    /// Retrieve full details of a runner by ID.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn runner(&self, id: i64) -> Result<RunnerDetail, Error>;

    /// List jobs processed by a runner.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn runner_jobs(&self, id: i64) -> Result<Vec<Job>, Error>;

    /// List the managers of a runner.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn runner_managers(&self, id: i64) -> Result<Vec<RunnerManager>, Error>;

    /// List all runners available to a project.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_runners(&self, project_id: i64) -> Result<Vec<Runner>, Error>;

    /// List all runners available to a group.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_runners(&self, group_id: i64) -> Result<Vec<Runner>, Error>;

    // --- Writes ---

    /// Update a runner's configuration. Returns the updated runner details.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn update_runner(&self, id: i64, update: &UpdateRunner) -> Result<RunnerDetail, Error>;

    /// Pause (`true`) or resume (`false`) a runner. Convenience over
    /// [`Self::update_runner`]. Returns the updated runner details.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn set_runner_paused(&self, id: i64, paused: bool) -> Result<RunnerDetail, Error>;

    /// Delete a runner.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_runner(&self, id: i64) -> Result<(), Error>;

    /// Assign an existing runner to a project. Returns the assigned runner.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn assign_runner_to_project(&self, project_id: i64, runner_id: i64) -> Result<Runner, Error>;

    /// Unassign a runner from a project.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn unassign_runner_from_project(&self, project_id: i64, runner_id: i64) -> Result<(), Error>;

    /// Reset a runner's authentication token. Returns the new token.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn reset_runner_token(&self, id: i64) -> Result<RunnerAuthToken, Error>;
}

impl RunnerEndpoints for GitlabClient {
    fn runners(&self, query: &RunnerQuery) -> Result<Vec<Runner>, Error> {
        self.get_paginated(&with_query("api/v4/runners", query))
    }

    fn all_runners(&self, query: &RunnerQuery) -> Result<Vec<Runner>, Error> {
        self.get_paginated(&with_query("api/v4/runners/all", query))
    }

    fn runner(&self, id: i64) -> Result<RunnerDetail, Error> {
        self.get(&format!("api/v4/runners/{id}"))
    }

    fn runner_jobs(&self, id: i64) -> Result<Vec<Job>, Error> {
        self.get_paginated(&format!("api/v4/runners/{id}/jobs"))
    }

    fn runner_managers(&self, id: i64) -> Result<Vec<RunnerManager>, Error> {
        self.get(&format!("api/v4/runners/{id}/managers"))
    }

    fn project_runners(&self, project_id: i64) -> Result<Vec<Runner>, Error> {
        self.get_paginated(&format!("api/v4/projects/{project_id}/runners"))
    }

    fn group_runners(&self, group_id: i64) -> Result<Vec<Runner>, Error> {
        self.get_paginated(&format!("api/v4/groups/{group_id}/runners"))
    }

    fn update_runner(&self, id: i64, update: &UpdateRunner) -> Result<RunnerDetail, Error> {
        self.put(&format!("api/v4/runners/{id}"), update)
    }

    fn set_runner_paused(&self, id: i64, paused: bool) -> Result<RunnerDetail, Error> {
        self.update_runner(id, &UpdateRunner::new().with_paused(paused))
    }

    fn delete_runner(&self, id: i64) -> Result<(), Error> {
        self.delete(&format!("api/v4/runners/{id}"))
    }

    fn assign_runner_to_project(&self, project_id: i64, runner_id: i64) -> Result<Runner, Error> {
        self.post(
            &format!("api/v4/projects/{project_id}/runners"),
            &AssignRunner { runner_id },
        )
    }

    fn unassign_runner_from_project(&self, project_id: i64, runner_id: i64) -> Result<(), Error> {
        self.delete(&format!("api/v4/projects/{project_id}/runners/{runner_id}"))
    }

    fn reset_runner_token(&self, id: i64) -> Result<RunnerAuthToken, Error> {
        self.post_no_body(&format!("api/v4/runners/{id}/reset_authentication_token"))
    }
}

/// Body for assigning an existing runner to a project.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
struct AssignRunner {
    runner_id: i64,
}

/// Append a rendered [`RunnerQuery`] to a base path when non-empty.
fn with_query(base: &str, query: &RunnerQuery) -> String {
    let qs = query.to_query_string();
    if qs.is_empty() {
        base.to_string()
    } else {
        format!("{base}?{qs}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_empty_by_default() {
        assert_eq!(RunnerQuery::new().to_query_string(), "");
    }

    #[test]
    fn query_combines_and_encodes() {
        let q = RunnerQuery::new()
            .with_type("project_type")
            .with_status("online")
            .with_paused(false);
        assert_eq!(
            q.to_query_string(),
            "type=project_type&status=online&paused=false"
        );
    }
}
