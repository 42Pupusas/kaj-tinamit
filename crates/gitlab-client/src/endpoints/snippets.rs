//! Snippet read endpoints: the **Snippets** (personal) and **Project
//! snippets** API categories.
//!
//! All read-only (GET). Create/update/delete are intentionally omitted.

use gitlab_model::{Snippet, SnippetUserAgentDetail};

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

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
}
