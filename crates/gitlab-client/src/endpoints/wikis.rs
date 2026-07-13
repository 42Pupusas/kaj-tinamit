//! Wiki read endpoints.

use gitlab_model::{WikiPage, WikiPageList};

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

/// Wiki read endpoints.
pub trait WikiEndpoints {
    /// List all wiki pages for a project (entries omit `content`).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn wiki_pages(&self, project_id: i32) -> Result<Vec<WikiPageList>, Error>;

    /// Get a single wiki page (including `content`) by slug.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn wiki_page(&self, project_id: i32, slug: &str) -> Result<WikiPage, Error>;
}

impl WikiEndpoints for GitlabClient {
    fn wiki_pages(&self, project_id: i32) -> Result<Vec<WikiPageList>, Error> {
        self.get_paginated(&format!("api/v4/projects/{project_id}/wikis"))
    }

    fn wiki_page(&self, project_id: i32, slug: &str) -> Result<WikiPage, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/wikis/{}",
            slug.percent_encode()
        ))
    }
}
