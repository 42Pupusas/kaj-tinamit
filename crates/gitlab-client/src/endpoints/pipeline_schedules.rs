//! Pipeline-schedule read endpoints: the **Pipeline schedules** API category.
//!
//! All read-only (GET). Create/update/delete/play/take-ownership are
//! intentionally omitted.

use gitlab_model::PipelineSchedule;

use crate::client::GitlabClient;
use crate::error::Error;

/// Read endpoints for CI/CD pipeline schedules.
pub trait PipelineScheduleEndpoints {
    /// List a project's pipeline schedules.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn pipeline_schedules(&self, project_id: i64) -> Result<Vec<PipelineSchedule>, Error>;

    /// Retrieve a single pipeline schedule by ID (includes `last_pipeline`).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn pipeline_schedule(
        &self,
        project_id: i64,
        schedule_id: i64,
    ) -> Result<PipelineSchedule, Error>;
}

impl PipelineScheduleEndpoints for GitlabClient {
    fn pipeline_schedules(&self, project_id: i64) -> Result<Vec<PipelineSchedule>, Error> {
        self.get_paginated(&format!("api/v4/projects/{project_id}/pipeline_schedules"))
    }

    fn pipeline_schedule(
        &self,
        project_id: i64,
        schedule_id: i64,
    ) -> Result<PipelineSchedule, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/pipeline_schedules/{schedule_id}"
        ))
    }
}
