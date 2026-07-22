//! Endpoints for the **Environments** and **Deployments** API categories.
//!
//! Reads list/fetch environments and deployments; writes create/update/stop/
//! delete environments and create/update deployments.

use json_bourne::ToJson;
use kaj_tinamit::{Deployment, Environment, MergeRequest};

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

/// Body for creating a project environment. Only `name` is required.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct CreateEnvironment {
    pub name: String,
    #[bourne(skip_if_none)]
    pub external_url: Option<String>,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    #[bourne(skip_if_none)]
    pub tier: Option<String>,
}

impl CreateEnvironment {
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            external_url: None,
            description: None,
            tier: None,
        }
    }

    #[must_use]
    pub fn with_external_url(mut self, url: impl Into<String>) -> Self {
        self.external_url = Some(url.into());
        self
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Deployment tier (`production`, `staging`, `testing`, `development`,
    /// `other`).
    #[must_use]
    pub fn with_tier(mut self, tier: impl Into<String>) -> Self {
        self.tier = Some(tier.into());
        self
    }
}

/// Body for updating a project environment. All fields optional; unset
/// fields are omitted so a partial update touches only what you set.
#[derive(Debug, Clone, Default, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct UpdateEnvironment {
    #[bourne(skip_if_none)]
    pub external_url: Option<String>,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    #[bourne(skip_if_none)]
    pub tier: Option<String>,
}

impl UpdateEnvironment {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_external_url(mut self, url: impl Into<String>) -> Self {
        self.external_url = Some(url.into());
        self
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn with_tier(mut self, tier: impl Into<String>) -> Self {
        self.tier = Some(tier.into());
        self
    }
}

/// Body for creating a deployment. `environment`, `sha`, `ref`, and
/// `status` are required (status e.g. `running`, `success`, `failed`,
/// `canceled`).
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct CreateDeployment {
    pub environment: String,
    pub sha: String,
    #[bourne(rename = "ref")]
    pub ref_name: String,
    pub tag: bool,
    pub status: String,
}

impl CreateDeployment {
    #[must_use]
    pub fn new(
        environment: impl Into<String>,
        sha: impl Into<String>,
        ref_name: impl Into<String>,
        status: impl Into<String>,
    ) -> Self {
        Self {
            environment: environment.into(),
            sha: sha.into(),
            ref_name: ref_name.into(),
            tag: false,
            status: status.into(),
        }
    }

    /// Mark the `ref` as a tag rather than a branch.
    #[must_use]
    pub fn tagged(mut self, tag: bool) -> Self {
        self.tag = tag;
        self
    }
}

/// Body for updating a deployment's status.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct UpdateDeployment {
    pub status: String,
}

impl UpdateDeployment {
    #[must_use]
    pub fn new(status: impl Into<String>) -> Self {
        Self {
            status: status.into(),
        }
    }
}

/// Read endpoints for project environments.
pub trait EnvironmentEndpoints {
    /// List a project's environments.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn environments(&self, project_id: i64) -> Result<Vec<Environment>, Error>;

    /// List environments filtered by state (`available`, `stopping`,
    /// `stopped`).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn environments_by_state(
        &self,
        project_id: i64,
        state: &str,
    ) -> Result<Vec<Environment>, Error>;

    /// Retrieve a single environment by ID.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn environment(&self, project_id: i64, environment_id: i64) -> Result<Environment, Error>;

    // --- Writes ---

    /// Create a project environment. Returns the created environment.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_environment(
        &self,
        project_id: i64,
        environment: &CreateEnvironment,
    ) -> Result<Environment, Error>;

    /// Update a project environment. Returns the updated environment.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn update_environment(
        &self,
        project_id: i64,
        environment_id: i64,
        update: &UpdateEnvironment,
    ) -> Result<Environment, Error>;

    /// Stop an environment. Returns the stopped environment.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn stop_environment(&self, project_id: i64, environment_id: i64) -> Result<Environment, Error>;

    /// Stop all stale environments not updated since `before` (an ISO-8601
    /// timestamp, at least 10 days in the past).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn stop_stale_environments(&self, project_id: i64, before: &str) -> Result<(), Error>;

    /// Delete a (stopped) project environment.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_environment(&self, project_id: i64, environment_id: i64) -> Result<(), Error>;

    /// Delete every environment in the `stopped` state for a project.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_stopped_environments(&self, project_id: i64) -> Result<(), Error>;
}

impl EnvironmentEndpoints for GitlabClient {
    fn environments(&self, project_id: i64) -> Result<Vec<Environment>, Error> {
        self.get_paginated(&format!("api/v4/projects/{project_id}/environments"))
    }

