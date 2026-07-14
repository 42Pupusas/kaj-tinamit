//! Issue endpoints and the [`IssueQuery`] filter builder.

use gitlab_model::{GitlabIssue, IssueStatistics};
use json_bourne::ToJson;

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

/// Body for creating an issue via `POST /projects/:id/issues`.
///
/// Only `title` is required; every other field is omitted from the JSON
/// payload when unset (via `skip_if_none`), so GitLab applies its own
/// defaults. Build fluently, then hand to [`IssueEndpoints::create_issue`].
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct CreateIssue {
    pub title: String,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    #[bourne(skip_if_none)]
    pub labels: Option<String>,
    #[bourne(skip_if_none)]
    pub assignee_ids: Option<Vec<i64>>,
    #[bourne(skip_if_none)]
    pub milestone_id: Option<i64>,
    #[bourne(skip_if_none)]
    pub confidential: Option<bool>,
    #[bourne(skip_if_none)]
    pub due_date: Option<String>,
}

impl CreateIssue {
    /// Start a new issue with the given (required) title.
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            labels: None,
            assignee_ids: None,
            milestone_id: None,
            confidential: None,
            due_date: None,
        }
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set labels; GitLab wants a comma-separated list, so `labels` is joined.
    #[must_use]
    pub fn with_labels(mut self, labels: &[&str]) -> Self {
        self.labels = Some(labels.join(","));
        self
    }

    #[must_use]
    pub fn with_assignees(mut self, assignee_ids: Vec<i64>) -> Self {
        self.assignee_ids = Some(assignee_ids);
        self
    }

    #[must_use]
    pub fn with_milestone(mut self, milestone_id: i64) -> Self {
        self.milestone_id = Some(milestone_id);
        self
    }

    #[must_use]
    pub fn confidential(mut self, confidential: bool) -> Self {
        self.confidential = Some(confidential);
        self
    }

    #[must_use]
    pub fn with_due_date(mut self, due_date: impl Into<String>) -> Self {
        self.due_date = Some(due_date.into());
        self
    }
}

/// Body for updating an issue via `PUT /projects/:id/issues/:iid`.
///
/// Every field is optional and omitted when unset, so an update touches
/// only the fields you set. `state_event` drives close/reopen
/// (`"close"` / `"reopen"`).
#[derive(Debug, Clone, Default, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct UpdateIssue {
    #[bourne(skip_if_none)]
    pub title: Option<String>,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    #[bourne(skip_if_none)]
    pub labels: Option<String>,
    #[bourne(skip_if_none)]
    pub assignee_ids: Option<Vec<i64>>,
    #[bourne(skip_if_none)]
    pub milestone_id: Option<i64>,
    #[bourne(skip_if_none)]
    pub state_event: Option<String>,
    #[bourne(skip_if_none)]
    pub confidential: Option<bool>,
    #[bourne(skip_if_none)]
    pub due_date: Option<String>,
}

impl UpdateIssue {
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
    pub fn with_labels(mut self, labels: &[&str]) -> Self {
        self.labels = Some(labels.join(","));
        self
    }

    #[must_use]
    pub fn with_assignees(mut self, assignee_ids: Vec<i64>) -> Self {
        self.assignee_ids = Some(assignee_ids);
        self
    }

    #[must_use]
    pub fn with_milestone(mut self, milestone_id: i64) -> Self {
        self.milestone_id = Some(milestone_id);
        self
    }

    /// Close the issue on update.
    #[must_use]
    pub fn close(mut self) -> Self {
        self.state_event = Some("close".to_string());
        self
    }

    /// Reopen the issue on update.
    #[must_use]
    pub fn reopen(mut self) -> Self {
        self.state_event = Some("reopen".to_string());
        self
    }

    #[must_use]
    pub fn confidential(mut self, confidential: bool) -> Self {
        self.confidential = Some(confidential);
        self
    }

    #[must_use]
    pub fn with_due_date(mut self, due_date: impl Into<String>) -> Self {
        self.due_date = Some(due_date.into());
        self
    }
}

