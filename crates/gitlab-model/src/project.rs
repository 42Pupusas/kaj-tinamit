//! Project-related wire shapes.

use json_bourne::{FromJson, ToJson};

#[derive(Debug, FromJson, ToJson, PartialEq, Eq, Clone, Copy)]
#[bourne(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Internal,
    Private,
}

#[derive(Debug, FromJson, ToJson, PartialEq, Eq, Clone, Copy)]
#[bourne(rename_all = "lowercase")]
pub enum NamespaceKind {
    User,
    Group,
}

#[derive(Debug, FromJson, ToJson, Clone)]
#[bourne(deny_unknown_fields = false)]
pub struct Namespace {
    pub id: i32,
    pub name: Option<String>,
    pub path: Option<String>,
    pub kind: Option<NamespaceKind>,
    pub full_path: Option<String>,
    pub avatar_url: Option<String>,
    pub web_url: Option<String>,
}

#[derive(Debug, FromJson, ToJson, Clone)]
#[bourne(deny_unknown_fields = false)]
pub struct GitlabProject {
    pub id: i32,
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
    pub creator_id: Option<i32>,
    pub namespace: Namespace,
    #[bourne(default)]
    pub archived: bool,
    pub repository_storage: Option<String>,
}
