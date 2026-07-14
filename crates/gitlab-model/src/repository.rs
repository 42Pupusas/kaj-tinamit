//! Wire shapes for GitLab's **Repository** category: trees, blobs,
//! branches, tags, files, blame, contributors, comparisons, and merge base.

use json_bourne::{FromJson, Lexer, ToJson};

/// The kind of a repository tree entry.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
pub enum TreeEntryType {
    /// A directory.
    Tree,
    /// A file.
    Blob,
    /// A commit (git submodule).
    Commit,
    /// Any value GitLab may add later.
    Unknown,
}

impl<'input> FromJson<'input> for TreeEntryType {
    fn from_lex(lex: &mut Lexer<'input>) -> Result<Self, json_bourne::Error> {
        let s = String::from_lex(lex)?;
        Ok(match s.as_str() {
            "tree" => Self::Tree,
            "blob" => Self::Blob,
            "commit" => Self::Commit,
            _ => Self::Unknown,
        })
    }
}

impl ToJson for TreeEntryType {
    fn write_json<W: json_bourne::JsonWrite + ?Sized>(&self, w: &mut W) -> Result<(), W::Error> {
        let s = match self {
            Self::Tree => "tree",
            Self::Blob => "blob",
            Self::Commit => "commit",
            Self::Unknown => "unknown",
        };
        s.write_json(w)
    }
}

/// An entry in a repository tree listing
/// (`GET /projects/:id/repository/tree`).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct TreeEntry {
    pub id: String,
    pub name: String,
    #[bourne(rename = "type")]
    pub entry_type: TreeEntryType,
    pub path: String,
    pub mode: String,
}

/// A blob's metadata and Base64-encoded content
/// (`GET /projects/:id/repository/blobs/:sha`).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct Blob {
    pub size: i64,
    pub encoding: String,
    /// Base64-encoded blob content.
    pub content: String,
    pub sha: String,
}

/// A repository contributor
/// (`GET /projects/:id/repository/contributors`).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct Contributor {
    pub name: String,
    pub email: String,
    #[bourne(default)]
    pub commits: i64,
    #[bourne(default)]
    pub additions: i64,
    #[bourne(default)]
    pub deletions: i64,
}

/// A lightweight commit as embedded in branch, tag, and merge-base
/// responses. Fields GitLab sometimes omits (like `web_url` on tags) are
/// optional so a single type covers every ref context.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct RefCommit {
    pub id: String,
    pub short_id: String,
    pub title: Option<String>,
    pub message: Option<String>,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub authored_date: Option<String>,
    pub committer_name: Option<String>,
    pub committer_email: Option<String>,
    pub committed_date: Option<String>,
    pub created_at: Option<String>,
    #[bourne(default)]
    pub parent_ids: Vec<String>,
    pub web_url: Option<String>,
}

/// A repository branch
/// (`GET /projects/:id/repository/branches[/:branch]`).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct Branch {
    pub name: String,
    #[bourne(default)]
    pub merged: bool,
    #[bourne(default)]
    pub protected: bool,
    #[bourne(default)]
    pub default: bool,
    #[bourne(default)]
    pub developers_can_push: bool,
    #[bourne(default)]
    pub developers_can_merge: bool,
    #[bourne(default)]
    pub can_push: bool,
    pub web_url: Option<String>,
    pub commit: Option<RefCommit>,
}

/// The release object embedded in a tag.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct TagRelease {
    pub tag_name: Option<String>,
    pub description: Option<String>,
}

/// A repository tag
/// (`GET /projects/:id/repository/tags[/:tag_name]`).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct Tag {
    pub name: String,
    pub message: Option<String>,
    pub target: Option<String>,
    #[bourne(default)]
    pub protected: bool,
    pub created_at: Option<String>,
    pub commit: Option<RefCommit>,
    pub release: Option<TagRelease>,
}

/// A file's metadata and Base64-encoded content
/// (`GET /projects/:id/repository/files/:file_path`).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct RepositoryFile {
    pub file_name: String,
    pub file_path: String,
    pub size: i64,
    pub encoding: String,
    /// Base64-encoded file content.
    pub content: String,
    pub content_sha256: Option<String>,
    #[bourne(rename = "ref")]
    pub ref_name: Option<String>,
    pub blob_id: Option<String>,
    pub commit_id: Option<String>,
    pub last_commit_id: Option<String>,
    #[bourne(default)]
    pub execute_filemode: bool,
}

/// The result of a file create/update/delete
/// (`POST`/`PUT`/`DELETE /projects/:id/repository/files/:file_path`).
///
/// GitLab echoes just the file path and the branch the change landed on.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct FileMutationResult {
    pub file_path: String,
    pub branch: String,
}

/// The commit portion of a blame range. Blame commits omit `web_url` and
/// `short_id`, so this is a distinct, minimal shape.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct BlameCommit {
    pub id: String,
    pub message: Option<String>,
    #[bourne(default)]
    pub parent_ids: Vec<String>,
    pub authored_date: Option<String>,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub committed_date: Option<String>,
    pub committer_name: Option<String>,
    pub committer_email: Option<String>,
}

/// A blame range: a run of consecutive lines attributed to one commit
/// (`GET /projects/:id/repository/files/:file_path/blame`).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct BlameRange {
    pub commit: BlameCommit,
    #[bourne(default)]
    pub lines: Vec<String>,
}

/// Generated changelog data
/// (`GET /projects/:id/repository/changelog`).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct Changelog {
    /// Generated changelog in Markdown format.
    pub notes: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_tree_entry() {
        let json = r#"{
            "id": "a1e8f8d7",
            "name": "html",
            "type": "tree",
            "path": "files/html",
            "mode": "040000"
        }"#;
        let e: TreeEntry = parse_str(json).unwrap();
        assert_eq!(e.name, "html");
        assert_eq!(e.entry_type, TreeEntryType::Tree);
    }

    #[test]
    fn parse_branch_with_commit() {
        let json = r#"{
            "name": "main",
            "merged": false,
            "protected": true,
            "default": true,
            "can_push": true,
            "commit": { "id": "7b5c3cc8", "short_id": "7b5c3cc" }
        }"#;
        let b: Branch = parse_str(json).unwrap();
        assert_eq!(b.name, "main");
        assert!(b.default);
        assert_eq!(b.commit.unwrap().short_id, "7b5c3cc");
    }

    #[test]
    fn parse_lightweight_tag() {
        // Lightweight tag: message/release null, minimal commit.
        let json = r#"{
            "name": "v5.0.0",
            "message": null,
            "target": "60a8ff03",
            "release": null,
            "protected": false,
            "commit": { "id": "60a8ff03", "short_id": "60a8ff03" }
        }"#;
        let t: Tag = parse_str(json).unwrap();
        assert_eq!(t.name, "v5.0.0");
        assert!(t.message.is_none());
        assert!(t.release.is_none());
    }
}
