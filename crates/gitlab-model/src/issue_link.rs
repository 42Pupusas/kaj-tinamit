//! Wire shapes for the **Issue links** API category — typed dependencies
//! between issues (`relates_to`, `blocks`, `is_blocked_by`). Essential for
//! surfacing cross-story dependencies during planning.

use json_bourne::{FromJson, ToJson};

use crate::issue::GitlabIssue;

/// The kind of relationship a link expresses, from the perspective of the
/// issue the listing was requested for.
#[derive(Debug, FromJson, ToJson, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(rename_all = "snake_case")]
pub enum IssueLinkType {
    RelatesTo,
    Blocks,
    IsBlockedBy,
}

/// A linked issue as returned by `GET /projects/:id/issues/:iid/links`.
///
/// The payload is a full issue shape augmented with the link metadata; we
/// model the fields useful for dependency analysis rather than re-embedding
/// the entire [`GitlabIssue`](crate::GitlabIssue).
#[derive(Debug, FromJson, ToJson, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct IssueLink {
    pub id: i64,
    #[bourne(default)]
    pub iid: i64,
    pub project_id: Option<i64>,
    pub title: Option<String>,
    pub state: Option<String>,
    #[bourne(default)]
    pub labels: Vec<String>,
    /// Id of the link row itself — address it to inspect/remove the link.
    pub issue_link_id: Option<i64>,
    /// How the *target* issue relates to the source issue.
    pub link_type: Option<IssueLinkType>,
    pub link_created_at: Option<String>,
    pub link_updated_at: Option<String>,
    pub web_url: Option<String>,
}

/// The result of creating or deleting an issue link
/// (`POST`/`DELETE /projects/:id/issues/:iid/links[/:link_id]`).
///
/// Unlike the listing shape [`IssueLink`], this response echoes the two
/// full issues involved plus the relationship between them.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct IssueLinkResult {
    pub source_issue: GitlabIssue,
    pub target_issue: GitlabIssue,
    pub link_type: Option<IssueLinkType>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_issue_link() {
        let json = r#"{
            "id": 84,
            "iid": 14,
            "project_id": 4,
            "title": "Login flow",
            "state": "opened",
            "issue_link_id": 2,
            "link_type": "is_blocked_by"
        }"#;
        let l: IssueLink = parse_str(json).unwrap();
        assert_eq!(l.iid, 14);
        assert_eq!(l.link_type, Some(IssueLinkType::IsBlockedBy));
        assert_eq!(l.issue_link_id, Some(2));
    }
}
