//! CI/CD endpoints: the **Pipelines** and **Jobs** API categories.
//!
//! Reads list pipelines/jobs and their variables/traces; writes create a
//! pipeline, retry/cancel/delete one, and play/retry/cancel/erase a job.

use gitlab_model::{Job, PipelineDetail, PipelineSummary, PipelineVariable};
use json_bourne::ToJson;

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

/// A CI/CD variable passed when triggering a pipeline.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct PipelineInput {
    pub key: String,
    pub value: String,
    /// `env_var` (default) or `file`.
    #[bourne(skip_if_none)]
    pub variable_type: Option<String>,
}

impl PipelineInput {
    /// A simple `env_var` key/value input.
    #[must_use]
    pub fn env(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            variable_type: None,
        }
    }

    /// A `file`-type input (value becomes the file contents).
    #[must_use]
    pub fn file(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            variable_type: Some("file".to_string()),
        }
    }
}

/// Body for creating a pipeline (`POST /projects/:id/pipeline`).
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
struct CreatePipelineBody {
    #[bourne(rename = "ref")]
    ref_name: String,
    #[bourne(skip_if_none)]
    variables: Option<Vec<PipelineInput>>,
}

/// Filters for [`PipelineEndpoints::pipelines`]. Empty by default; each
/// setter narrows the listing. Values are percent-encoded.
#[derive(Debug, Default, Clone)]
pub struct PipelineQuery {
    params: Vec<(&'static str, String)>,
}

impl PipelineQuery {
    /// A new, empty query.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by branch or tag.
    #[must_use]
    pub fn with_ref(mut self, ref_name: &str) -> Self {
        self.params.push(("ref", ref_name.to_string()));
        self
    }

    /// Filter by status (e.g. `success`, `failed`, `running`).
    #[must_use]
    pub fn with_status(mut self, status: &str) -> Self {
        self.params.push(("status", status.to_string()));
        self
    }

    /// Filter by scope (`running`, `pending`, `finished`, `branches`,
    /// `tags`).
    #[must_use]
    pub fn with_scope(mut self, scope: &str) -> Self {
        self.params.push(("scope", scope.to_string()));
        self
    }

    /// Filter by source (e.g. `push`, `web`, `schedule`).
    #[must_use]
    pub fn with_source(mut self, source: &str) -> Self {
        self.params.push(("source", source.to_string()));
        self
    }

    /// Filter by commit SHA.
    #[must_use]
    pub fn with_sha(mut self, sha: &str) -> Self {
        self.params.push(("sha", sha.to_string()));
        self
    }

    /// Filter by triggering username.
    #[must_use]
    pub fn with_username(mut self, username: &str) -> Self {
        self.params.push(("username", username.to_string()));
        self
    }

    /// Render as a URL query string (without leading `?`); empty when no
    /// filters are set.
    fn to_query_string(&self) -> String {
        self.params
            .iter()
            .map(|(k, v)| format!("{k}={}", v.percent_encode()))
            .collect::<Vec<_>>()
            .join("&")
    }
}

/// Read endpoints for CI/CD pipelines.
pub trait PipelineEndpoints {
    /// List a project's pipelines, filtered by `query`.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn pipelines(
        &self,
        project_id: i64,
        query: &PipelineQuery,
    ) -> Result<Vec<PipelineSummary>, Error>;

    /// Retrieve a single pipeline by ID.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn pipeline(&self, project_id: i64, pipeline_id: i64) -> Result<PipelineDetail, Error>;

    /// Retrieve the latest pipeline for a ref (default branch when `None`).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn latest_pipeline(
        &self,
        project_id: i64,
        ref_name: Option<&str>,
    ) -> Result<PipelineDetail, Error>;

    /// Retrieve a pipeline's CI/CD variables.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn pipeline_variables(
        &self,
        project_id: i64,
        pipeline_id: i64,
    ) -> Result<Vec<PipelineVariable>, Error>;

    // --- Writes ---

    /// Create (trigger) a new pipeline on `ref_name`, optionally passing
    /// CI/CD `variables`. Returns the created pipeline.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_pipeline(
        &self,
        project_id: i64,
        ref_name: &str,
        variables: Vec<PipelineInput>,
    ) -> Result<PipelineDetail, Error>;

    /// Retry failed/canceled jobs in a pipeline. Returns the pipeline.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn retry_pipeline(&self, project_id: i64, pipeline_id: i64) -> Result<PipelineDetail, Error>;

    /// Cancel a pipeline's jobs. Returns the pipeline.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn cancel_pipeline(&self, project_id: i64, pipeline_id: i64) -> Result<PipelineDetail, Error>;

    /// Delete a pipeline and its jobs/artifacts.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_pipeline(&self, project_id: i64, pipeline_id: i64) -> Result<(), Error>;
}

impl PipelineEndpoints for GitlabClient {
    fn pipelines(
        &self,
        project_id: i64,
        query: &PipelineQuery,
    ) -> Result<Vec<PipelineSummary>, Error> {
        let mut url = format!("api/v4/projects/{project_id}/pipelines");
        let qs = query.to_query_string();
        if !qs.is_empty() {
            url.push('?');
            url.push_str(&qs);
        }
        self.get_paginated(&url)
    }

