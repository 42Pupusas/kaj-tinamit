//! Project endpoints.

use gitlab_model::GitlabProject;

use crate::client::GitlabClient;
use crate::error::Error;

impl GitlabClient {
    /// Fetch all projects visible to the token (paginated).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    pub fn projects(&self) -> Result<Vec<GitlabProject>, Error> {
        self.get_paginated("api/v4/projects?membership=true")
    }

    /// Fetch a single project by numeric ID.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    pub fn project(&self, id: i32) -> Result<GitlabProject, Error> {
        self.get(&format!("api/v4/projects/{id}"))
    }
}
