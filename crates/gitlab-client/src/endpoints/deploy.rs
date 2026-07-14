//! Read endpoints for the **Deploy keys** and **Deploy tokens** API
//! categories.
//!
//! All read-only (GET). Create/update/enable/delete are intentionally
//! omitted.

use gitlab_model::{DeployKey, DeployToken};

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

/// Read endpoints for deploy keys.
pub trait DeployKeyEndpoints {
    /// List all deploy keys across the instance (admin only).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn all_deploy_keys(&self) -> Result<Vec<DeployKey>, Error>;

    /// List a project's deploy keys.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_deploy_keys(&self, project_id: i64) -> Result<Vec<DeployKey>, Error>;

    /// Retrieve a single project deploy key by ID.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_deploy_key(&self, project_id: i64, key_id: i64) -> Result<DeployKey, Error>;

    /// List the deploy keys a user shares with the authenticated user via
    /// common projects.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn user_project_deploy_keys(&self, user: &str) -> Result<Vec<DeployKey>, Error>;
}

impl DeployKeyEndpoints for GitlabClient {
    fn all_deploy_keys(&self) -> Result<Vec<DeployKey>, Error> {
        self.get_paginated("api/v4/deploy_keys")
    }

    fn project_deploy_keys(&self, project_id: i64) -> Result<Vec<DeployKey>, Error> {
        self.get_paginated(&format!("api/v4/projects/{project_id}/deploy_keys"))
    }

    fn project_deploy_key(&self, project_id: i64, key_id: i64) -> Result<DeployKey, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/deploy_keys/{key_id}"
        ))
    }

    fn user_project_deploy_keys(&self, user: &str) -> Result<Vec<DeployKey>, Error> {
        self.get_paginated(&format!(
            "api/v4/users/{}/project_deploy_keys",
            user.percent_encode()
        ))
    }
}

/// Read endpoints for deploy tokens.
pub trait DeployTokenEndpoints {
    /// List all deploy tokens across the instance (admin only).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn all_deploy_tokens(&self) -> Result<Vec<DeployToken>, Error>;

    /// List a project's deploy tokens.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_deploy_tokens(&self, project_id: i64) -> Result<Vec<DeployToken>, Error>;

    /// Retrieve a single project deploy token by ID.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_deploy_token(&self, project_id: i64, token_id: i64) -> Result<DeployToken, Error>;

    /// List a group's deploy tokens.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_deploy_tokens(&self, group_id: i64) -> Result<Vec<DeployToken>, Error>;

    /// Retrieve a single group deploy token by ID.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_deploy_token(&self, group_id: i64, token_id: i64) -> Result<DeployToken, Error>;
}

impl DeployTokenEndpoints for GitlabClient {
    fn all_deploy_tokens(&self) -> Result<Vec<DeployToken>, Error> {
        self.get_paginated("api/v4/deploy_tokens")
    }

    fn project_deploy_tokens(&self, project_id: i64) -> Result<Vec<DeployToken>, Error> {
        self.get_paginated(&format!("api/v4/projects/{project_id}/deploy_tokens"))
    }

    fn project_deploy_token(&self, project_id: i64, token_id: i64) -> Result<DeployToken, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/deploy_tokens/{token_id}"
        ))
    }

    fn group_deploy_tokens(&self, group_id: i64) -> Result<Vec<DeployToken>, Error> {
        self.get_paginated(&format!("api/v4/groups/{group_id}/deploy_tokens"))
    }

    fn group_deploy_token(&self, group_id: i64, token_id: i64) -> Result<DeployToken, Error> {
        self.get(&format!(
            "api/v4/groups/{group_id}/deploy_tokens/{token_id}"
        ))
    }
}
