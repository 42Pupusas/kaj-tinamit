//! Activity-event read endpoints.

use gitlab_model::GitlabEvent;

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

/// Activity-event read endpoints.
pub trait EventEndpoints {
    /// Fetch the authenticated user's activity events, optionally only those
    /// created on or after `after` (ISO 8601 *date*, e.g. `2021-01-01`).
    ///
    /// GitLab's `after` filter is an **exclusive** date — it returns events
    /// created strictly after that day. Pass the day before your window
    /// start to include the whole window.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn events(&self, after: Option<&str>) -> Result<Vec<GitlabEvent>, Error>;

    /// Fetch activity events for a single project — **all** members'
    /// activity, not just the caller's, and needs no admin privileges.
    ///
    /// `after` is an exclusive ISO 8601 *date* filter (see [`Self::events`]).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_events(
        &self,
        project_id: i64,
        after: Option<&str>,
    ) -> Result<Vec<GitlabEvent>, Error>;
}

impl EventEndpoints for GitlabClient {
    fn events(&self, after: Option<&str>) -> Result<Vec<GitlabEvent>, Error> {
        let mut path = String::from("api/v4/events");
        if let Some(after) = after {
            path.push_str(&format!("?after={}", after.percent_encode()));
        }
        self.get_paginated(&path)
    }

    fn project_events(
        &self,
        project_id: i64,
        after: Option<&str>,
    ) -> Result<Vec<GitlabEvent>, Error> {
        let mut path = format!("api/v4/projects/{project_id}/events");
        if let Some(after) = after {
            path.push_str(&format!("?after={}", after.percent_encode()));
        }
        self.get_paginated(&path)
    }
}
