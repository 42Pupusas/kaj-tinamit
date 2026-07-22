//! Endpoints for the **Deploy keys** and **Deploy tokens** API categories.
//!
//! Reads list/fetch keys and tokens; writes add/update/enable/delete keys
//! and create/delete tokens.

use json_bourne::ToJson;
use kaj_tinamit::{CreatedDeployToken, DeployKey, DeployToken};

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

/// Body for adding a deploy key to a project. `title` and `key` (the public
/// key material) are required.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct CreateDeployKey {
    pub title: String,
    pub key: String,
    #[bourne(skip_if_none)]
    pub can_push: Option<bool>,
    #[bourne(skip_if_none)]
    pub expires_at: Option<String>,
}

impl CreateDeployKey {
    #[must_use]
    pub fn new(title: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            key: key.into(),
            can_push: None,
            expires_at: None,
        }
    }

    #[must_use]
    pub fn can_push(mut self, can_push: bool) -> Self {
        self.can_push = Some(can_push);
        self
    }

    #[must_use]
    pub fn with_expiry(mut self, expires_at: impl Into<String>) -> Self {
        self.expires_at = Some(expires_at.into());
        self
    }
}

/// Body for creating a deploy token (project or group). `name` and `scopes`
/// are required (scopes e.g. `read_repository`, `read_registry`).
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct CreateDeployToken {
    pub name: String,
    pub scopes: Vec<String>,
    #[bourne(skip_if_none)]
    pub expires_at: Option<String>,
    #[bourne(skip_if_none)]
    pub username: Option<String>,
}

impl CreateDeployToken {
    #[must_use]
    pub fn new(name: impl Into<String>, scopes: Vec<String>) -> Self {
        Self {
            name: name.into(),
            scopes,
            expires_at: None,
            username: None,
        }
    }

    #[must_use]
    pub fn with_expiry(mut self, expires_at: impl Into<String>) -> Self {
        self.expires_at = Some(expires_at.into());
        self
    }

    #[must_use]
    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }
}

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

    // --- Writes ---

    /// Add a deploy key to a project. Returns the created key.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn add_project_deploy_key(
        &self,
        project_id: i64,
        key: &CreateDeployKey,
    ) -> Result<DeployKey, Error>;

    /// Enable a deploy key (already defined elsewhere) for a project.
    /// Returns the enabled key.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn enable_project_deploy_key(&self, project_id: i64, key_id: i64) -> Result<DeployKey, Error>;

    /// Delete (disable) a project deploy key.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_project_deploy_key(&self, project_id: i64, key_id: i64) -> Result<(), Error>;
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

    fn add_project_deploy_key(
        &self,
        project_id: i64,
        key: &CreateDeployKey,
    ) -> Result<DeployKey, Error> {
        self.post(&format!("api/v4/projects/{project_id}/deploy_keys"), key)
    }

    fn enable_project_deploy_key(&self, project_id: i64, key_id: i64) -> Result<DeployKey, Error> {
        self.post_no_body(&format!(
            "api/v4/projects/{project_id}/deploy_keys/{key_id}/enable"
        ))
    }

    fn delete_project_deploy_key(&self, project_id: i64, key_id: i64) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/projects/{project_id}/deploy_keys/{key_id}"
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

    // --- Writes ---

    /// Create a project deploy token. Returns the token **including its
    /// secret value** (available only at creation).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_project_deploy_token(
        &self,
        project_id: i64,
        token: &CreateDeployToken,
    ) -> Result<CreatedDeployToken, Error>;

    /// Delete a project deploy token.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_project_deploy_token(&self, project_id: i64, token_id: i64) -> Result<(), Error>;

    /// Create a group deploy token. Returns the token including its secret.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_group_deploy_token(
        &self,
        group_id: i64,
        token: &CreateDeployToken,
    ) -> Result<CreatedDeployToken, Error>;

    /// Delete a group deploy token.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_group_deploy_token(&self, group_id: i64, token_id: i64) -> Result<(), Error>;
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

    fn create_project_deploy_token(
        &self,
        project_id: i64,
        token: &CreateDeployToken,
    ) -> Result<CreatedDeployToken, Error> {
        self.post(
            &format!("api/v4/projects/{project_id}/deploy_tokens"),
            token,
        )
    }

    fn delete_project_deploy_token(&self, project_id: i64, token_id: i64) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/projects/{project_id}/deploy_tokens/{token_id}"
        ))
    }

    fn create_group_deploy_token(
        &self,
        group_id: i64,
        token: &CreateDeployToken,
    ) -> Result<CreatedDeployToken, Error> {
        self.post(&format!("api/v4/groups/{group_id}/deploy_tokens"), token)
    }

    fn delete_group_deploy_token(&self, group_id: i64, token_id: i64) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/groups/{group_id}/deploy_tokens/{token_id}"
        ))
    }
}
