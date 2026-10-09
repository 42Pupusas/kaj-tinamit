//! Wire shapes for the **Project webhooks** API category.

use crate::Id;
use json_bourne::{FromJson, ToJson};

/// A project webhook (`GET /projects/:id/hooks`, `.../hooks/:hook_id`).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct ProjectHook {
    pub id: Id,
    pub url: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub project_id: Option<Id>,
    #[bourne(default)]
    pub push_events: bool,
    pub push_events_branch_filter: Option<String>,
    #[bourne(default)]
    pub issues_events: bool,
    #[bourne(default)]
    pub confidential_issues_events: bool,
    #[bourne(default)]
    pub merge_requests_events: bool,
    #[bourne(default)]
    pub tag_push_events: bool,
    #[bourne(default)]
    pub note_events: bool,
    #[bourne(default)]
    pub confidential_note_events: bool,
    #[bourne(default)]
    pub job_events: bool,
    #[bourne(default)]
    pub pipeline_events: bool,
    #[bourne(default)]
    pub wiki_page_events: bool,
    #[bourne(default)]
    pub deployment_events: bool,
    #[bourne(default)]
    pub releases_events: bool,
    #[bourne(default)]
    pub enable_ssl_verification: bool,
    pub alert_status: Option<String>,
    pub disabled_until: Option<String>,
    pub created_at: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_hook() {
        let json = r#"{
            "id": 1,
            "url": "http://example.com/hook",
            "name": "Hook name",
            "project_id": 3,
            "push_events": true,
            "push_events_branch_filter": "",
            "pipeline_events": true,
            "enable_ssl_verification": true,
            "alert_status": "executable",
            "disabled_until": null,
            "created_at": "2012-10-12T17:04:47Z"
        }"#;
        let h: ProjectHook = parse_str(json).unwrap();
        assert_eq!(h.id, 1);
        assert!(h.push_events);
        assert!(h.pipeline_events);
        assert!(!h.job_events);
    }
}