/// Query parameters for filtering issues. Build fluently, then hand to
/// [`GitlabClient::issues`] or [`GitlabClient::project_issues`].
#[derive(Debug, Default, Clone)]
pub struct IssueQuery {
    pub scope: Option<IssueScope>,
    pub assignee_username: Option<String>,
    pub state: Option<IssueStateFilter>,
    pub labels: Vec<String>,
    pub milestone: Option<String>,
    /// Only issues updated on or after this instant (ISO 8601, e.g.
    /// `2021-01-01T00:00:00Z`). Maps to GitLab's `updated_after`.
    pub updated_after: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum IssueScope {
    CreatedByMe,
    AssignedToMe,
    All,
}

#[derive(Debug, Clone, Copy)]
pub enum IssueStateFilter {
    Opened,
    Closed,
    All,
}

impl IssueQuery {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_scope(mut self, scope: IssueScope) -> Self {
        self.scope = Some(scope);
        self
    }

    #[must_use]
    pub fn with_assignee(mut self, username: impl Into<String>) -> Self {
        self.assignee_username = Some(username.into());
        self
    }

    #[must_use]
    pub fn with_state(mut self, state: IssueStateFilter) -> Self {
        self.state = Some(state);
        self
    }

    #[must_use]
    pub fn with_labels(mut self, labels: Vec<String>) -> Self {
        self.labels = labels;
        self
    }

    #[must_use]
    pub fn with_milestone(mut self, milestone: impl Into<String>) -> Self {
        self.milestone = Some(milestone.into());
        self
    }

    /// Restrict to issues updated on or after `timestamp` (ISO 8601).
    /// Server-side, so it doesn't drag the whole issue history over the wire.
    #[must_use]
    pub fn with_updated_after(mut self, timestamp: impl Into<String>) -> Self {
        self.updated_after = Some(timestamp.into());
        self
    }

    /// Build the `?a=b&c=d` query string, percent-encoding all values.
    /// Returns an empty string when no filters are set.
    fn to_query_string(&self) -> String {
        let mut params: Vec<String> = Vec::new();

        if let Some(scope) = self.scope {
            let s = match scope {
                IssueScope::CreatedByMe => "created_by_me",
                IssueScope::AssignedToMe => "assigned_to_me",
                IssueScope::All => "all",
            };
            params.push(format!("scope={s}"));
        }

        if let Some(username) = &self.assignee_username {
            params.push(format!("assignee_username={}", username.percent_encode()));
        }

        if let Some(state) = self.state {
            let s = match state {
                IssueStateFilter::Opened => "opened",
                IssueStateFilter::Closed => "closed",
                IssueStateFilter::All => "all",
            };
            params.push(format!("state={s}"));
        }

        if !self.labels.is_empty() {
            let joined = self
                .labels
                .iter()
                .map(|l| l.percent_encode())
                .collect::<Vec<_>>()
                .join(",");
            params.push(format!("labels={joined}"));
        }

        if let Some(milestone) = &self.milestone {
            params.push(format!("milestone={}", milestone.percent_encode()));
        }

        if let Some(updated_after) = &self.updated_after {
            params.push(format!("updated_after={}", updated_after.percent_encode()));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

/// Issue-related GitLab endpoints.
pub trait IssueEndpoints {
    /// Fetch issues across the instance with optional filters.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn issues(&self, query: &IssueQuery) -> Result<Vec<GitlabIssue>, Error>;

    /// Fetch all issues assigned to `username` (instance-wide).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn user_issues(&self, username: &str) -> Result<Vec<GitlabIssue>, Error>;

    /// Fetch all issues updated on or after `timestamp` (ISO 8601).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn issues_updated_after(&self, timestamp: impl Into<String>)
    -> Result<Vec<GitlabIssue>, Error>;

    /// Fetch issues for a specific project with optional filters.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_issues(
        &self,
        project_id: i32,
        query: &IssueQuery,
    ) -> Result<Vec<GitlabIssue>, Error>;

    /// Fetch a single issue by project ID and issue IID (project-scoped ID).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn issue(&self, project_id: i32, issue_iid: i32) -> Result<GitlabIssue, Error>;

    /// Fetch instance-wide issue statistics (open/closed/all counts) matching
    /// the given filters — cheaper than paging every issue for a summary.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn issues_statistics(&self, query: &IssueQuery) -> Result<IssueStatistics, Error>;

    /// Fetch issue statistics scoped to a single project.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_issues_statistics(
        &self,
        project_id: i32,
        query: &IssueQuery,
    ) -> Result<IssueStatistics, Error>;

    /// Create a new issue in a project. Returns the created issue.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_issue(&self, project_id: i32, issue: &CreateIssue) -> Result<GitlabIssue, Error>;

    /// Update an existing issue (by IID). Returns the updated issue.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn update_issue(
        &self,
        project_id: i32,
        issue_iid: i32,
        update: &UpdateIssue,
    ) -> Result<GitlabIssue, Error>;

