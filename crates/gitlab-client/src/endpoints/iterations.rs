//! Iteration read endpoints: the **Iterations** and **Iteration cadences**
//! API categories — GitLab's native sprints.
//!
//! All read-only (GET). Create/update/delete cadences are intentionally
//! omitted.

use gitlab_model::{Iteration, IterationCadence};

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

/// Read endpoints for iterations (sprints) and their cadences.
///
/// Iterations are owned by groups; the project-scoped listing returns the
/// iterations inherited from the project's ancestor groups.
pub trait IterationEndpoints {
    /// List a group's iterations.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_iterations(&self, group_id: i64) -> Result<Vec<Iteration>, Error>;

    /// List a group's iterations filtered by state (`opened`, `upcoming`,
    /// `current`, `closed`, `all`).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_iterations_by_state(
        &self,
        group_id: i64,
        state: &str,
    ) -> Result<Vec<Iteration>, Error>;

    /// List the iterations that apply to a project (inherited from its
    /// ancestor groups).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_iterations(&self, project_id: i64) -> Result<Vec<Iteration>, Error>;

    /// List a group's iteration cadences.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_iteration_cadences(&self, group_id: i64) -> Result<Vec<IterationCadence>, Error>;
}

impl IterationEndpoints for GitlabClient {
    fn group_iterations(&self, group_id: i64) -> Result<Vec<Iteration>, Error> {
        self.get_paginated(&format!("api/v4/groups/{group_id}/iterations"))
    }

    fn group_iterations_by_state(
        &self,
        group_id: i64,
        state: &str,
    ) -> Result<Vec<Iteration>, Error> {
        self.get_paginated(&format!(
            "api/v4/groups/{group_id}/iterations?state={}",
            state.percent_encode()
        ))
    }

    fn project_iterations(&self, project_id: i64) -> Result<Vec<Iteration>, Error> {
        self.get_paginated(&format!("api/v4/projects/{project_id}/iterations"))
    }

    fn group_iteration_cadences(&self, group_id: i64) -> Result<Vec<IterationCadence>, Error> {
        self.get_paginated(&format!("api/v4/groups/{group_id}/iteration_cadences"))
    }
}
