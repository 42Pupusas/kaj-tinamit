//! Project endpoints.

use gitlab_model::GitlabProject;

use crate::client::GitlabClient;
use crate::error::Error;

/// Project-related GitLab endpoints.
pub trait ProjectEndpoints {
    /// Fetch all projects the token is a member of (paginated).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn projects(&self) -> Result<Vec<GitlabProject>, Error>;

    /// Fetch a single project by numeric ID.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project(&self, id: i32) -> Result<GitlabProject, Error>;
}

impl ProjectEndpoints for GitlabClient {
    fn projects(&self) -> Result<Vec<GitlabProject>, Error> {
        self.get_paginated("api/v4/projects?membership=true")
    }

    fn project(&self, id: i32) -> Result<GitlabProject, Error> {
        self.get(&format!("api/v4/projects/{id}"))
    }
}