    /// Delete an issue (by IID). Requires elevated permissions on GitLab.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_issue(&self, project_id: i32, issue_iid: i32) -> Result<(), Error>;
}

impl IssueEndpoints for GitlabClient {
    fn issues(&self, query: &IssueQuery) -> Result<Vec<GitlabIssue>, Error> {
        let path = format!("api/v4/issues{}", query.to_query_string());
        self.get_paginated(&path)
    }

    fn user_issues(&self, username: &str) -> Result<Vec<GitlabIssue>, Error> {
        let query = IssueQuery::new()
            .with_assignee(username)
            .with_scope(IssueScope::All);
        self.issues(&query)
    }

    fn issues_updated_after(
        &self,
        timestamp: impl Into<String>,
    ) -> Result<Vec<GitlabIssue>, Error> {
        let query = IssueQuery::new()
            .with_scope(IssueScope::All)
            .with_updated_after(timestamp);
        self.issues(&query)
    }

    fn project_issues(
        &self,
        project_id: i32,
        query: &IssueQuery,
    ) -> Result<Vec<GitlabIssue>, Error> {
        let path = format!(
            "api/v4/projects/{project_id}/issues{}",
            query.to_query_string()
        );
        self.get_paginated(&path)
    }

    fn issue(&self, project_id: i32, issue_iid: i32) -> Result<GitlabIssue, Error> {
        self.get(&format!("api/v4/projects/{project_id}/issues/{issue_iid}"))
    }

    fn issues_statistics(&self, query: &IssueQuery) -> Result<IssueStatistics, Error> {
        self.get(&format!(
            "api/v4/issues_statistics{}",
            query.to_query_string()
        ))
    }

    fn project_issues_statistics(
        &self,
        project_id: i32,
        query: &IssueQuery,
    ) -> Result<IssueStatistics, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/issues_statistics{}",
            query.to_query_string()
        ))
    }

    fn create_issue(&self, project_id: i32, issue: &CreateIssue) -> Result<GitlabIssue, Error> {
        self.post(&format!("api/v4/projects/{project_id}/issues"), issue)
    }

    fn update_issue(
        &self,
        project_id: i32,
        issue_iid: i32,
        update: &UpdateIssue,
    ) -> Result<GitlabIssue, Error> {
        self.put(
            &format!("api/v4/projects/{project_id}/issues/{issue_iid}"),
            update,
        )
    }

    fn delete_issue(&self, project_id: i32, issue_iid: i32) -> Result<(), Error> {
        self.delete(&format!("api/v4/projects/{project_id}/issues/{issue_iid}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_string_empty_by_default() {
        assert_eq!(IssueQuery::new().to_query_string(), "");
    }

    #[test]
    fn query_string_includes_updated_after() {
        let qs = IssueQuery::new()
            .with_scope(IssueScope::All)
            .with_updated_after("2021-01-01T00:00:00Z")
            .to_query_string();
        assert!(qs.starts_with('?'));
        assert!(qs.contains("scope=all"));
        // Colons are percent-encoded to %3A.
        assert!(qs.contains("updated_after=2021-01-01T00%3A00%3A00Z"));
    }

    #[test]
    fn create_issue_serializes_only_set_fields() {
        let body = CreateIssue::new("Bug").with_labels(&["bug", "p1"]);
        let json = json_bourne::to_string(&body).unwrap();
        assert!(json.contains("\"title\":\"Bug\""));
        assert!(json.contains("\"labels\":\"bug,p1\""));
        // Unset optionals must be omitted, not emitted as null.
        assert!(!json.contains("description"));
        assert!(!json.contains("assignee_ids"));
        assert!(!json.contains("null"));
    }

    #[test]
    fn update_issue_close_sets_state_event() {
        let body = UpdateIssue::new().close();
        let json = json_bourne::to_string(&body).unwrap();
        assert_eq!(json, "{\"state_event\":\"close\"}");
    }

    #[test]
    fn update_issue_empty_is_empty_object() {
        let json = json_bourne::to_string(&UpdateIssue::new()).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn query_string_combines_filters() {
        let qs = IssueQuery::new()
            .with_assignee("alice")
            .with_state(IssueStateFilter::Opened)
            .with_labels(vec!["bug".to_string(), "needs review".to_string()])
            .to_query_string();
        assert!(qs.contains("assignee_username=alice"));
        assert!(qs.contains("state=opened"));
        // Label list is comma-joined; the space in the second label is encoded.
        assert!(qs.contains("labels=bug,needs%20review"));
    }
}
