//! Issue endpoints and the [`IssueQuery`] filter builder.

use gitlab_model::{GitlabIssue, IssueStatistics};

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

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
