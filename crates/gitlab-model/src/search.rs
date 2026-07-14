//! Wire shapes for the **Search** API category. Search is scoped: each
//! scope (`projects`, `issues`, `merge_requests`, `users`, `commits`,
//! `blobs`, `wiki_blobs`, `milestones`, …) returns rows of a different
//! type. The typed endpoint methods deserialize into the matching existing
//! model; this module only adds the blob shape that has no home elsewhere.

use json_bourne::{FromJson, ToJson};

/// A code/wiki search hit (`blobs` and `wiki_blobs` scopes). Returns the
/// matching file, the matched line range, and a snippet of surrounding data.
#[derive(Debug, FromJson, ToJson, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct SearchBlob {
    pub basename: Option<String>,
    /// Repository-relative path of the matching file.
    pub filename: Option<String>,
    pub path: Option<String>,
    /// The matched data snippet (may span several lines).
    pub data: Option<String>,
    /// 1-based line where the snippet begins.
    pub startline: Option<i64>,
    pub project_id: Option<i64>,
    /// The ref the match was found on.
    #[bourne(rename = "ref")]
    pub ref_name: Option<String>,
    pub id: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_search_blob() {
        let json = r#"{
            "basename": "main",
            "data": "fn main() {}\n",
            "path": "src/main.rs",
            "filename": "src/main.rs",
            "id": null,
            "ref": "main",
            "startline": 1,
            "project_id": 6
        }"#;
        let b: SearchBlob = parse_str(json).unwrap();
        assert_eq!(b.filename.as_deref(), Some("src/main.rs"));
        assert_eq!(b.startline, Some(1));
        assert_eq!(b.ref_name.as_deref(), Some("main"));
    }
}