    fn environments_by_state(
        &self,
        project_id: i64,
        state: &str,
    ) -> Result<Vec<Environment>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/environments?states={}",
            state.percent_encode()
        ))
    }

    fn environment(&self, project_id: i64, environment_id: i64) -> Result<Environment, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/environments/{environment_id}"
        ))
    }

    fn create_environment(
        &self,
        project_id: i64,
        environment: &CreateEnvironment,
    ) -> Result<Environment, Error> {
        self.post(
            &format!("api/v4/projects/{project_id}/environments"),
            environment,
        )
    }

    fn update_environment(
        &self,
        project_id: i64,
        environment_id: i64,
        update: &UpdateEnvironment,
    ) -> Result<Environment, Error> {
        self.put(
            &format!("api/v4/projects/{project_id}/environments/{environment_id}"),
            update,
        )
    }

    fn stop_environment(&self, project_id: i64, environment_id: i64) -> Result<Environment, Error> {
        self.post_no_body(&format!(
            "api/v4/projects/{project_id}/environments/{environment_id}/stop"
        ))
    }

    fn stop_stale_environments(&self, project_id: i64, before: &str) -> Result<(), Error> {
        self.post_discard(&format!(
            "api/v4/projects/{project_id}/environments/stop_stale?before={}",
            before.percent_encode()
        ))
    }

    fn delete_environment(&self, project_id: i64, environment_id: i64) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/projects/{project_id}/environments/{environment_id}"
        ))
    }

    fn delete_stopped_environments(&self, project_id: i64) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/projects/{project_id}/environments/review_apps"
        ))
    }
}

/// Read endpoints for project deployments.
pub trait DeploymentEndpoints {
    /// List a project's deployments.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn deployments(&self, project_id: i64) -> Result<Vec<Deployment>, Error>;

    /// List deployments to a specific environment.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn environment_deployments(
        &self,
        project_id: i64,
        environment: &str,
    ) -> Result<Vec<Deployment>, Error>;

    /// Retrieve a single deployment by ID.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn deployment(&self, project_id: i64, deployment_id: i64) -> Result<Deployment, Error>;

    /// List the merge requests shipped with a given deployment.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn deployment_merge_requests(
        &self,
        project_id: i64,
        deployment_id: i64,
    ) -> Result<Vec<MergeRequest>, Error>;

    // --- Writes ---

    /// Create a deployment. Returns the created deployment.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_deployment(
        &self,
        project_id: i64,
        deployment: &CreateDeployment,
    ) -> Result<Deployment, Error>;

    /// Update a deployment's status. Returns the updated deployment.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn update_deployment(
        &self,
        project_id: i64,
        deployment_id: i64,
        update: &UpdateDeployment,
    ) -> Result<Deployment, Error>;

    /// Delete a specific deployment (only permitted for the oldest
    /// deployments once they are no longer referenced).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_deployment(&self, project_id: i64, deployment_id: i64) -> Result<(), Error>;
}

impl DeploymentEndpoints for GitlabClient {
    fn deployments(&self, project_id: i64) -> Result<Vec<Deployment>, Error> {
        self.get_paginated(&format!("api/v4/projects/{project_id}/deployments"))
    }

    fn environment_deployments(
        &self,
        project_id: i64,
        environment: &str,
    ) -> Result<Vec<Deployment>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/deployments?environment={}",
            environment.percent_encode()
        ))
    }

    fn deployment(&self, project_id: i64, deployment_id: i64) -> Result<Deployment, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/deployments/{deployment_id}"
        ))
    }

    fn deployment_merge_requests(
        &self,
        project_id: i64,
        deployment_id: i64,
    ) -> Result<Vec<MergeRequest>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/deployments/{deployment_id}/merge_requests"
        ))
    }

    fn create_deployment(
        &self,
        project_id: i64,
        deployment: &CreateDeployment,
    ) -> Result<Deployment, Error> {
        self.post(
            &format!("api/v4/projects/{project_id}/deployments"),
            deployment,
        )
    }

    fn update_deployment(
        &self,
        project_id: i64,
        deployment_id: i64,
        update: &UpdateDeployment,
    ) -> Result<Deployment, Error> {
        self.put(
            &format!("api/v4/projects/{project_id}/deployments/{deployment_id}"),
            update,
        )
    }

    fn delete_deployment(&self, project_id: i64, deployment_id: i64) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/projects/{project_id}/deployments/{deployment_id}"
        ))
    }
}
