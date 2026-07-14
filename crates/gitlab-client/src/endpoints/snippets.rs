//! Snippet endpoints: the **Snippets** (personal) and **Project snippets**
//! API categories.
//!
//! Reads list/fetch snippets and raw content; writes create, update, and
//! delete them (multi-file `files` form).

use gitlab_model::{Snippet, SnippetUserAgentDetail};
use json_bourne::ToJson;

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

/// A file within a snippet create/update body.
///
/// `action` is one of `create`, `update`, `delete`, or `move` and is
/// required by GitLab's multi-file snippet API. Use the constructors.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct SnippetFileInput {
    pub action: String,
    pub file_path: String,
    #[bourne(skip_if_none)]
    pub content: Option<String>,
    #[bourne(skip_if_none)]
    pub previous_path: Option<String>,
}

impl SnippetFileInput {
    /// Add a new file with `content`.
    #[must_use]
    pub fn create(file_path: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            action: "create".to_string(),
            file_path: file_path.into(),
            content: Some(content.into()),
            previous_path: None,
        }
    }

    /// Replace an existing file's `content`.
    #[must_use]
    pub fn update(file_path: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            action: "update".to_string(),
            file_path: file_path.into(),
            content: Some(content.into()),
            previous_path: None,
        }
    }

    /// Delete a file from the snippet.
    #[must_use]
    pub fn delete(file_path: impl Into<String>) -> Self {
        Self {
            action: "delete".to_string(),
            file_path: file_path.into(),
            content: None,
            previous_path: None,
        }
    }
}

/// Body for creating a snippet. `title`, `files`, and `visibility` are
/// required by GitLab (`visibility` is one of `private`/`internal`/`public`).
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct CreateSnippet {
    pub title: String,
    pub files: Vec<SnippetFileInput>,
    pub visibility: String,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
}

impl CreateSnippet {
    /// A new snippet with the given title, files, and visibility.
    #[must_use]
    pub fn new(
        title: impl Into<String>,
        files: Vec<SnippetFileInput>,
        visibility: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            files,
            visibility: visibility.into(),
            description: None,
        }
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// Body for updating a snippet. Every field optional; pass `files` with
/// per-file actions to change contents.
#[derive(Debug, Clone, Default, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct UpdateSnippet {
    #[bourne(skip_if_none)]
    pub title: Option<String>,
    #[bourne(skip_if_none)]
    pub files: Option<Vec<SnippetFileInput>>,
    #[bourne(skip_if_none)]
    pub visibility: Option<String>,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
}

impl UpdateSnippet {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    #[must_use]
    pub fn with_files(mut self, files: Vec<SnippetFileInput>) -> Self {
        self.files = Some(files);
        self
    }

