//! Project endpoints.
//!
//! Reads list/fetch projects; writes create, update, delete, and toggle
//! archive/star state.

use gitlab_model::GitlabProject;
use json_bourne::ToJson;

use crate::client::GitlabClient;
use crate::error::Error;

/// Body for creating a project. `name` is required; GitLab derives `path`
/// from it when omitted.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct CreateProject {
    pub name: String,
    #[bourne(skip_if_none)]
    pub path: Option<String>,
    #[bourne(skip_if_none)]
    pub namespace_id: Option<i64>,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    /// `private`, `internal`, or `public`.
    #[bourne(skip_if_none)]
    pub visibility: Option<String>,
    #[bourne(skip_if_none)]
    pub initialize_with_readme: Option<bool>,
    #[bourne(skip_if_none)]
    pub default_branch: Option<String>,
}

impl CreateProject {
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            path: None,
            namespace_id: None,
            description: None,
            visibility: None,
            initialize_with_readme: None,
            default_branch: None,
        }
    }

    #[must_use]
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    #[must_use]
    pub fn in_namespace(mut self, namespace_id: i64) -> Self {
        self.namespace_id = Some(namespace_id);
        self
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn with_visibility(mut self, visibility: impl Into<String>) -> Self {
        self.visibility = Some(visibility.into());
        self
    }

    #[must_use]
    pub fn initialize_with_readme(mut self, init: bool) -> Self {
        self.initialize_with_readme = Some(init);
        self
    }
}

/// Body for updating a project. Every field optional.
#[derive(Debug, Clone, Default, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct UpdateProject {
    #[bourne(skip_if_none)]
    pub name: Option<String>,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    #[bourne(skip_if_none)]
    pub visibility: Option<String>,
    #[bourne(skip_if_none)]
    pub default_branch: Option<String>,
    #[bourne(skip_if_none)]
    pub topics: Option<Vec<String>>,
}

impl UpdateProject {
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
    pub fn with_visibility(mut self, visibility: impl Into<String>) -> Self {
        self.visibility = Some(visibility.into());
        self
    }

    #[must_use]
    pub fn with_default_branch(mut self, default_branch: impl Into<String>) -> Self {
        self.default_branch = Some(default_branch.into());
        self
    }

    #[must_use]
    pub fn with_topics(mut self, topics: Vec<String>) -> Self {
        self.topics = Some(topics);
        self
    }
}

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
    fn update_project(&self, id: i32, update: &UpdateProject) -> Result<GitlabProject, Error>;

    /// Delete a project (GitLab may soft-delete then purge asynchronously).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_project(&self, id: i32) -> Result<(), Error>;

    /// Archive a project. Returns the project.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn archive_project(&self, id: i32) -> Result<GitlabProject, Error>;

    /// Unarchive a project. Returns the project.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn unarchive_project(&self, id: i32) -> Result<GitlabProject, Error>;

    /// Star a project. Returns the project.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn star_project(&self, id: i32) -> Result<GitlabProject, Error>;

    /// Unstar a project. Returns the project.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn unstar_project(&self, id: i32) -> Result<GitlabProject, Error>;
}

impl ProjectEndpoints for GitlabClient {
    fn projects(&self) -> Result<Vec<GitlabProject>, Error> {
        self.get_paginated("api/v4/projects?membership=true")
    }

    fn project(&self, id: i32) -> Result<GitlabProject, Error> {
        self.get(&format!("api/v4/projects/{id}"))
    }

    fn create_project(&self, project: &CreateProject) -> Result<GitlabProject, Error> {
        self.post("api/v4/projects", project)
    }

    fn update_project(&self, id: i32, update: &UpdateProject) -> Result<GitlabProject, Error> {
        self.put(&format!("api/v4/projects/{id}"), update)
    }

    fn delete_project(&self, id: i32) -> Result<(), Error> {
        self.delete(&format!("api/v4/projects/{id}"))
    }

    fn archive_project(&self, id: i32) -> Result<GitlabProject, Error> {
        self.post_no_body(&format!("api/v4/projects/{id}/archive"))
    }

    fn unarchive_project(&self, id: i32) -> Result<GitlabProject, Error> {
        self.post_no_body(&format!("api/v4/projects/{id}/unarchive"))
    }

    fn star_project(&self, id: i32) -> Result<GitlabProject, Error> {
        self.post_no_body(&format!("api/v4/projects/{id}/star"))
    }

    fn unstar_project(&self, id: i32) -> Result<GitlabProject, Error> {
        self.post_no_body(&format!("api/v4/projects/{id}/unstar"))
    }
}
