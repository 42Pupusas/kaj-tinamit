//! Wiki-page wire shapes.

use json_bourne::{FromJson, Lexer, ToJson};

/// A wiki page's markup format. Unknown values fall back to
/// [`WikiFormat::Unknown`] so parsing never fails on a format GitLab adds
/// later.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[non_exhaustive]
pub enum WikiFormat {
    Markdown,
    Rdoc,
    Asciidoc,
    Org,
    Unknown,
}

impl<'input> FromJson<'input> for WikiFormat {
    fn from_lex(lex: &mut Lexer<'input>) -> Result<Self, json_bourne::Error> {
        let s = String::from_lex(lex)?;
        Ok(match s.as_str() {
            "markdown" => Self::Markdown,
            "rdoc" => Self::Rdoc,
            "asciidoc" => Self::Asciidoc,
            "org" => Self::Org,
            _ => Self::Unknown,
        })
    }
}

impl ToJson for WikiFormat {
    fn write_json<W: json_bourne::JsonWrite + ?Sized>(&self, w: &mut W) -> Result<(), W::Error> {
        let s = match self {
            Self::Markdown => "markdown",
            Self::Rdoc => "rdoc",
            Self::Asciidoc => "asciidoc",
            Self::Org => "org",
            Self::Unknown => "unknown",
        };
        s.write_json(w)
    }
}

/// A full wiki page (list entries omit `content`).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
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
#[non_exhaustive]
pub struct WikiPageList {
    pub slug: String,
    pub title: String,
}

#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct WikiAttachment {
    pub file_name: String,
    pub file_path: String,
    pub branch: String,
    pub link: WikiAttachmentLink,
}

#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct WikiAttachmentLink {
    pub url: String,
    pub markdown: String,
}
