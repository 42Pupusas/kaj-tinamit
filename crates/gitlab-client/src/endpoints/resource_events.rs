//! Resource-event read endpoints: the **Resource label events**, **Resource
//! state events**, and **Resource milestone events** API categories for
//! issues and merge requests.
//!
//! These are read-only by nature — GitLab emits them as a side effect of
//! other actions. They're the audit trail behind flow/cycle-time metrics.

use gitlab_model::{ResourceLabelEvent, ResourceMilestoneEvent, ResourceStateEvent};

use crate::client::GitlabClient;
use crate::error::Error;

/// Read endpoints for resource change events on issues and merge requests.
pub trait ResourceEventEndpoints {
    /// List label add/remove events for an issue.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn issue_label_events(
        &self,
        project_id: i64,
        issue_iid: i64,
    ) -> Result<Vec<ResourceLabelEvent>, Error>;

    /// List state-change events (open/close/reopen) for an issue.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn issue_state_events(
        &self,
        project_id: i64,
        issue_iid: i64,
    ) -> Result<Vec<ResourceStateEvent>, Error>;

    /// List milestone add/remove events for an issue.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn issue_milestone_events(
        &self,
        project_id: i64,
        issue_iid: i64,
    ) -> Result<Vec<ResourceMilestoneEvent>, Error>;

    /// List label add/remove events for a merge request.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn merge_request_label_events(
        &self,
        project_id: i64,
        mr_iid: i64,
    ) -> Result<Vec<ResourceLabelEvent>, Error>;

    /// List state-change events (open/close/reopen/merge) for a merge
    /// request.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn merge_request_state_events(
        &self,
        project_id: i64,
        mr_iid: i64,
    ) -> Result<Vec<ResourceStateEvent>, Error>;

    /// List milestone add/remove events for a merge request.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn merge_request_milestone_events(
        &self,
        project_id: i64,
        mr_iid: i64,
    ) -> Result<Vec<ResourceMilestoneEvent>, Error>;
}

impl ResourceEventEndpoints for GitlabClient {
    fn issue_label_events(
        &self,
        project_id: i64,
        issue_iid: i64,
    ) -> Result<Vec<ResourceLabelEvent>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/issues/{issue_iid}/resource_label_events"
        ))
    }

    fn issue_state_events(
        &self,
        project_id: i64,
        issue_iid: i64,
    ) -> Result<Vec<ResourceStateEvent>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/issues/{issue_iid}/resource_state_events"
        ))
    }

    fn issue_milestone_events(
        &self,
        project_id: i64,
        issue_iid: i64,
    ) -> Result<Vec<ResourceMilestoneEvent>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/issues/{issue_iid}/resource_milestone_events"
        ))
    }

    fn merge_request_label_events(
        &self,
        project_id: i64,
        mr_iid: i64,
    ) -> Result<Vec<ResourceLabelEvent>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/merge_requests/{mr_iid}/resource_label_events"
        ))
    }

    fn merge_request_state_events(
        &self,
        project_id: i64,
        mr_iid: i64,
    ) -> Result<Vec<ResourceStateEvent>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/merge_requests/{mr_iid}/resource_state_events"
        ))
    }

    fn merge_request_milestone_events(
        &self,
        project_id: i64,
        mr_iid: i64,
    ) -> Result<Vec<ResourceMilestoneEvent>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/merge_requests/{mr_iid}/resource_milestone_events"
        ))
    }
}
