//! Project-related wire shapes.

use crate::Id;
use json_bourne::{FromJson, Lexer, ToJson};

/// A project's visibility level. Unknown values fall back to
/// [`Visibility::Unknown`] so parsing never fails on a level GitLab adds later.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[non_exhaustive]
pub enum Visibility {
    Public,
    Internal,
    Private,
    Unknown,
}

impl<'input> FromJson<'input> for Visibility {
    fn from_lex(lex: &mut Lexer<'input>) -> Result<Self, json_bourne::Error> {
        let s = String::from_lex(lex)?;
        Ok(match s.as_str() {
            "public" => Self::Public,
            "internal" => Self::Internal,
            "private" => Self::Private,
            _ => Self::Unknown,
        })
    }
}

impl ToJson for Visibility {
    fn write_json<W: json_bourne::JsonWrite + ?Sized>(&self, w: &mut W) -> Result<(), W::Error> {
        let s = match self {
            Self::Public => "public",
            Self::Internal => "internal",
            Self::Private => "private",
            Self::Unknown => "unknown",
        };
        s.write_json(w)
    }
}

/// The kind of namespace that owns a project. Unknown values fall back to
/// [`NamespaceKind::Unknown`].
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[non_exhaustive]
pub enum NamespaceKind {
    User,
    Group,
    Unknown,
}

impl<'input> FromJson<'input> for NamespaceKind {
    fn from_lex(lex: &mut Lexer<'input>) -> Result<Self, json_bourne::Error> {
        let s = String::from_lex(lex)?;
        Ok(match s.as_str() {
            "user" => Self::User,
            "group" => Self::Group,
            _ => Self::Unknown,
        })
    }
}

impl ToJson for NamespaceKind {
    fn write_json<W: json_bourne::JsonWrite + ?Sized>(&self, w: &mut W) -> Result<(), W::Error> {
        let s = match self {
            Self::User => "user",
            Self::Group => "group",
            Self::Unknown => "unknown",
        };
        s.write_json(w)
    }
}

#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct Namespace {
    pub id: Id,
    pub name: Option<String>,
    pub path: Option<String>,
    pub kind: Option<NamespaceKind>,
    pub full_path: Option<String>,
    pub avatar_url: Option<String>,
    pub web_url: Option<String>,
}

#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct GitlabProject {
    pub id: Id,
    pub description: Option<String>,
    pub visibility: Option<Visibility>,
    pub name: Option<String>,
    pub name_with_namespace: Option<String>,
    pub path: Option<String>,
    pub path_with_namespace: Option<String>,
    #[bourne(default)]
    pub open_issues_count: i32,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub last_activity_at: Option<String>,
    pub creator_id: Option<Id>,
    pub namespace: Namespace,
    #[bourne(default)]
    pub archived: bool,
    pub repository_storage: Option<String>,
}
