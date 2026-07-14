//! Read endpoints for the **Environments** and **Deployments** API
//! categories.
//!
//! All read-only (GET). Create/update/stop/delete/approve are intentionally
//! omitted.

use gitlab_model::{Deployment, Environment, MergeRequest};

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

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
}
