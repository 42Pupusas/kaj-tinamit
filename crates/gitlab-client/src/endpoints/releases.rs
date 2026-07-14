//! Release endpoints: the **Releases** API category.
//!
//! Reads list/fetch releases; writes create, update, and delete them.

use gitlab_model::Release;
use json_bourne::ToJson;

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

/// Body for creating a release. `tag_name` is required; provide either an
/// existing tag or a `ref` to create the tag from.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct CreateRelease {
    pub tag_name: String,
    #[bourne(skip_if_none)]
    pub name: Option<String>,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    /// Commit SHA or branch to tag from, when `tag_name` doesn't yet exist.
    #[bourne(skip_if_none, rename = "ref")]
    pub ref_name: Option<String>,
    #[bourne(skip_if_none)]
    pub released_at: Option<String>,
    #[bourne(skip_if_none)]
    pub milestones: Option<Vec<String>>,
}

impl CreateRelease {
    #[must_use]
    pub fn new(tag_name: impl Into<String>) -> Self {
        Self {
            tag_name: tag_name.into(),
            name: None,
            description: None,
            ref_name: None,
            released_at: None,
            milestones: None,
        }
    }

    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Create the tag from this commit SHA or branch name.
    #[must_use]
    pub fn with_ref(mut self, ref_name: impl Into<String>) -> Self {
        self.ref_name = Some(ref_name.into());
        self
    }

    #[must_use]
    pub fn with_released_at(mut self, released_at: impl Into<String>) -> Self {
        self.released_at = Some(released_at.into());
        self
    }

    #[must_use]
    pub fn with_milestones(mut self, milestones: Vec<String>) -> Self {
        self.milestones = Some(milestones);
        self
    }
}

/// Body for updating a release. Every field optional.
#[derive(Debug, Clone, Default, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct UpdateRelease {
    #[bourne(skip_if_none)]
    pub name: Option<String>,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    #[bourne(skip_if_none)]
    pub released_at: Option<String>,
    #[bourne(skip_if_none)]
    pub milestones: Option<Vec<String>>,
}

impl UpdateRelease {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn with_released_at(mut self, released_at: impl Into<String>) -> Self {
        self.released_at = Some(released_at.into());
        self
    }

    #[must_use]
    pub fn with_milestones(mut self, milestones: Vec<String>) -> Self {
        self.milestones = Some(milestones);
        self
    }
}

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

    // --- Writes ---

    /// Create a release. Returns the created release.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_release(&self, project_id: i64, release: &CreateRelease) -> Result<Release, Error>;

    /// Update a release (by tag name). Returns the updated release.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn update_release(
        &self,
        project_id: i64,
        tag_name: &str,
        update: &UpdateRelease,
    ) -> Result<Release, Error>;

    /// Delete a release (by tag name). Returns the deleted release.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_release(&self, project_id: i64, tag_name: &str) -> Result<Release, Error>;
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

    fn create_release(&self, project_id: i64, release: &CreateRelease) -> Result<Release, Error> {
        self.post(&format!("api/v4/projects/{project_id}/releases"), release)
    }

    fn update_release(
        &self,
        project_id: i64,
        tag_name: &str,
        update: &UpdateRelease,
    ) -> Result<Release, Error> {
        self.put(
            &format!(
                "api/v4/projects/{project_id}/releases/{}",
                tag_name.percent_encode()
            ),
            update,
        )
    }

    fn delete_release(&self, project_id: i64, tag_name: &str) -> Result<Release, Error> {
        self.delete_with_response(&format!(
            "api/v4/projects/{project_id}/releases/{}",
            tag_name.percent_encode()
        ))
    }
}
