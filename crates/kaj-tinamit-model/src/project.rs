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

impl Visibility {
    /// The wire spelling GitLab uses for this level.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Internal => "internal",
            Self::Private => "private",
            Self::Unknown => "unknown",
        }
    }
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
        self.as_str().write_json(w)
    }
}

/// Who may use a project feature (`issues_access_level`,
/// `builds_access_level`, …). Unknown values fall back to
/// [`FeatureAccess::Unknown`].
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[non_exhaustive]
pub enum FeatureAccess {
    Disabled,
    Private,
    Enabled,
    Public,
    Unknown,
}

impl FeatureAccess {
    /// The wire spelling GitLab uses for this level.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Private => "private",
            Self::Enabled => "enabled",
            Self::Public => "public",
            Self::Unknown => "unknown",
        }
    }
}

impl<'input> FromJson<'input> for FeatureAccess {
    fn from_lex(lex: &mut Lexer<'input>) -> Result<Self, json_bourne::Error> {
        let s = String::from_lex(lex)?;
        Ok(match s.as_str() {
            "disabled" => Self::Disabled,
            "private" => Self::Private,
            "enabled" => Self::Enabled,
            "public" => Self::Public,
            _ => Self::Unknown,
        })
    }
}

impl ToJson for FeatureAccess {
    fn write_json<W: json_bourne::JsonWrite + ?Sized>(&self, w: &mut W) -> Result<(), W::Error> {
        self.as_str().write_json(w)
    }
}

/// How merge requests are merged into a project. Unknown values fall back
/// to [`MergeMethod::Unknown`].
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[non_exhaustive]
pub enum MergeMethod {
    Merge,
    RebaseMerge,
    FastForward,
    Unknown,
}

impl MergeMethod {
    /// The wire spelling GitLab uses for this method.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Merge => "merge",
            Self::RebaseMerge => "rebase_merge",
            Self::FastForward => "ff",
            Self::Unknown => "unknown",
        }
    }
}

impl<'input> FromJson<'input> for MergeMethod {
    fn from_lex(lex: &mut Lexer<'input>) -> Result<Self, json_bourne::Error> {
        let s = String::from_lex(lex)?;
        Ok(match s.as_str() {
            "merge" => Self::Merge,
            "rebase_merge" => Self::RebaseMerge,
            "ff" => Self::FastForward,
            _ => Self::Unknown,
        })
    }
}

impl ToJson for MergeMethod {
    fn write_json<W: json_bourne::JsonWrite + ?Sized>(&self, w: &mut W) -> Result<(), W::Error> {
        self.as_str().write_json(w)
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

/// A concise reference to another project (`forked_from_project`).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct ProjectRef {
    pub id: Id,
    pub name: Option<String>,
    pub name_with_namespace: Option<String>,
    pub path: Option<String>,
    pub path_with_namespace: Option<String>,
    pub default_branch: Option<String>,
    pub web_url: Option<String>,
}

/// One access grant inside [`ProjectPermissions`].
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct AccessGrant {
    #[bourne(default)]
    pub access_level: i32,
    pub notification_level: Option<i32>,
}

/// The caller's own access to a project, direct and through its group.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct ProjectPermissions {
    pub project_access: Option<AccessGrant>,
    pub group_access: Option<AccessGrant>,
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
    pub default_branch: Option<String>,
    pub web_url: Option<String>,
    pub http_url_to_repo: Option<String>,
    pub ssh_url_to_repo: Option<String>,
    pub readme_url: Option<String>,
    pub avatar_url: Option<String>,
    #[bourne(default)]
    pub topics: Vec<String>,
    #[bourne(default)]
    pub star_count: i32,
    #[bourne(default)]
    pub forks_count: i32,
    #[bourne(default)]
    pub empty_repo: bool,
    pub ci_config_path: Option<String>,
    pub merge_method: Option<MergeMethod>,
    pub issues_access_level: Option<FeatureAccess>,
    pub merge_requests_access_level: Option<FeatureAccess>,
    pub wiki_access_level: Option<FeatureAccess>,
    pub builds_access_level: Option<FeatureAccess>,
    pub snippets_access_level: Option<FeatureAccess>,
    pub container_registry_access_level: Option<FeatureAccess>,
    pub forked_from_project: Option<ProjectRef>,
    pub permissions: Option<ProjectPermissions>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_project_with_urls_and_permissions() {
        let json = r#"{
            "id": 3,
            "name": "Diaspora Project Site",
            "default_branch": "main",
            "web_url": "https://example.com/diaspora/diaspora-project-site",
            "ssh_url_to_repo": "git@example.com:diaspora/diaspora-project-site.git",
            "http_url_to_repo": "https://example.com/diaspora/diaspora-project-site.git",
            "topics": ["example", "disapora project"],
            "star_count": 4,
            "forks_count": 1,
            "empty_repo": false,
            "merge_method": "ff",
            "issues_access_level": "enabled",
            "builds_access_level": "disabled",
            "namespace": {"id": 3, "name": "Diaspora", "kind": "group"},
            "forked_from_project": {"id": 1, "path_with_namespace": "upstream/site"},
            "permissions": {
                "project_access": {"access_level": 10, "notification_level": 3},
                "group_access": null
            }
        }"#;
        let p: GitlabProject = parse_str(json).unwrap();
        assert_eq!(p.default_branch.as_deref(), Some("main"));
        assert_eq!(p.topics.len(), 2);
        assert_eq!(p.forks_count, 1);
        assert_eq!(p.merge_method, Some(MergeMethod::FastForward));
        assert_eq!(p.builds_access_level, Some(FeatureAccess::Disabled));
        assert_eq!(p.forked_from_project.unwrap().id, 1);
        let access = p.permissions.unwrap().project_access.unwrap();
        assert_eq!(access.access_level, 10);
    }

    #[test]
    fn unknown_feature_access_and_merge_method_fall_back() {
        let json = r#"{"id":1,"namespace":{"id":1},"merge_method":"squash_only","wiki_access_level":"secret"}"#;
        let p: GitlabProject = parse_str(json).unwrap();
        assert_eq!(p.merge_method, Some(MergeMethod::Unknown));
        assert_eq!(p.wiki_access_level, Some(FeatureAccess::Unknown));
    }
}
