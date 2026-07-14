//! Merge-request read endpoints and the [`MergeRequestQuery`] builder.

use gitlab_model::{
    Discussion, MergeRequest, MergeRequestApprovals, MergeRequestChanges, MergeRequestState,
    Pipeline, TimeStats,
};
use json_bourne::ToJson;

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

/// Body for creating a merge request
/// (`POST /projects/:id/merge_requests`). `source_branch`,
/// `target_branch`, and `title` are required; the rest are omitted when
/// unset.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct CreateMergeRequest {
    pub source_branch: String,
    pub target_branch: String,
    pub title: String,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    #[bourne(skip_if_none)]
    pub assignee_ids: Option<Vec<i64>>,
    #[bourne(skip_if_none)]
    pub reviewer_ids: Option<Vec<i64>>,
    #[bourne(skip_if_none)]
    pub labels: Option<String>,
    #[bourne(skip_if_none)]
    pub milestone_id: Option<i64>,
    #[bourne(skip_if_none)]
    pub remove_source_branch: Option<bool>,
    #[bourne(skip_if_none)]
    pub squash: Option<bool>,
}

impl CreateMergeRequest {
    #[must_use]
    pub fn new(
        source_branch: impl Into<String>,
        target_branch: impl Into<String>,
        title: impl Into<String>,
    ) -> Self {
        Self {
            source_branch: source_branch.into(),
            target_branch: target_branch.into(),
            title: title.into(),
            description: None,
            assignee_ids: None,
            reviewer_ids: None,
            labels: None,
            milestone_id: None,
            remove_source_branch: None,
            squash: None,
        }
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn with_assignees(mut self, assignee_ids: Vec<i64>) -> Self {
        self.assignee_ids = Some(assignee_ids);
        self
    }

    #[must_use]
    pub fn with_reviewers(mut self, reviewer_ids: Vec<i64>) -> Self {
        self.reviewer_ids = Some(reviewer_ids);
        self
    }

    #[must_use]
    pub fn with_labels(mut self, labels: &[&str]) -> Self {
        self.labels = Some(labels.join(","));
        self
    }

    #[must_use]
    pub fn with_milestone(mut self, milestone_id: i64) -> Self {
        self.milestone_id = Some(milestone_id);
        self
    }

    #[must_use]
    pub fn remove_source_branch(mut self, remove: bool) -> Self {
        self.remove_source_branch = Some(remove);
        self
    }

    #[must_use]
    pub fn squash(mut self, squash: bool) -> Self {
        self.squash = Some(squash);
        self
    }
}

/// Body for updating a merge request
/// (`PUT /projects/:id/merge_requests/:iid`). Every field optional and
/// omitted when unset. `state_event` drives close/reopen.
#[derive(Debug, Clone, Default, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct UpdateMergeRequest {
    #[bourne(skip_if_none)]
    pub title: Option<String>,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    #[bourne(skip_if_none)]
    pub target_branch: Option<String>,
    #[bourne(skip_if_none)]
    pub assignee_ids: Option<Vec<i64>>,
    #[bourne(skip_if_none)]
    pub reviewer_ids: Option<Vec<i64>>,
    #[bourne(skip_if_none)]
    pub labels: Option<String>,
    #[bourne(skip_if_none)]
    pub milestone_id: Option<i64>,
    #[bourne(skip_if_none)]
    pub state_event: Option<String>,
    #[bourne(skip_if_none)]
    pub remove_source_branch: Option<bool>,
    #[bourne(skip_if_none)]
    pub squash: Option<bool>,
}

impl UpdateMergeRequest {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn with_target_branch(mut self, target_branch: impl Into<String>) -> Self {
        self.target_branch = Some(target_branch.into());
        self
    }

    #[must_use]
    pub fn with_assignees(mut self, assignee_ids: Vec<i64>) -> Self {
        self.assignee_ids = Some(assignee_ids);
        self
    }

    #[must_use]
    pub fn with_reviewers(mut self, reviewer_ids: Vec<i64>) -> Self {
        self.reviewer_ids = Some(reviewer_ids);
        self
    }

    #[must_use]
    pub fn with_labels(mut self, labels: &[&str]) -> Self {
        self.labels = Some(labels.join(","));
        self
    }

    #[must_use]
    pub fn with_milestone(mut self, milestone_id: i64) -> Self {
        self.milestone_id = Some(milestone_id);
        self
    }

