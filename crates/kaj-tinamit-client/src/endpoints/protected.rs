//! Governance endpoints: the **Protected branches**, **Protected tags**,
//! and **Protected environments** API categories.
//!
//! Reads list/fetch protection rules; writes protect a branch/tag at a
//! chosen access level and unprotect them.

use kaj_tinamit::{AccessLevel, ProtectedBranch, ProtectedEnvironment, ProtectedTag};

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
    fn protected_environments(&self, project_id: i64) -> Result<Vec<ProtectedEnvironment>, Error>;

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

    // --- Writes ---

    /// Protect a branch (or wildcard, e.g. `release/*`) with the given push
    /// and merge access levels. Returns the protection rule.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn protect_branch(
        &self,
        project_id: i64,
        name: &str,
        push_access_level: AccessLevel,
        merge_access_level: AccessLevel,
    ) -> Result<ProtectedBranch, Error>;

    /// Unprotect a branch by name.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn unprotect_branch(&self, project_id: i64, name: &str) -> Result<(), Error>;

    /// Protect a tag (or wildcard) with the given create access level.
    /// Returns the protection rule.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn protect_tag(
        &self,
        project_id: i64,
        name: &str,
        create_access_level: AccessLevel,
    ) -> Result<ProtectedTag, Error>;

    /// Unprotect a tag by name.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn unprotect_tag(&self, project_id: i64, name: &str) -> Result<(), Error>;
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

    fn protected_environments(&self, project_id: i64) -> Result<Vec<ProtectedEnvironment>, Error> {
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

    fn protect_branch(
        &self,
        project_id: i64,
        name: &str,
        push_access_level: AccessLevel,
        merge_access_level: AccessLevel,
    ) -> Result<ProtectedBranch, Error> {
        // The protect endpoint takes its parameters in the query string.
        self.post_no_body(&format!(
            "api/v4/projects/{project_id}/protected_branches?name={}&push_access_level={}&merge_access_level={}",
            name.percent_encode(),
            push_access_level.as_raw(),
            merge_access_level.as_raw(),
        ))
    }

    fn unprotect_branch(&self, project_id: i64, name: &str) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/projects/{project_id}/protected_branches/{}",
            name.percent_encode()
        ))
    }

    fn protect_tag(
        &self,
        project_id: i64,
        name: &str,
        create_access_level: AccessLevel,
    ) -> Result<ProtectedTag, Error> {
        self.post_no_body(&format!(
            "api/v4/projects/{project_id}/protected_tags?name={}&create_access_level={}",
            name.percent_encode(),
            create_access_level.as_raw(),
        ))
    }

    fn unprotect_tag(&self, project_id: i64, name: &str) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/projects/{project_id}/protected_tags/{}",
            name.percent_encode()
        ))
    }
}
