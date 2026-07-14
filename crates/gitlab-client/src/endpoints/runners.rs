//! Runner read endpoints: the **Runners** API category.
//!
//! All read-only (GET). Create/update/pause/delete/reset are intentionally
//! omitted.

use gitlab_model::{Job, Runner, RunnerDetail, RunnerManager};

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

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
