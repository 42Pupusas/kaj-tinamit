//! Wire shapes for the **Notes** API category (comments and system records
//! on issues, merge requests, snippets, epics, and wiki pages).
//!
//! This is the comprehensive note shape. A narrower [`Note`](crate::merge_request::Note)
//! exists for the issue/MR-embedded case; `GitlabNote` here uses a permissive
//! string `noteable_type` so it also parses epic and `WikiPage::Meta` notes.

use json_bourne::{FromJson, ToJson};

use crate::Id;

/// The author of a note (includes `email`, exposed on note responses).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct NoteAuthor {
    pub id: Id,
    pub username: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub state: Option<String>,
    pub avatar_url: Option<String>,
    pub web_url: Option<String>,
    pub created_at: Option<String>,
}

/// A note: a comment or system record attached to a GitLab object.
///
/// `noteable_type` is a free-form string (`Issue`, `MergeRequest`,
/// `Snippet`, `Epic`, `Commit`, `WikiPage::Meta`, …) so this one type covers
/// every notes endpoint. `system` distinguishes user comments (`false`) from
/// system-generated records (`true`).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct GitlabNote {
    pub id: Id,
    pub body: Option<String>,
    pub author: Option<NoteAuthor>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    #[bourne(default)]
    pub system: bool,
    pub noteable_id: Option<Id>,
    pub noteable_iid: Option<Id>,
    pub noteable_type: Option<String>,
    pub project_id: Option<Id>,
    #[bourne(default)]
    pub resolvable: bool,
    pub resolved: Option<bool>,
    #[bourne(default)]
    pub confidential: bool,
    #[bourne(default)]
    pub internal: bool,
    pub imported: Option<bool>,
    pub imported_from: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_issue_note() {
        let json = r#"{
            "id": 302, "body": "closed",
            "author": {"id": 1, "username": "pipin", "email": "admin@example.com",
                       "name": "Pip", "state": "active"},
            "created_at": "2013-10-02T09:22:45Z", "updated_at": "2013-10-02T10:22:45Z",
            "system": true, "noteable_id": 377, "noteable_type": "Issue",
            "project_id": 5, "noteable_iid": 377, "resolvable": false,
            "confidential": false, "internal": false
        }"#;
        let n: GitlabNote = parse_str(json).unwrap();
        assert_eq!(n.id, 302);
        assert!(n.system);
        assert_eq!(n.noteable_type.as_deref(), Some("Issue"));
    }

    #[test]
    fn parse_wiki_note_with_unknown_noteable_type() {
        // WikiPage::Meta would break a strict enum; the string field copes.
        let json = r#"{
            "id": 1218, "body": "foobar", "noteable_type": "WikiPage::Meta",
            "noteable_id": 35, "noteable_iid": null, "project_id": 5,
            "system": false, "resolvable": false
        }"#;
        let n: GitlabNote = parse_str(json).unwrap();
        assert_eq!(n.noteable_type.as_deref(), Some("WikiPage::Meta"));
        assert!(n.noteable_iid.is_none());
    }
}
