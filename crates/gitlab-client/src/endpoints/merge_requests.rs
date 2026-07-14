//! Merge-request read endpoints and the [`MergeRequestQuery`] builder.

use gitlab_model::{
    Discussion, MergeRequest, MergeRequestApprovals, MergeRequestChanges, MergeRequestState,
    Pipeline, TimeStats,
};

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

/// Merge-request read endpoints (and the issue notes/time-stats that share
/// the same discussion machinery).
pub trait MergeRequestEndpoints {
    /// Get a single merge request by project ID and MR IID (not ID!).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn merge_request(&self, project_id: i64, mr_iid: i64) -> Result<MergeRequest, Error>;

    /// List merge requests for a project with optional filters.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn merge_requests(
        &self,
        project_id: i64,
        query: &MergeRequestQuery,
    ) -> Result<Vec<MergeRequest>, Error>;

    /// Get discussions (comment threads) on a merge request.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn merge_request_discussions(
        &self,
        project_id: i64,
        mr_iid: i64,
    ) -> Result<Vec<Discussion>, Error>;

    /// Get changes (diffs) for a merge request.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn merge_request_changes(
        &self,
        project_id: i64,
        mr_iid: i64,
    ) -> Result<MergeRequestChanges, Error>;

    /// Get pipelines for a merge request.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn merge_request_pipelines(&self, project_id: i64, mr_iid: i64)
    -> Result<Vec<Pipeline>, Error>;

    /// Get approval information for a merge request.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn merge_request_approvals(
        &self,
        project_id: i64,
        mr_iid: i64,
    ) -> Result<MergeRequestApprovals, Error>;

    /// Get time-tracking statistics for an issue.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn issue_time_stats(&self, project_id: i64, issue_iid: i64) -> Result<TimeStats, Error>;

    /// Get time-tracking statistics for a merge request.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn merge_request_time_stats(&self, project_id: i64, mr_iid: i64) -> Result<TimeStats, Error>;
}

impl MergeRequestEndpoints for GitlabClient {
    fn merge_request(&self, project_id: i64, mr_iid: i64) -> Result<MergeRequest, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/merge_requests/{mr_iid}"
        ))
    }

    fn merge_requests(
        &self,
        project_id: i64,
        query: &MergeRequestQuery,
    ) -> Result<Vec<MergeRequest>, Error> {
        let mut url = format!("api/v4/projects/{project_id}/merge_requests");
        let params = query.build_params();
        if !params.is_empty() {
            url.push('?');
            url.push_str(&params);
        }
        self.get_paginated(&url)
    }

    fn merge_request_discussions(
        &self,
        project_id: i64,
        mr_iid: i64,
    ) -> Result<Vec<Discussion>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/merge_requests/{mr_iid}/discussions"
        ))
    }

    fn merge_request_changes(
        &self,
        project_id: i64,
        mr_iid: i64,
    ) -> Result<MergeRequestChanges, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/merge_requests/{mr_iid}/changes"
        ))
    }

    fn merge_request_pipelines(
        &self,
        project_id: i64,
        mr_iid: i64,
    ) -> Result<Vec<Pipeline>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/merge_requests/{mr_iid}/pipelines"
        ))
    }

    fn merge_request_approvals(
        &self,
        project_id: i64,
        mr_iid: i64,
    ) -> Result<MergeRequestApprovals, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/merge_requests/{mr_iid}/approvals"
        ))
    }

    fn issue_time_stats(&self, project_id: i64, issue_iid: i64) -> Result<TimeStats, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/issues/{issue_iid}/time_stats"
        ))
    }

    fn merge_request_time_stats(&self, project_id: i64, mr_iid: i64) -> Result<TimeStats, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/merge_requests/{mr_iid}/time_stats"
        ))
    }
}

/// Query builder for listing merge requests.
#[derive(Debug, Default, Clone)]
pub struct MergeRequestQuery {
    state: Option<MergeRequestState>,
    source_branch: Option<String>,
    target_branch: Option<String>,
    search: Option<String>,
    labels: Option<Vec<String>>,
    author_id: Option<i64>,
    assignee_id: Option<i64>,
}

impl MergeRequestQuery {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_state(mut self, state: MergeRequestState) -> Self {
        self.state = Some(state);
        self
    }

    #[must_use]
    pub fn with_source_branch(mut self, branch: impl Into<String>) -> Self {
        self.source_branch = Some(branch.into());
        self
    }

    #[must_use]
    pub fn with_target_branch(mut self, branch: impl Into<String>) -> Self {
        self.target_branch = Some(branch.into());
        self
    }

    #[must_use]
    pub fn with_search(mut self, query: impl Into<String>) -> Self {
        self.search = Some(query.into());
        self
    }

    #[must_use]
    pub fn with_labels(mut self, labels: Vec<String>) -> Self {
        self.labels = Some(labels);
        self
    }

    #[must_use]
    pub fn with_author_id(mut self, author_id: i64) -> Self {
        self.author_id = Some(author_id);
        self
    }

    #[must_use]
    pub fn with_assignee_id(mut self, assignee_id: i64) -> Self {
        self.assignee_id = Some(assignee_id);
        self
    }

    fn build_params(&self) -> String {
        let mut params = Vec::new();

        if let Some(state) = &self.state {
            params.push(format!("state={}", state.as_ref()));
        }
        if let Some(branch) = &self.source_branch {
            params.push(format!("source_branch={}", branch.percent_encode()));
        }
        if let Some(branch) = &self.target_branch {
            params.push(format!("target_branch={}", branch.percent_encode()));
        }
        if let Some(search) = &self.search {
            params.push(format!("search={}", search.percent_encode()));
        }
        if let Some(labels) = &self.labels {
            let joined = labels
                .iter()
                .map(|l| l.percent_encode())
                .collect::<Vec<_>>()
                .join(",");
            params.push(format!("labels={joined}"));
        }
        if let Some(author_id) = self.author_id {
            params.push(format!("author_id={author_id}"));
        }
        if let Some(assignee_id) = self.assignee_id {
            params.push(format!("assignee_id={assignee_id}"));
        }

        params.join("&")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_empty_by_default() {
        assert_eq!(MergeRequestQuery::new().build_params(), "");
    }

    #[test]
    fn params_combine_and_encode() {
        let p = MergeRequestQuery::new()
            .with_state(MergeRequestState::Merged)
            .with_target_branch("release/1.0")
            .with_search("fix bug")
            .build_params();
        assert!(p.contains("state=merged"));
        assert!(p.contains("target_branch=release%2F1.0"));
        assert!(p.contains("search=fix%20bug"));
    }
}