    /// Close the merge request on update.
    #[must_use]
    pub fn close(mut self) -> Self {
        self.state_event = Some("close".to_string());
        self
    }

    /// Reopen the merge request on update.
    #[must_use]
    pub fn reopen(mut self) -> Self {
        self.state_event = Some("reopen".to_string());
        self
    }

    #[must_use]
    pub fn remove_source_branch(mut self, remove: bool) -> Self {
        self.remove_source_branch = Some(remove);
        self
    }

    #[must_use]
    pub fn squash(mut self, squash: bool) -> Self {
        self.squash = Some(squash);
        self
    }
}

/// Body for accepting/merging a merge request
/// (`PUT /projects/:id/merge_requests/:iid/merge`). All fields optional.
#[derive(Debug, Clone, Default, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct AcceptMergeRequest {
    #[bourne(skip_if_none)]
    pub merge_commit_message: Option<String>,
    #[bourne(skip_if_none)]
    pub squash_commit_message: Option<String>,
    #[bourne(skip_if_none)]
    pub squash: Option<bool>,
    #[bourne(skip_if_none)]
    pub should_remove_source_branch: Option<bool>,
    #[bourne(skip_if_none)]
    pub merge_when_pipeline_succeeds: Option<bool>,
    #[bourne(skip_if_none)]
    pub sha: Option<String>,
}

impl AcceptMergeRequest {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_merge_commit_message(mut self, message: impl Into<String>) -> Self {
        self.merge_commit_message = Some(message.into());
        self
    }

    #[must_use]
    pub fn with_squash_commit_message(mut self, message: impl Into<String>) -> Self {
        self.squash_commit_message = Some(message.into());
        self
    }

    #[must_use]
    pub fn squash(mut self, squash: bool) -> Self {
        self.squash = Some(squash);
        self
    }

    #[must_use]
    pub fn should_remove_source_branch(mut self, remove: bool) -> Self {
        self.should_remove_source_branch = Some(remove);
        self
    }

    #[must_use]
    pub fn merge_when_pipeline_succeeds(mut self, enable: bool) -> Self {
        self.merge_when_pipeline_succeeds = Some(enable);
        self
    }

    /// Require the tip of the source branch to match `sha` before merging.
    #[must_use]
    pub fn with_sha(mut self, sha: impl Into<String>) -> Self {
        self.sha = Some(sha.into());
        self
    }
}

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

    // --- Writes ---

    /// Create a merge request. Returns the created MR.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_merge_request(
        &self,
        project_id: i64,
        mr: &CreateMergeRequest,
    ) -> Result<MergeRequest, Error>;

    /// Update a merge request (by IID). Returns the updated MR.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn update_merge_request(
        &self,
        project_id: i64,
        mr_iid: i64,
        update: &UpdateMergeRequest,
    ) -> Result<MergeRequest, Error>;

    /// Accept (merge) a merge request. Returns the merged MR.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn accept_merge_request(
        &self,
        project_id: i64,
        mr_iid: i64,
        accept: &AcceptMergeRequest,
    ) -> Result<MergeRequest, Error>;

    /// Delete a merge request (by IID). Requires elevated permissions.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_merge_request(&self, project_id: i64, mr_iid: i64) -> Result<(), Error>;
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

    fn create_merge_request(
        &self,
        project_id: i64,
        mr: &CreateMergeRequest,
    ) -> Result<MergeRequest, Error> {
        self.post(&format!("api/v4/projects/{project_id}/merge_requests"), mr)
    }

    fn update_merge_request(
        &self,
        project_id: i64,
        mr_iid: i64,
        update: &UpdateMergeRequest,
    ) -> Result<MergeRequest, Error> {
        self.put(
            &format!("api/v4/projects/{project_id}/merge_requests/{mr_iid}"),
            update,
        )
    }

    fn accept_merge_request(
        &self,
        project_id: i64,
        mr_iid: i64,
        accept: &AcceptMergeRequest,
    ) -> Result<MergeRequest, Error> {
        self.put(
            &format!("api/v4/projects/{project_id}/merge_requests/{mr_iid}/merge"),
            accept,
        )
    }

    fn delete_merge_request(&self, project_id: i64, mr_iid: i64) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/projects/{project_id}/merge_requests/{mr_iid}"
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
