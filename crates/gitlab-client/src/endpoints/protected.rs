//! Governance read endpoints: the **Protected branches**, **Protected
//! tags**, and **Protected environments** API categories.
//!
//! All read-only (GET). Protect/unprotect (write) are intentionally omitted.

use gitlab_model::{ProtectedBranch, ProtectedEnvironment, ProtectedTag};

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

/// Read endpoints for a project's protection rules.
pub trait ProtectedEndpoints {
    /// List a project's protected branches.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn protected_branches(&self, project_id: i64) -> Result<Vec<ProtectedBranch>, Error>;

    /// Retrieve a single protected branch by name.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn protected_branch(&self, project_id: i64, name: &str) -> Result<ProtectedBranch, Error>;

    /// List a project's protected tags.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn protected_tags(&self, project_id: i64) -> Result<Vec<ProtectedTag>, Error>;

    /// Retrieve a single protected tag by name.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn protected_tag(&self, project_id: i64, name: &str) -> Result<ProtectedTag, Error>;

    /// List a project's protected environments.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn protected_environments(
        &self,
        project_id: i64,
    ) -> Result<Vec<ProtectedEnvironment>, Error>;

    /// Retrieve a single protected environment by name.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn protected_environment(
        &self,
        project_id: i64,
        name: &str,
    ) -> Result<ProtectedEnvironment, Error>;
}

impl ProtectedEndpoints for GitlabClient {
    fn protected_branches(&self, project_id: i64) -> Result<Vec<ProtectedBranch>, Error> {
        self.get_paginated(&format!("api/v4/projects/{project_id}/protected_branches"))
    }

    fn protected_branch(&self, project_id: i64, name: &str) -> Result<ProtectedBranch, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/protected_branches/{}",
            name.percent_encode()
        ))
    }

    fn protected_tags(&self, project_id: i64) -> Result<Vec<ProtectedTag>, Error> {
        self.get_paginated(&format!("api/v4/projects/{project_id}/protected_tags"))
    }

    fn protected_tag(&self, project_id: i64, name: &str) -> Result<ProtectedTag, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/protected_tags/{}",
            name.percent_encode()
        ))
    }

    fn protected_environments(
        &self,
        project_id: i64,
    ) -> Result<Vec<ProtectedEnvironment>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/protected_environments"
        ))
    }

    fn protected_environment(
        &self,
        project_id: i64,
        name: &str,
    ) -> Result<ProtectedEnvironment, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/protected_environments/{}",
            name.percent_encode()
        ))
    }
}
