//! Wiki endpoints (project- and group-scoped).
//!
//! Reads list/fetch pages; writes create, update, and delete them.

use json_bourne::ToJson;
use kaj_tinamit::{WikiPage, WikiPageList};

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

/// Body for creating a wiki page. `title` and `content` are required;
/// `format` (`markdown` default, or `rdoc`/`asciidoc`/`org`) is optional.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct CreateWikiPage {
    pub title: String,
    pub content: String,
    #[bourne(skip_if_none)]
    pub format: Option<String>,
}

impl CreateWikiPage {
    #[must_use]
    pub fn new(title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            format: None,
        }
    }

    #[must_use]
    pub fn with_format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }
}

/// Body for updating a wiki page. Every field optional — supply only what
/// changes (retitle, rewrite content, or switch format).
#[derive(Debug, Clone, Default, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct UpdateWikiPage {
    #[bourne(skip_if_none)]
    pub title: Option<String>,
    #[bourne(skip_if_none)]
    pub content: Option<String>,
    #[bourne(skip_if_none)]
    pub format: Option<String>,
}

impl UpdateWikiPage {
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
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    #[must_use]
    pub fn with_format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }
}

/// Wiki read endpoints.
pub trait WikiEndpoints {
    /// List all wiki pages for a project (entries omit `content`).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn wiki_pages(&self, project_id: i64) -> Result<Vec<WikiPageList>, Error>;

    /// Get a single wiki page (including `content`) by slug.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn wiki_page(&self, project_id: i64, slug: &str) -> Result<WikiPage, Error>;

    /// List all wiki pages for a group (entries omit `content`).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_wiki_pages(&self, group_id: i64) -> Result<Vec<WikiPageList>, Error>;

    /// Get a single group wiki page (including `content`) by slug.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_wiki_page(&self, group_id: i64, slug: &str) -> Result<WikiPage, Error>;

    // --- Writes ---

    /// Create a project wiki page. Returns the created page.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_wiki_page(&self, project_id: i64, page: &CreateWikiPage) -> Result<WikiPage, Error>;

    /// Update a project wiki page (by slug). Returns the updated page.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn update_wiki_page(
        &self,
        project_id: i64,
        slug: &str,
        update: &UpdateWikiPage,
    ) -> Result<WikiPage, Error>;

    /// Delete a project wiki page (by slug).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_wiki_page(&self, project_id: i64, slug: &str) -> Result<(), Error>;

    /// Create a group wiki page. Returns the created page.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_group_wiki_page(
        &self,
        group_id: i64,
        page: &CreateWikiPage,
    ) -> Result<WikiPage, Error>;

    /// Update a group wiki page (by slug). Returns the updated page.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn update_group_wiki_page(
        &self,
        group_id: i64,
        slug: &str,
        update: &UpdateWikiPage,
    ) -> Result<WikiPage, Error>;

    /// Delete a group wiki page (by slug).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_group_wiki_page(&self, group_id: i64, slug: &str) -> Result<(), Error>;
}

impl WikiEndpoints for GitlabClient {
    fn wiki_pages(&self, project_id: i64) -> Result<Vec<WikiPageList>, Error> {
        self.get_paginated(&format!("api/v4/projects/{project_id}/wikis"))
    }

    fn wiki_page(&self, project_id: i64, slug: &str) -> Result<WikiPage, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/wikis/{}",
            slug.percent_encode()
        ))
    }

    fn group_wiki_pages(&self, group_id: i64) -> Result<Vec<WikiPageList>, Error> {
        self.get_paginated(&format!("api/v4/groups/{group_id}/wikis"))
    }

    fn group_wiki_page(&self, group_id: i64, slug: &str) -> Result<WikiPage, Error> {
        self.get(&format!(
            "api/v4/groups/{group_id}/wikis/{}",
            slug.percent_encode()
        ))
    }

    fn create_wiki_page(&self, project_id: i64, page: &CreateWikiPage) -> Result<WikiPage, Error> {
        self.post(&format!("api/v4/projects/{project_id}/wikis"), page)
    }

    fn update_wiki_page(
        &self,
        project_id: i64,
        slug: &str,
        update: &UpdateWikiPage,
    ) -> Result<WikiPage, Error> {
        self.put(
            &format!(
                "api/v4/projects/{project_id}/wikis/{}",
                slug.percent_encode()
            ),
            update,
        )
    }

    fn delete_wiki_page(&self, project_id: i64, slug: &str) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/projects/{project_id}/wikis/{}",
            slug.percent_encode()
        ))
    }

    fn create_group_wiki_page(
        &self,
        group_id: i64,
        page: &CreateWikiPage,
    ) -> Result<WikiPage, Error> {
        self.post(&format!("api/v4/groups/{group_id}/wikis"), page)
    }

    fn update_group_wiki_page(
        &self,
        group_id: i64,
        slug: &str,
        update: &UpdateWikiPage,
    ) -> Result<WikiPage, Error> {
        self.put(
            &format!("api/v4/groups/{group_id}/wikis/{}", slug.percent_encode()),
            update,
        )
    }

    fn delete_group_wiki_page(&self, group_id: i64, slug: &str) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/groups/{group_id}/wikis/{}",
            slug.percent_encode()
        ))
    }
}
