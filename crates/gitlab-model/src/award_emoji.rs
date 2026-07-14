//! Wire shapes for the **Emoji reactions** API category (`award_emoji` on
//! issues, merge requests, and snippets, plus their notes).

use json_bourne::{FromJson, ToJson};

use crate::issue::Author;

/// An emoji reaction awarded to an issue, MR, snippet, or note.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct AwardEmoji {
    pub id: i64,
    /// The emoji name, e.g. `thumbsup`, `rocket`.
    pub name: Option<String>,
    pub user: Option<Author>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    /// The awardable's type (`Issue`, `MergeRequest`, `Snippet`, `Note`).
    pub awardable_type: Option<String>,
    pub awardable_id: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_award_emoji() {
        let json = r#"{
            "id": 4,
            "name": "1234",
            "awardable_type": "Issue",
            "awardable_id": 80,
            "created_at": "2016-06-15T10:09:34.206Z"
        }"#;
        let a: AwardEmoji = parse_str(json).unwrap();
        assert_eq!(a.id, 4);
        assert_eq!(a.awardable_type.as_deref(), Some("Issue"));
    }
}
