//! Wire shapes for the **Todos** API category — the authenticated user's
//! action queue (review requests, mentions, assignments). The reviewer's
//! inbox, from a delivery standpoint.

use json_bourne::{FromJson, ToJson};

use crate::issue::Author;

/// Why a to-do was created.
#[derive(Debug, FromJson, ToJson, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(rename_all = "snake_case")]
pub enum TodoActionName {
    Assigned,
    Mentioned,
    BuildFailed,
    Marked,
    ApprovalRequired,
    Unmergeable,
    DirectlyAddressed,
    MergeTrainRemoved,
    ReviewRequested,
}

/// Whether a to-do is still actionable.
#[derive(Debug, FromJson, ToJson, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(rename_all = "lowercase")]
pub enum TodoState {
    Pending,
    Done,
}

/// A lightweight project reference embedded in a to-do. The Todos endpoint
/// returns only a slim project object, so we model that rather than reusing
/// the full [`GitlabProject`](crate::GitlabProject) (which isn't comparable).
#[derive(Debug, FromJson, ToJson, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct TodoProject {
    pub id: i64,
    pub name: Option<String>,
    pub name_with_namespace: Option<String>,
    pub path: Option<String>,
    pub path_with_namespace: Option<String>,
}

/// A single to-do item. The `target` varies by `target_type` (Issue,
/// MergeRequest, Epic, …); we keep it as raw fields common to all rather
/// than a typed union, so a new target type never breaks parsing.
#[derive(Debug, FromJson, ToJson, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct Todo {
    pub id: i64,
    pub project: Option<TodoProject>,
    pub author: Option<Author>,
    pub action_name: Option<TodoActionName>,
    /// `Issue`, `MergeRequest`, `Epic`, `DesignManagement::Design`, …
    pub target_type: Option<String>,
    /// URL of the object the to-do points at.
    pub target_url: Option<String>,
    /// A short excerpt of the triggering comment/description.
    pub body: Option<String>,
    pub state: Option<TodoState>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_todo() {
        let json = r#"{
            "id": 102,
            "action_name": "review_requested",
            "target_type": "MergeRequest",
            "target_url": "https://example.com/g/p/-/merge_requests/7",
            "body": "Please review",
            "state": "pending",
            "created_at": "2021-01-02T09:00:00Z"
        }"#;
        let t: Todo = parse_str(json).unwrap();
        assert_eq!(t.action_name, Some(TodoActionName::ReviewRequested));
        assert_eq!(t.state, Some(TodoState::Pending));
    }
}
