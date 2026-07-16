//! Wire shapes for the **Snippets** API category (personal + project).

use json_bourne::{FromJson, ToJson};

use crate::Id;

/// The author of a snippet. Includes `email`, which GitLab exposes on the
/// single-snippet responses but not always on listings.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct SnippetAuthor {
    pub id: Id,
    pub username: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub state: Option<String>,
    pub avatar_url: Option<String>,
    pub web_url: Option<String>,
    pub created_at: Option<String>,
}

/// A file entry within a snippet (present on multi-file snippets).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct SnippetFile {
    pub path: Option<String>,
    pub raw_url: Option<String>,
}

/// A snippet, personal (`project_id` null) or project-scoped.
///
/// Covers `GET /snippets`, `/snippets/all`, `/snippets/public`,
/// `/snippets/:id`, and the project equivalents. Fields only present on
/// some variants (repo URLs, `expires_at`, `files`) are optional.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct Snippet {
    pub id: Id,
    pub title: Option<String>,
    pub file_name: Option<String>,
    pub description: Option<String>,
    pub visibility: Option<String>,
    pub author: Option<SnippetAuthor>,
    pub project_id: Option<Id>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub expires_at: Option<String>,
    pub web_url: Option<String>,
    pub raw_url: Option<String>,
    pub ssh_url_to_repo: Option<String>,
    pub http_url_to_repo: Option<String>,
    pub repository_storage: Option<String>,
    pub imported: Option<bool>,
    pub imported_from: Option<String>,
    #[bourne(default)]
    pub files: Vec<SnippetFile>,
}

/// User-agent details for a snippet (`.../user_agent_detail`, admin only).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct SnippetUserAgentDetail {
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    #[bourne(default)]
    pub akismet_submitted: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_personal_snippet() {
        let json = r#"{
            "id": 1, "title": "test", "file_name": "add.rb",
            "description": "Ruby test snippet", "visibility": "private",
            "author": {"id": 1, "username": "john_smith", "email": "john@example.com",
                       "name": "John Smith", "state": "active"},
            "expires_at": null, "updated_at": "2012-06-28T10:52:04Z",
            "created_at": "2012-06-28T10:52:04Z", "project_id": null,
            "web_url": "http://example.com/snippets/1",
            "raw_url": "http://example.com/snippets/1/raw"
        }"#;
        let s: Snippet = parse_str(json).unwrap();
        assert_eq!(s.id, 1);
        assert!(s.project_id.is_none());
        assert_eq!(s.author.unwrap().email.as_deref(), Some("john@example.com"));
    }

    #[test]
    fn parse_snippet_with_files() {
        let json = r#"{
            "id": 113, "title": "Internal", "visibility": "internal", "project_id": 35,
            "file_name": "", "files": [{"path": "a.txt", "raw_url": "http://x/raw"}],
            "repository_storage": "default"
        }"#;
        let s: Snippet = parse_str(json).unwrap();
        assert_eq!(s.project_id, Some(Id::new(35)));
        assert_eq!(s.files.len(), 1);
        assert_eq!(s.files[0].path.as_deref(), Some("a.txt"));
    }
}
