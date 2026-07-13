//! User-related wire shapes.

use json_bourne::{FromJson, ToJson};

#[derive(Debug, FromJson, ToJson, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[bourne(rename_all = "snake_case")]
pub enum UserState {
    Active,
    Blocked,
    Deactivated,
    BlockedPendingApproval,
}

#[derive(Debug, FromJson, ToJson, PartialEq, Hash, Eq, Clone)]
#[bourne(deny_unknown_fields = false)]
pub struct GitlabUser {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub state: UserState,
    pub locked: bool,
    pub avatar_url: String,
    pub web_url: String,
}
