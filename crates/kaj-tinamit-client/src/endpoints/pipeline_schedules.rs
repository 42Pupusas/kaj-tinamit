//! Pipeline-schedule endpoints: the **Pipeline schedules** API category.
//!
//! Reads list/fetch schedules; writes create, update, delete, play, and
//! take ownership of them.

use json_bourne::ToJson;
use kaj_tinamit::PipelineSchedule;

use crate::client::GitlabClient;
use crate::error::Error;

/// Body for creating a pipeline schedule. `description`, `ref`, and `cron`
/// are required.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct CreatePipelineSchedule {
    pub description: String,
    #[bourne(rename = "ref")]
    pub ref_name: String,
    pub cron: String,
    #[bourne(skip_if_none)]
    pub cron_timezone: Option<String>,
    #[bourne(skip_if_none)]
    pub active: Option<bool>,
}

impl CreatePipelineSchedule {
    #[must_use]
    pub fn new(
        description: impl Into<String>,
        ref_name: impl Into<String>,
        cron: impl Into<String>,
    ) -> Self {
        Self {
            description: description.into(),
            ref_name: ref_name.into(),
            cron: cron.into(),
            cron_timezone: None,
            active: None,
        }
    }

    #[must_use]
    pub fn with_timezone(mut self, tz: impl Into<String>) -> Self {
        self.cron_timezone = Some(tz.into());
        self
    }

    #[must_use]
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }
}

/// Body for updating a pipeline schedule. Every field optional.
#[derive(Debug, Clone, Default, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct UpdatePipelineSchedule {
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    #[bourne(skip_if_none, rename = "ref")]
    pub ref_name: Option<String>,
    #[bourne(skip_if_none)]
    pub cron: Option<String>,
    #[bourne(skip_if_none)]
    pub cron_timezone: Option<String>,
    #[bourne(skip_if_none)]
    pub active: Option<bool>,
}

impl UpdatePipelineSchedule {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn with_ref(mut self, ref_name: impl Into<String>) -> Self {
        self.ref_name = Some(ref_name.into());
        self
    }

    #[must_use]
    pub fn with_cron(mut self, cron: impl Into<String>) -> Self {
        self.cron = Some(cron.into());
        self
    }

    #[must_use]
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }
}

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

    // --- Writes ---

    /// Create a pipeline schedule. Returns the created schedule.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_pipeline_schedule(
        &self,
        project_id: i64,
        schedule: &CreatePipelineSchedule,
    ) -> Result<PipelineSchedule, Error>;

    /// Update a pipeline schedule. Returns the updated schedule.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn update_pipeline_schedule(
        &self,
        project_id: i64,
        schedule_id: i64,
        update: &UpdatePipelineSchedule,
    ) -> Result<PipelineSchedule, Error>;

    /// Delete a pipeline schedule.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_pipeline_schedule(&self, project_id: i64, schedule_id: i64) -> Result<(), Error>;

    /// Trigger a pipeline schedule to run immediately. GitLab returns
    /// `201 Created` with no useful body.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn play_pipeline_schedule(&self, project_id: i64, schedule_id: i64) -> Result<(), Error>;

    /// Take ownership of a pipeline schedule. Returns the schedule.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn take_ownership_pipeline_schedule(
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

    fn create_pipeline_schedule(
        &self,
        project_id: i64,
        schedule: &CreatePipelineSchedule,
    ) -> Result<PipelineSchedule, Error> {
        self.post(
            &format!("api/v4/projects/{project_id}/pipeline_schedules"),
            schedule,
        )
    }

    fn update_pipeline_schedule(
        &self,
        project_id: i64,
        schedule_id: i64,
        update: &UpdatePipelineSchedule,
    ) -> Result<PipelineSchedule, Error> {
        self.put(
            &format!("api/v4/projects/{project_id}/pipeline_schedules/{schedule_id}"),
            update,
        )
    }

    fn delete_pipeline_schedule(&self, project_id: i64, schedule_id: i64) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/projects/{project_id}/pipeline_schedules/{schedule_id}"
        ))
    }

    fn play_pipeline_schedule(&self, project_id: i64, schedule_id: i64) -> Result<(), Error> {
        self.post_discard(&format!(
            "api/v4/projects/{project_id}/pipeline_schedules/{schedule_id}/play"
        ))
    }

    fn take_ownership_pipeline_schedule(
        &self,
        project_id: i64,
        schedule_id: i64,
    ) -> Result<PipelineSchedule, Error> {
        self.post_no_body(&format!(
            "api/v4/projects/{project_id}/pipeline_schedules/{schedule_id}/take_ownership"
        ))
    }
}
