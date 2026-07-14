//! Label read endpoints: the **Project labels** and **Group labels** API
//! categories.
//!
//! All read-only (GET). Create/update/delete/subscribe are intentionally
//! omitted.

use gitlab_model::Label;

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

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
}