    #[must_use]
    pub fn with_visibility(mut self, visibility: impl Into<String>) -> Self {
        self.visibility = Some(visibility.into());
        self
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// Read endpoints for personal snippets.
pub trait SnippetEndpoints {
    /// List the current user's snippets.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn snippets(&self) -> Result<Vec<Snippet>, Error>;

    /// List all snippets the current user has access to (admin/auditor see
    /// every snippet).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn all_snippets(&self) -> Result<Vec<Snippet>, Error>;

    /// List all public snippets.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn public_snippets(&self) -> Result<Vec<Snippet>, Error>;

    /// Retrieve a single snippet by ID.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn snippet(&self, id: i64) -> Result<Snippet, Error>;

    /// Retrieve a snippet's raw content as plain text.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn snippet_raw(&self, id: i64) -> Result<String, Error>;

    /// Retrieve the raw content of a single file within a snippet's repo.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn snippet_file_raw(&self, id: i64, ref_name: &str, file_path: &str) -> Result<String, Error>;

    // --- Writes ---

    /// Create a personal snippet. Returns the created snippet.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_snippet(&self, snippet: &CreateSnippet) -> Result<Snippet, Error>;

    /// Update a personal snippet (by ID). Returns the updated snippet.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn update_snippet(&self, id: i64, update: &UpdateSnippet) -> Result<Snippet, Error>;

    /// Delete a personal snippet (by ID).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_snippet(&self, id: i64) -> Result<(), Error>;
}

impl SnippetEndpoints for GitlabClient {
    fn snippets(&self) -> Result<Vec<Snippet>, Error> {
        self.get_paginated("api/v4/snippets")
    }

    fn all_snippets(&self) -> Result<Vec<Snippet>, Error> {
        self.get_paginated("api/v4/snippets/all")
    }

    fn public_snippets(&self) -> Result<Vec<Snippet>, Error> {
        self.get_paginated("api/v4/snippets/public")
    }

    fn snippet(&self, id: i64) -> Result<Snippet, Error> {
        self.get(&format!("api/v4/snippets/{id}"))
    }

    fn snippet_raw(&self, id: i64) -> Result<String, Error> {
        self.get_raw(&format!("api/v4/snippets/{id}/raw"))
    }

    fn snippet_file_raw(&self, id: i64, ref_name: &str, file_path: &str) -> Result<String, Error> {
        self.get_raw(&format!(
            "api/v4/snippets/{id}/files/{}/{}/raw",
            ref_name.percent_encode(),
            file_path.percent_encode()
        ))
    }

    fn create_snippet(&self, snippet: &CreateSnippet) -> Result<Snippet, Error> {
        self.post("api/v4/snippets", snippet)
    }

    fn update_snippet(&self, id: i64, update: &UpdateSnippet) -> Result<Snippet, Error> {
        self.put(&format!("api/v4/snippets/{id}"), update)
    }

    fn delete_snippet(&self, id: i64) -> Result<(), Error> {
        self.delete(&format!("api/v4/snippets/{id}"))
    }
}

/// Read endpoints for project snippets.
pub trait ProjectSnippetEndpoints {
    /// List a project's snippets.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_snippets(&self, project_id: i64) -> Result<Vec<Snippet>, Error>;

    /// Retrieve a single project snippet by ID.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_snippet(&self, project_id: i64, snippet_id: i64) -> Result<Snippet, Error>;

    /// Retrieve a project snippet's raw content as plain text.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_snippet_raw(&self, project_id: i64, snippet_id: i64) -> Result<String, Error>;

    /// Retrieve the raw content of a single file within a project snippet.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_snippet_file_raw(
        &self,
        project_id: i64,
        snippet_id: i64,
        ref_name: &str,
        file_path: &str,
    ) -> Result<String, Error>;

    /// Retrieve user-agent details for a project snippet (admin only).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_snippet_user_agent_detail(
        &self,
        project_id: i64,
        snippet_id: i64,
    ) -> Result<SnippetUserAgentDetail, Error>;

    // --- Writes ---

    /// Create a project snippet. Returns the created snippet.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_project_snippet(
        &self,
        project_id: i64,
        snippet: &CreateSnippet,
    ) -> Result<Snippet, Error>;

    /// Update a project snippet (by ID). Returns the updated snippet.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn update_project_snippet(
        &self,
        project_id: i64,
        snippet_id: i64,
        update: &UpdateSnippet,
    ) -> Result<Snippet, Error>;

    /// Delete a project snippet (by ID).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_project_snippet(&self, project_id: i64, snippet_id: i64) -> Result<(), Error>;
}

impl ProjectSnippetEndpoints for GitlabClient {
    fn project_snippets(&self, project_id: i64) -> Result<Vec<Snippet>, Error> {
        self.get_paginated(&format!("api/v4/projects/{project_id}/snippets"))
    }

    fn project_snippet(&self, project_id: i64, snippet_id: i64) -> Result<Snippet, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/snippets/{snippet_id}"
        ))
    }

    fn project_snippet_raw(&self, project_id: i64, snippet_id: i64) -> Result<String, Error> {
        self.get_raw(&format!(
            "api/v4/projects/{project_id}/snippets/{snippet_id}/raw"
        ))
    }

    fn project_snippet_file_raw(
        &self,
        project_id: i64,
        snippet_id: i64,
        ref_name: &str,
        file_path: &str,
    ) -> Result<String, Error> {
        self.get_raw(&format!(
            "api/v4/projects/{project_id}/snippets/{snippet_id}/files/{}/{}/raw",
            ref_name.percent_encode(),
            file_path.percent_encode()
        ))
    }

    fn project_snippet_user_agent_detail(
        &self,
        project_id: i64,
        snippet_id: i64,
    ) -> Result<SnippetUserAgentDetail, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/snippets/{snippet_id}/user_agent_detail"
        ))
    }

    fn create_project_snippet(
        &self,
        project_id: i64,
        snippet: &CreateSnippet,
    ) -> Result<Snippet, Error> {
        self.post(&format!("api/v4/projects/{project_id}/snippets"), snippet)
    }

    fn update_project_snippet(
        &self,
        project_id: i64,
        snippet_id: i64,
        update: &UpdateSnippet,
    ) -> Result<Snippet, Error> {
        self.put(
            &format!("api/v4/projects/{project_id}/snippets/{snippet_id}"),
            update,
        )
    }

    fn delete_project_snippet(&self, project_id: i64, snippet_id: i64) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/projects/{project_id}/snippets/{snippet_id}"
        ))
    }
}
