//! Project endpoints.
//!
//! Reads list/fetch projects and their forks; writes create, update,
//! delete, archive, star, fork, transfer, and share projects.

use json_bourne::ToJson;
use kaj_tinamit::{AccessLevel, GitlabProject};

use super::{CreateProject, ForkProject, ProjectQuery, UpdateProject};
use crate::client::GitlabClient;
use crate::error::Error;

/// Body for `PUT /projects/:id/transfer`.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
struct TransferBody {
    namespace: String,
}

/// Body for `POST /projects/:id/share`.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
struct ShareBody {
    group_id: i64,
    group_access: i32,
    #[bourne(skip_if_none)]
    expires_at: Option<String>,
}

/// Project-related GitLab endpoints.
pub trait ProjectEndpoints {
    /// List projects matching `query` (paginated).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn projects(&self, query: &ProjectQuery) -> Result<Vec<GitlabProject>, Error>;

    /// Fetch a single project by numeric ID.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project(&self, id: i64) -> Result<GitlabProject, Error>;

    /// List the forks of a project.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn forks(&self, id: i64) -> Result<Vec<GitlabProject>, Error>;

    // --- Writes ---

    /// Create a new project. Returns the created project.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_project(&self, project: &CreateProject) -> Result<GitlabProject, Error>;

    /// Update a project. Returns the updated project.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn update_project(&self, id: i64, update: &UpdateProject) -> Result<GitlabProject, Error>;

    /// Delete a project (GitLab may soft-delete then purge asynchronously).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_project(&self, id: i64) -> Result<(), Error>;

    /// Archive a project. Returns the project.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn archive_project(&self, id: i64) -> Result<GitlabProject, Error>;

    /// Unarchive a project. Returns the project.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn unarchive_project(&self, id: i64) -> Result<GitlabProject, Error>;

    /// Star a project. Returns the project.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn star_project(&self, id: i64) -> Result<GitlabProject, Error>;

    /// Unstar a project. Returns the project.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn unstar_project(&self, id: i64) -> Result<GitlabProject, Error>;

    /// Fork a project. Returns the new fork, whose repository may still be
    /// importing (`import_status`) when this returns.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn fork_project(&self, id: i64, fork: &ForkProject) -> Result<GitlabProject, Error>;

    /// Remove the fork relationship between a project and its upstream.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn unlink_fork(&self, id: i64) -> Result<(), Error>;

    /// Move a project into another namespace, given by ID or full path.
    /// Returns the moved project.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn transfer_project(&self, id: i64, namespace: &str) -> Result<GitlabProject, Error>;

    /// Grant a group `access` to a project, optionally until `expires_at`
    /// (`YYYY-MM-DD`).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn share_project_with_group(
        &self,
        id: i64,
        group_id: i64,
        access: AccessLevel,
        expires_at: Option<&str>,
    ) -> Result<(), Error>;

    /// Revoke a group's shared access to a project.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn unshare_project_with_group(&self, id: i64, group_id: i64) -> Result<(), Error>;
}

impl ProjectEndpoints for GitlabClient {
    fn projects(&self, query: &ProjectQuery) -> Result<Vec<GitlabProject>, Error> {
        self.get_paginated(&query.apply("api/v4/projects".to_string()))
    }

    fn project(&self, id: i64) -> Result<GitlabProject, Error> {
        self.get(&format!("api/v4/projects/{id}"))
    }

    fn forks(&self, id: i64) -> Result<Vec<GitlabProject>, Error> {
        self.get_paginated(&format!("api/v4/projects/{id}/forks"))
    }

    fn create_project(&self, project: &CreateProject) -> Result<GitlabProject, Error> {
        self.post("api/v4/projects", project)
    }

    fn update_project(&self, id: i64, update: &UpdateProject) -> Result<GitlabProject, Error> {
        self.put(&format!("api/v4/projects/{id}"), update)
    }

    fn delete_project(&self, id: i64) -> Result<(), Error> {
        self.delete(&format!("api/v4/projects/{id}"))
    }

    fn archive_project(&self, id: i64) -> Result<GitlabProject, Error> {
        self.post_no_body(&format!("api/v4/projects/{id}/archive"))
    }

    fn unarchive_project(&self, id: i64) -> Result<GitlabProject, Error> {
        self.post_no_body(&format!("api/v4/projects/{id}/unarchive"))
    }

    fn star_project(&self, id: i64) -> Result<GitlabProject, Error> {
        self.post_no_body(&format!("api/v4/projects/{id}/star"))
    }

    fn unstar_project(&self, id: i64) -> Result<GitlabProject, Error> {
        self.post_no_body(&format!("api/v4/projects/{id}/unstar"))
    }

    fn fork_project(&self, id: i64, fork: &ForkProject) -> Result<GitlabProject, Error> {
        self.post(&format!("api/v4/projects/{id}/fork"), fork)
    }

    fn unlink_fork(&self, id: i64) -> Result<(), Error> {
        self.delete(&format!("api/v4/projects/{id}/fork"))
    }

    fn transfer_project(&self, id: i64, namespace: &str) -> Result<GitlabProject, Error> {
        self.put(
            &format!("api/v4/projects/{id}/transfer"),
            &TransferBody {
                namespace: namespace.to_string(),
            },
        )
    }

    fn share_project_with_group(
        &self,
        id: i64,
        group_id: i64,
        access: AccessLevel,
        expires_at: Option<&str>,
    ) -> Result<(), Error> {
        self.post_body_discard(
            &format!("api/v4/projects/{id}/share"),
            &ShareBody {
                group_id,
                group_access: access.as_raw(),
                expires_at: expires_at.map(str::to_string),
            },
        )
    }

    fn unshare_project_with_group(&self, id: i64, group_id: i64) -> Result<(), Error> {
        self.delete(&format!("api/v4/projects/{id}/share/{group_id}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn share_body_uses_the_numeric_access_level() {
        let body = ShareBody {
            group_id: 4,
            group_access: AccessLevel::Developer.as_raw(),
            expires_at: None,
        };
        assert_eq!(
            json_bourne::to_string(&body).unwrap(),
            r#"{"group_id":4,"group_access":30}"#
        );
    }
}
