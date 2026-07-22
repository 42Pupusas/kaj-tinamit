//! Label endpoints: the **Project labels** and **Group labels** API
//! categories.
//!
//! Reads list/fetch labels; writes create, update, and delete them for both
//! projects and groups.

use json_bourne::ToJson;
use kaj_tinamit::Label;

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

/// Body for creating a label (`POST .../labels`). `name` and `color` are
/// required by GitLab; the rest are omitted when unset.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct CreateLabel {
    pub name: String,
    /// A 6-digit hex color prefixed with `#`, or a CSS color name.
    pub color: String,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    #[bourne(skip_if_none)]
    pub priority: Option<i64>,
}

impl CreateLabel {
    #[must_use]
    pub fn new(name: impl Into<String>, color: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            color: color.into(),
            description: None,
            priority: None,
        }
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn with_priority(mut self, priority: i64) -> Self {
        self.priority = Some(priority);
        self
    }
}

/// Body for updating a label (`PUT .../labels/:id`). The label to change is
/// identified in the path; every field here is optional. `new_name` renames.
#[derive(Debug, Clone, Default, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct UpdateLabel {
    #[bourne(skip_if_none)]
    pub new_name: Option<String>,
    #[bourne(skip_if_none)]
    pub color: Option<String>,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    #[bourne(skip_if_none)]
    pub priority: Option<i64>,
}

impl UpdateLabel {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_new_name(mut self, new_name: impl Into<String>) -> Self {
        self.new_name = Some(new_name.into());
        self
    }

    #[must_use]
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn with_priority(mut self, priority: i64) -> Self {
        self.priority = Some(priority);
        self
    }
}

/// Read endpoints for project and group labels.
pub trait LabelEndpoints {
    /// List all labels for a project. When `with_counts` is set, GitLab
    /// populates the issue / merge-request count fields.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_labels(&self, project_id: i64, with_counts: bool) -> Result<Vec<Label>, Error>;

    /// Retrieve a single project label by ID or title.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_label(&self, project_id: i64, label: &str) -> Result<Label, Error>;

    /// List all labels for a group. When `with_counts` is set, GitLab
    /// populates the issue / merge-request count fields.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_labels(&self, group_id: i64, with_counts: bool) -> Result<Vec<Label>, Error>;

    /// Retrieve a single group label by ID or title.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_label(&self, group_id: i64, label: &str) -> Result<Label, Error>;

    // --- Writes ---

    /// Create a new project label. Returns the created label.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_project_label(&self, project_id: i64, label: &CreateLabel) -> Result<Label, Error>;

    /// Update a project label (identified by ID or title). Returns the label.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn update_project_label(
        &self,
        project_id: i64,
        label: &str,
        update: &UpdateLabel,
    ) -> Result<Label, Error>;

    /// Delete a project label (by ID or title).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_project_label(&self, project_id: i64, label: &str) -> Result<(), Error>;

    /// Create a new group label. Returns the created label.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_group_label(&self, group_id: i64, label: &CreateLabel) -> Result<Label, Error>;

    /// Update a group label (identified by ID or title). Returns the label.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn update_group_label(
        &self,
        group_id: i64,
        label: &str,
        update: &UpdateLabel,
    ) -> Result<Label, Error>;

    /// Delete a group label (by ID or title).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_group_label(&self, group_id: i64, label: &str) -> Result<(), Error>;
}

impl LabelEndpoints for GitlabClient {
    fn project_labels(&self, project_id: i64, with_counts: bool) -> Result<Vec<Label>, Error> {
        let mut url = format!("api/v4/projects/{project_id}/labels");
        if with_counts {
            url.push_str("?with_counts=true");
        }
        self.get_paginated(&url)
    }

    fn project_label(&self, project_id: i64, label: &str) -> Result<Label, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/labels/{}",
            label.percent_encode()
        ))
    }

    fn group_labels(&self, group_id: i64, with_counts: bool) -> Result<Vec<Label>, Error> {
        let mut url = format!("api/v4/groups/{group_id}/labels");
        if with_counts {
            url.push_str("?with_counts=true");
        }
        self.get_paginated(&url)
    }

    fn group_label(&self, group_id: i64, label: &str) -> Result<Label, Error> {
        self.get(&format!(
            "api/v4/groups/{group_id}/labels/{}",
            label.percent_encode()
        ))
    }

    fn create_project_label(&self, project_id: i64, label: &CreateLabel) -> Result<Label, Error> {
        self.post(&format!("api/v4/projects/{project_id}/labels"), label)
    }

    fn update_project_label(
        &self,
        project_id: i64,
        label: &str,
        update: &UpdateLabel,
    ) -> Result<Label, Error> {
        self.put(
            &format!(
                "api/v4/projects/{project_id}/labels/{}",
                label.percent_encode()
            ),
            update,
        )
    }

    fn delete_project_label(&self, project_id: i64, label: &str) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/projects/{project_id}/labels/{}",
            label.percent_encode()
        ))
    }

    fn create_group_label(&self, group_id: i64, label: &CreateLabel) -> Result<Label, Error> {
        self.post(&format!("api/v4/groups/{group_id}/labels"), label)
    }

    fn update_group_label(
        &self,
        group_id: i64,
        label: &str,
        update: &UpdateLabel,
    ) -> Result<Label, Error> {
        self.put(
            &format!("api/v4/groups/{group_id}/labels/{}", label.percent_encode()),
            update,
        )
    }

    fn delete_group_label(&self, group_id: i64, label: &str) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/groups/{group_id}/labels/{}",
            label.percent_encode()
        ))
    }
}
