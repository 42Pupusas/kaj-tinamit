//! Wire shapes for the **Epics** API category (group-level). Epics group
//! issues and other epics into a portfolio/roadmap hierarchy. Premium-tier
//! on GitLab.com, but the wire shape is stable.

use json_bourne::{FromJson, ToJson};

use crate::issue::Author;

/// Epic lifecycle state.
#[derive(Debug, FromJson, ToJson, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(rename_all = "lowercase")]
pub enum EpicState {
    Opened,
    Closed,
}

/// A group epic.
#[derive(Debug, FromJson, ToJson, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct Epic {
    pub id: i64,
    /// Group-scoped id used by most epic sub-endpoints.
    #[bourne(default)]
    pub iid: i64,
    pub group_id: Option<i64>,
    /// The parent epic's id, when this epic is nested.
    pub parent_id: Option<i64>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub state: Option<EpicState>,
    pub author: Option<Author>,
    #[bourne(default)]
    pub labels: Vec<String>,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    /// Inherited/fixed date fields — useful for roadmap rendering.
    pub start_date_is_fixed: Option<bool>,
    pub due_date_is_fixed: Option<bool>,
    pub start_date_from_inherited_source: Option<String>,
    pub due_date_from_inherited_source: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub closed_at: Option<String>,
    #[bourne(default)]
    pub confidential: bool,
    #[bourne(default)]
    pub upvotes: i64,
    #[bourne(default)]
    pub downvotes: i64,
    pub web_url: Option<String>,
}

/// A link between an epic and one of its child issues. Returned by the epic
/// issues listing; it's an issue shape plus the epic-membership metadata
/// (`epic_issue_id`, `relative_position`).
#[derive(Debug, FromJson, ToJson, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct EpicIssue {
    pub id: i64,
    #[bourne(default)]
    pub iid: i64,
    pub project_id: Option<i64>,
    pub title: Option<String>,
    pub state: Option<String>,
    /// Id of the epic-issue link itself (not the issue) — needed to address
    /// the membership.
    pub epic_issue_id: Option<i64>,
    pub relative_position: Option<i64>,
    pub web_url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_epic() {
        let json = r#"{
            "id": 30,
            "iid": 5,
            "group_id": 7,
            "title": "Q3 Roadmap",
            "state": "opened",
            "labels": ["roadmap"],
            "start_date": "2021-07-01",
            "due_date": "2021-09-30"
        }"#;
        let e: Epic = parse_str(json).unwrap();
        assert_eq!(e.iid, 5);
        assert_eq!(e.state, Some(EpicState::Opened));
        assert_eq!(e.labels, vec!["roadmap".to_string()]);
    }
}
