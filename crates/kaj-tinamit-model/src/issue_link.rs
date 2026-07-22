//! Wire shapes for the **Issue links** API category — typed dependencies
//! between issues (`relates_to`, `blocks`, `is_blocked_by`). Essential for
//! surfacing cross-story dependencies during planning.

use json_bourne::{FromJson, Lexer, ToJson};

use crate::Id;
use crate::issue::GitlabIssue;

/// The kind of relationship a link expresses, from the perspective of the
/// issue the listing was requested for. Unknown values fall back to
/// [`IssueLinkType::Unknown`].
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[non_exhaustive]
pub enum IssueLinkType {
    RelatesTo,
    Blocks,
    IsBlockedBy,
    Unknown,
}

impl<'input> FromJson<'input> for IssueLinkType {
    fn from_lex(lex: &mut Lexer<'input>) -> Result<Self, json_bourne::Error> {
        let s = String::from_lex(lex)?;
        Ok(match s.as_str() {
            "relates_to" => Self::RelatesTo,
            "blocks" => Self::Blocks,
            "is_blocked_by" => Self::IsBlockedBy,
            _ => Self::Unknown,
        })
    }
}

impl ToJson for IssueLinkType {
    fn write_json<W: json_bourne::JsonWrite + ?Sized>(&self, w: &mut W) -> Result<(), W::Error> {
        let s = match self {
            Self::RelatesTo => "relates_to",
            Self::Blocks => "blocks",
            Self::IsBlockedBy => "is_blocked_by",
            Self::Unknown => "unknown",
        };
        s.write_json(w)
    }
}

/// A linked issue as returned by `GET /projects/:id/issues/:iid/links`.
///
/// The payload is a full issue shape augmented with the link metadata; we
/// model the fields useful for dependency analysis rather than re-embedding
/// the entire [`GitlabIssue`](crate::GitlabIssue).
#[derive(Debug, FromJson, ToJson, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct IssueLink {
    pub id: Id,
    #[bourne(default)]
    pub iid: Id,
    pub project_id: Option<Id>,
    pub title: Option<String>,
    pub state: Option<String>,
    #[bourne(default)]
    pub labels: Vec<String>,
    /// Id of the link row itself — address it to inspect/remove the link.
    pub issue_link_id: Option<Id>,
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
#[non_exhaustive]
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
        assert_eq!(l.issue_link_id, Some(Id::new(2)));
    }
}
