//! Release read endpoints: the **Releases** API category.
//!
//! All read-only (GET). Create/update/delete/evidence are intentionally
//! omitted.

use gitlab_model::Release;

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

/// Read endpoints for project releases.
pub trait ReleaseEndpoints {
    /// List a project's releases, sorted by `released_at` (newest first).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn releases(&self, project_id: i64) -> Result<Vec<Release>, Error>;

    /// Retrieve a single release by its associated Git tag name.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn release(&self, project_id: i64, tag_name: &str) -> Result<Release, Error>;

    /// Retrieve the latest release via the permanent `permalink/latest` URL.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn latest_release(&self, project_id: i64) -> Result<Release, Error>;

    /// List releases across all projects in a group (the **Group releases**
    /// category).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_releases(&self, group_id: i64) -> Result<Vec<Release>, Error>;
}

impl ReleaseEndpoints for GitlabClient {
    fn releases(&self, project_id: i64) -> Result<Vec<Release>, Error> {
        self.get_paginated(&format!("api/v4/projects/{project_id}/releases"))
    }

    fn release(&self, project_id: i64, tag_name: &str) -> Result<Release, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/releases/{}",
            tag_name.percent_encode()
        ))
    }

    fn latest_release(&self, project_id: i64) -> Result<Release, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/releases/permalink/latest"
        ))
    }

    fn group_releases(&self, group_id: i64) -> Result<Vec<Release>, Error> {
        self.get_paginated(&format!("api/v4/groups/{group_id}/releases"))
    }
}
