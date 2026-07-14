//! # gitlab-model
//!
//! Type-safe wire shapes for the GitLab REST API v4, (de)serialized via the
//! in-house [`json-bourne`](json_bourne). Pure data — no transport, no I/O.
//!
//! Enums are used instead of stringly-typed fields wherever GitLab's values
//! are a closed set (states, visibility, severity). Structs set
//! `deny_unknown_fields = false` so GitLab adding a field never breaks
//! parsing.

pub mod commit;
pub mod event;
pub mod group;
pub mod issue;
pub mod label;
pub mod merge_request;
pub mod project;
pub mod repository;
pub mod user;
pub mod wiki;

pub use commit::*;
pub use event::*;
pub use group::*;
pub use issue::*;
pub use label::*;
pub use merge_request::*;
pub use project::*;
pub use repository::*;
pub use user::*;
pub use wiki::*;

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_user() {
        let json = r#"{
            "id": 42,
            "username": "alice",
            "name": "Alice",
            "state": "active",
            "locked": false,
            "avatar_url": "https://example.com/a.png",
            "web_url": "https://example.com/alice"
        }"#;
        let user: GitlabUser = parse_str(json).unwrap();
        assert_eq!(user.id, 42);
        assert_eq!(user.username, "alice");
        assert_eq!(user.state, UserState::Active);
    }

    #[test]
    fn parse_issue_minimal_with_unknown_fields() {
        // Only a couple fields present + an unknown one GitLab might add.
        let json = r#"{
            "id": 1,
            "iid": 7,
            "title": "Bug",
            "state": "opened",
            "type": "ISSUE",
            "some_future_field": {"nested": true}
        }"#;
        let issue: GitlabIssue = parse_str(json).unwrap();
        assert_eq!(issue.iid, 7);
        assert_eq!(issue.state, Some(IssueState::Opened));
        assert_eq!(issue.issue_type, Some(IssueType::Issue));
        assert!(issue.labels.is_empty());
    }

    #[test]
    fn unknown_issue_type_falls_back() {
        let json = r#"{"id":1,"iid":1,"type":"SOMETHING_NEW"}"#;
        let issue: GitlabIssue = parse_str(json).unwrap();
        assert_eq!(issue.issue_type, Some(IssueType::Unknown));
    }
}
