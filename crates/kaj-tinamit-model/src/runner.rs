//! Wire shapes for the **Runners** API category.

use json_bourne::{FromJson, ToJson};

use crate::Id;

/// A CI runner as returned by the list endpoints (concise shape).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct Runner {
    pub id: Id,
    pub description: Option<String>,
    #[bourne(default)]
    pub active: bool,
    #[bourne(default)]
    pub paused: bool,
    #[bourne(default)]
    pub is_shared: bool,
    pub runner_type: Option<String>,
    pub name: Option<String>,
    pub online: Option<bool>,
    pub status: Option<String>,
    pub job_execution_status: Option<String>,
    pub ip_address: Option<String>,
}

/// A project a runner is assigned to (concise subset embedded in runner
/// details).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct RunnerProject {
    pub id: Id,
    pub name: Option<String>,
    pub name_with_namespace: Option<String>,
    pub path: Option<String>,
    pub path_with_namespace: Option<String>,
}

/// Full runner details (`GET /runners/:id`), with assigned projects and
/// configuration.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct RunnerDetail {
    pub id: Id,
    pub description: Option<String>,
    #[bourne(default)]
    pub active: bool,
    #[bourne(default)]
    pub paused: bool,
    #[bourne(default)]
    pub is_shared: bool,
    pub runner_type: Option<String>,
    pub name: Option<String>,
    pub online: Option<bool>,
    pub status: Option<String>,
    pub job_execution_status: Option<String>,
    pub contacted_at: Option<String>,
    pub maintenance_note: Option<String>,
    pub access_level: Option<String>,
    pub maximum_timeout: Option<i64>,
    #[bourne(default)]
    pub tag_list: Vec<String>,
    #[bourne(default)]
    pub projects: Vec<RunnerProject>,
}

/// A runner manager: an instance of the runner process
/// (`GET /runners/:id/managers`).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct RunnerManager {
    pub id: Id,
    pub system_id: Option<String>,
    pub version: Option<String>,
    pub revision: Option<String>,
    pub platform: Option<String>,
    pub architecture: Option<String>,
    pub created_at: Option<String>,
    pub contacted_at: Option<String>,
    pub ip_address: Option<String>,
    pub status: Option<String>,
    pub job_execution_status: Option<String>,
}

/// The authentication token returned when resetting a runner's token
/// (`POST /runners/:id/reset_authentication_token`).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct RunnerAuthToken {
    pub token: String,
    pub token_expires_at: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_runner() {
        let json = r#"{
            "active": true, "paused": false, "description": "test-1-20150125",
            "id": 6, "ip_address": "", "is_shared": false, "runner_type": "project_type",
            "name": null, "online": true, "status": "online", "job_execution_status": "idle"
        }"#;
        let r: Runner = parse_str(json).unwrap();
        assert_eq!(r.id, 6);
        assert_eq!(r.runner_type.as_deref(), Some("project_type"));
        assert_eq!(r.status.as_deref(), Some("online"));
    }

    #[test]
    fn parse_runner_detail_with_projects() {
        let json = r#"{
            "id": 6, "description": "test-1", "runner_type": "project_type",
            "status": "online", "access_level": "ref_protected", "maximum_timeout": 3600,
            "tag_list": ["ruby", "mysql"],
            "projects": [{"id": 1, "name": "CE", "path": "gitlab-foss",
                          "path_with_namespace": "gitlab-org/gitlab-foss"}]
        }"#;
        let r: RunnerDetail = parse_str(json).unwrap();
        assert_eq!(r.tag_list.len(), 2);
        assert_eq!(r.projects.len(), 1);
        assert_eq!(r.maximum_timeout, Some(3600));
    }
}
