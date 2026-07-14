//! Search read endpoints: the **Search** API category — instance-, group-,
//! and project-scoped.
//!
//! Each GitLab search is `scope` + `search` term; the row shape depends on
//! the scope, so we expose one typed method per commonly-used scope rather
//! than a stringly-typed catch-all. All read-only (GET).

use gitlab_model::{GitlabIssue, GitlabProject, GitlabUser, MergeRequest, SearchBlob};

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

/// The level a search runs at, mapped to the right URL prefix.
enum Scope {
    Global,
    Group(i64),
    Project(i64),
}

impl Scope {
    fn path(&self, scope: &str, term: &str) -> String {
        let base = match self {
            Scope::Global => "api/v4/search".to_string(),
            Scope::Group(id) => format!("api/v4/groups/{id}/search"),
            Scope::Project(id) => format!("api/v4/projects/{id}/search"),
        };
        format!("{base}?scope={scope}&search={}", term.percent_encode())
    }
}

/// Read endpoints for GitLab search, one method per scope × level.
pub trait SearchEndpoints {
    /// Search projects instance-wide.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn search_projects(&self, term: &str) -> Result<Vec<GitlabProject>, Error>;

    /// Search issues instance-wide.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn search_issues(&self, term: &str) -> Result<Vec<GitlabIssue>, Error>;

    /// Search merge requests instance-wide.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn search_merge_requests(&self, term: &str) -> Result<Vec<MergeRequest>, Error>;

    /// Search users instance-wide.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn search_users(&self, term: &str) -> Result<Vec<GitlabUser>, Error>;

    /// Search issues within a group.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_search_issues(&self, group_id: i64, term: &str) -> Result<Vec<GitlabIssue>, Error>;

    /// Search merge requests within a group.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_search_merge_requests(
        &self,
        group_id: i64,
        term: &str,
    ) -> Result<Vec<MergeRequest>, Error>;

    /// Search issues within a project.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_search_issues(&self, project_id: i64, term: &str)
    -> Result<Vec<GitlabIssue>, Error>;

    /// Search merge requests within a project.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_search_merge_requests(
        &self,
        project_id: i64,
        term: &str,
    ) -> Result<Vec<MergeRequest>, Error>;

    /// Search code (blobs) within a project.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_search_blobs(&self, project_id: i64, term: &str) -> Result<Vec<SearchBlob>, Error>;
}

impl SearchEndpoints for GitlabClient {
    fn search_projects(&self, term: &str) -> Result<Vec<GitlabProject>, Error> {
        self.get_paginated(&Scope::Global.path("projects", term))
    }

    fn search_issues(&self, term: &str) -> Result<Vec<GitlabIssue>, Error> {
        self.get_paginated(&Scope::Global.path("issues", term))
    }

    fn search_merge_requests(&self, term: &str) -> Result<Vec<MergeRequest>, Error> {
        self.get_paginated(&Scope::Global.path("merge_requests", term))
    }

    fn search_users(&self, term: &str) -> Result<Vec<GitlabUser>, Error> {
        self.get_paginated(&Scope::Global.path("users", term))
    }

    fn group_search_issues(&self, group_id: i64, term: &str) -> Result<Vec<GitlabIssue>, Error> {
        self.get_paginated(&Scope::Group(group_id).path("issues", term))
    }

    fn group_search_merge_requests(
        &self,
        group_id: i64,
        term: &str,
    ) -> Result<Vec<MergeRequest>, Error> {
        self.get_paginated(&Scope::Group(group_id).path("merge_requests", term))
    }

    fn project_search_issues(
        &self,
        project_id: i64,
        term: &str,
    ) -> Result<Vec<GitlabIssue>, Error> {
        self.get_paginated(&Scope::Project(project_id).path("issues", term))
    }

    fn project_search_merge_requests(
        &self,
        project_id: i64,
        term: &str,
    ) -> Result<Vec<MergeRequest>, Error> {
        self.get_paginated(&Scope::Project(project_id).path("merge_requests", term))
    }

    fn project_search_blobs(&self, project_id: i64, term: &str) -> Result<Vec<SearchBlob>, Error> {
        self.get_paginated(&Scope::Project(project_id).path("blobs", term))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn global_scope_path() {
        assert_eq!(
            Scope::Global.path("issues", "fix bug"),
            "api/v4/search?scope=issues&search=fix%20bug"
        );
    }

    #[test]
    fn project_scope_path() {
        assert_eq!(
            Scope::Project(7).path("blobs", "TODO"),
            "api/v4/projects/7/search?scope=blobs&search=TODO"
        );
    }
}
