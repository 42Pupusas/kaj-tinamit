//! Wiki-page wire shapes.

use json_bourne::{FromJson, ToJson};

#[derive(Debug, FromJson, ToJson, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(rename_all = "lowercase")]
pub enum WikiFormat {
    Markdown,
    Rdoc,
    Asciidoc,
    Org,
}

/// A full wiki page (list entries omit `content`).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct WikiPage {
    pub slug: String,
    pub title: String,
    pub content: Option<String>,
    pub format: WikiFormat,
    pub encoding: Option<String>,
}

/// A wiki page as it appears in a listing (no content).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct WikiPageList {
    pub slug: String,
    pub title: String,
}

#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct WikiAttachment {
    pub file_name: String,
    pub file_path: String,
    pub branch: String,
    pub link: WikiAttachmentLink,
}

#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct WikiAttachmentLink {
    pub url: String,
    pub markdown: String,
}
