//! To-do read endpoints: the **Todos** API category — the authenticated
//! user's action queue.
//!
//! All read-only (GET). Marking to-dos as done is a write and intentionally
//! omitted.

use gitlab_model::Todo;

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

/// Read endpoints for the current user's to-dos.
pub trait TodoEndpoints {
    /// List the authenticated user's pending to-dos.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn todos(&self) -> Result<Vec<Todo>, Error>;

    /// List to-dos filtered by state (`pending` or `done`).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn todos_by_state(&self, state: &str) -> Result<Vec<Todo>, Error>;
}

impl TodoEndpoints for GitlabClient {
    fn todos(&self) -> Result<Vec<Todo>, Error> {
        self.get_paginated("api/v4/todos")
    }

    fn todos_by_state(&self, state: &str) -> Result<Vec<Todo>, Error> {
        self.get_paginated(&format!("api/v4/todos?state={}", state.percent_encode()))
    }
}
