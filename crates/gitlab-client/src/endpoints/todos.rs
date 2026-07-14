//! To-do endpoints: the **Todos** API category — the authenticated user's
//! action queue.
//!
//! Reads list pending/done to-dos; writes mark a single to-do (or all of
//! them) as done.

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

    /// Mark a single to-do as done. Returns the updated to-do.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn mark_todo_done(&self, todo_id: i64) -> Result<Todo, Error>;

    /// Mark all of the caller's pending to-dos as done.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn mark_all_todos_done(&self) -> Result<(), Error>;
}

impl TodoEndpoints for GitlabClient {
    fn todos(&self) -> Result<Vec<Todo>, Error> {
        self.get_paginated("api/v4/todos")
    }

    fn todos_by_state(&self, state: &str) -> Result<Vec<Todo>, Error> {
        self.get_paginated(&format!("api/v4/todos?state={}", state.percent_encode()))
    }

    fn mark_todo_done(&self, todo_id: i64) -> Result<Todo, Error> {
        self.post_no_body(&format!("api/v4/todos/{todo_id}/mark_as_done"))
    }

    fn mark_all_todos_done(&self) -> Result<(), Error> {
        // GitLab returns 204 No Content; nothing to parse.
        self.post_discard("api/v4/todos/mark_as_done")
    }
}