    fn pipeline(&self, project_id: i64, pipeline_id: i64) -> Result<PipelineDetail, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/pipelines/{pipeline_id}"
        ))
    }

    fn latest_pipeline(
        &self,
        project_id: i64,
        ref_name: Option<&str>,
    ) -> Result<PipelineDetail, Error> {
        let mut url = format!("api/v4/projects/{project_id}/pipelines/latest");
        if let Some(r) = ref_name {
            url.push_str(&format!("?ref={}", r.percent_encode()));
        }
        self.get(&url)
    }

    fn pipeline_variables(
        &self,
        project_id: i64,
        pipeline_id: i64,
    ) -> Result<Vec<PipelineVariable>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/pipelines/{pipeline_id}/variables"
        ))
    }

    fn create_pipeline(
        &self,
        project_id: i64,
        ref_name: &str,
        variables: Vec<PipelineInput>,
    ) -> Result<PipelineDetail, Error> {
        let body = CreatePipelineBody {
            ref_name: ref_name.to_string(),
            variables: if variables.is_empty() {
                None
            } else {
                Some(variables)
            },
        };
        self.post(&format!("api/v4/projects/{project_id}/pipeline"), &body)
    }

    fn retry_pipeline(&self, project_id: i64, pipeline_id: i64) -> Result<PipelineDetail, Error> {
        self.post_no_body(&format!(
            "api/v4/projects/{project_id}/pipelines/{pipeline_id}/retry"
        ))
    }

    fn cancel_pipeline(&self, project_id: i64, pipeline_id: i64) -> Result<PipelineDetail, Error> {
        self.post_no_body(&format!(
            "api/v4/projects/{project_id}/pipelines/{pipeline_id}/cancel"
        ))
    }

    fn delete_pipeline(&self, project_id: i64, pipeline_id: i64) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/projects/{project_id}/pipelines/{pipeline_id}"
        ))
    }
}

/// Read endpoints for CI/CD jobs.
pub trait JobEndpoints {
    /// List all jobs for a project.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn jobs(&self, project_id: i64) -> Result<Vec<Job>, Error>;

    /// List all jobs for a specific pipeline.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn pipeline_jobs(&self, project_id: i64, pipeline_id: i64) -> Result<Vec<Job>, Error>;

    /// List all trigger (bridge) jobs for a specific pipeline.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn pipeline_trigger_jobs(&self, project_id: i64, pipeline_id: i64) -> Result<Vec<Job>, Error>;

    /// Retrieve a single job by ID.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn job(&self, project_id: i64, job_id: i64) -> Result<Job, Error>;

    /// Retrieve a job's log (trace) as raw text.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn job_trace(&self, project_id: i64, job_id: i64) -> Result<String, Error>;

    /// Retrieve the raw text content of a single artifact file within a job's
    /// archive, at `artifact_path`.
    ///
    /// Binary artifacts are lossily UTF-8 decoded by the transport; prefer
    /// this for text artifacts (reports, logs, coverage).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn job_artifact_file(
        &self,
        project_id: i64,
        job_id: i64,
        artifact_path: &str,
    ) -> Result<String, Error>;

    // --- Writes ---

    /// Play (run) a manual job. Returns the job.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn play_job(&self, project_id: i64, job_id: i64) -> Result<Job, Error>;

    /// Retry a failed/canceled job. Returns the new job.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn retry_job(&self, project_id: i64, job_id: i64) -> Result<Job, Error>;

    /// Cancel a running job. Returns the job.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn cancel_job(&self, project_id: i64, job_id: i64) -> Result<Job, Error>;

    /// Erase a job (remove artifacts and trace). Returns the job.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn erase_job(&self, project_id: i64, job_id: i64) -> Result<Job, Error>;
}

impl JobEndpoints for GitlabClient {
    fn jobs(&self, project_id: i64) -> Result<Vec<Job>, Error> {
        self.get_paginated(&format!("api/v4/projects/{project_id}/jobs"))
    }

    fn pipeline_jobs(&self, project_id: i64, pipeline_id: i64) -> Result<Vec<Job>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/pipelines/{pipeline_id}/jobs"
        ))
    }

    fn pipeline_trigger_jobs(&self, project_id: i64, pipeline_id: i64) -> Result<Vec<Job>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/pipelines/{pipeline_id}/trigger_jobs"
        ))
    }

    fn job(&self, project_id: i64, job_id: i64) -> Result<Job, Error> {
        self.get(&format!("api/v4/projects/{project_id}/jobs/{job_id}"))
    }

    fn job_trace(&self, project_id: i64, job_id: i64) -> Result<String, Error> {
        self.get_raw(&format!("api/v4/projects/{project_id}/jobs/{job_id}/trace"))
    }

    fn job_artifact_file(
        &self,
        project_id: i64,
        job_id: i64,
        artifact_path: &str,
    ) -> Result<String, Error> {
        self.get_raw(&format!(
            "api/v4/projects/{project_id}/jobs/{job_id}/artifacts/{}",
            artifact_path.percent_encode()
        ))
    }

    fn play_job(&self, project_id: i64, job_id: i64) -> Result<Job, Error> {
        self.post_no_body(&format!("api/v4/projects/{project_id}/jobs/{job_id}/play"))
    }

    fn retry_job(&self, project_id: i64, job_id: i64) -> Result<Job, Error> {
        self.post_no_body(&format!("api/v4/projects/{project_id}/jobs/{job_id}/retry"))
    }

    fn cancel_job(&self, project_id: i64, job_id: i64) -> Result<Job, Error> {
        self.post_no_body(&format!(
            "api/v4/projects/{project_id}/jobs/{job_id}/cancel"
        ))
    }

    fn erase_job(&self, project_id: i64, job_id: i64) -> Result<Job, Error> {
        self.post_no_body(&format!("api/v4/projects/{project_id}/jobs/{job_id}/erase"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_empty_by_default() {
        assert_eq!(PipelineQuery::new().to_query_string(), "");
    }

    #[test]
    fn query_combines_and_encodes() {
        let q = PipelineQuery::new()
            .with_ref("feature/x")
            .with_status("success");
        assert_eq!(q.to_query_string(), "ref=feature%2Fx&status=success");
    }
}
