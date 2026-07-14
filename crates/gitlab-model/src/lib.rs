//! # gitlab-model
//!
//! Type-safe wire shapes for the GitLab REST API v4, (de)serialized via the
//! in-house [`json-bourne`](json_bourne). Pure data — no transport, no I/O.
//!
//! Enums are used instead of stringly-typed fields wherever GitLab's values
//! are a closed set (states, visibility, severity). Structs set
//! `deny_unknown_fields = false` so GitLab adding a field never breaks
//! parsing.

pub mod award_emoji;
pub mod board;
pub mod ci;
pub mod commit;
pub mod commit_status;
pub mod deploy;
pub mod deployment;
pub mod epic;
pub mod event;
pub mod group;
pub mod issue;
pub mod issue_link;
pub mod issue_statistics;
pub mod iteration;
pub mod label;
pub mod merge_request;
pub mod metadata;
pub mod note;
pub mod pipeline_schedule;
pub mod project;
pub mod protected;
pub mod release;
pub mod repository;
pub mod resource_event;
pub mod runner;
pub mod search;
pub mod snippet;
pub mod todo;
pub mod user;
pub mod wiki;

pub use award_emoji::*;
pub use board::*;
pub use ci::*;
pub use commit::*;
pub use commit_status::*;
pub use deploy::*;
pub use deployment::*;
pub use epic::*;
pub use event::*;
pub use group::*;
pub use issue::*;
pub use issue_link::*;
pub use issue_statistics::*;
pub use iteration::*;
pub use label::*;
pub use merge_request::*;
pub use metadata::*;
pub use note::*;
pub use pipeline_schedule::*;
pub use project::*;
pub use protected::*;
pub use release::*;
pub use repository::*;
pub use resource_event::*;
pub use runner::*;
pub use search::*;
pub use snippet::*;
pub use todo::*;
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
