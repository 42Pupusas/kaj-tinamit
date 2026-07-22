//! User-related wire shapes.

use crate::Id;
use json_bourne::{FromJson, Lexer, ToJson};

/// A user's account state. Unknown values fall back to [`UserState::Unknown`]
/// so parsing never fails on a state GitLab adds later.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[non_exhaustive]
pub enum UserState {
    Active,
    Blocked,
    Deactivated,
    BlockedPendingApproval,
    LdapBlocked,
    Unknown,
}

impl<'input> FromJson<'input> for UserState {
    fn from_lex(lex: &mut Lexer<'input>) -> Result<Self, json_bourne::Error> {
        let s = String::from_lex(lex)?;
        Ok(match s.as_str() {
            "active" => Self::Active,
            "blocked" => Self::Blocked,
            "deactivated" => Self::Deactivated,
            "blocked_pending_approval" => Self::BlockedPendingApproval,
            "ldap_blocked" => Self::LdapBlocked,
            _ => Self::Unknown,
        })
    }
}

impl ToJson for UserState {
    fn write_json<W: json_bourne::JsonWrite + ?Sized>(&self, w: &mut W) -> Result<(), W::Error> {
        let s = match self {
            Self::Active => "active",
            Self::Blocked => "blocked",
            Self::Deactivated => "deactivated",
            Self::BlockedPendingApproval => "blocked_pending_approval",
            Self::LdapBlocked => "ldap_blocked",
            Self::Unknown => "unknown",
        };
        s.write_json(w)
    }
}

#[derive(Debug, FromJson, ToJson, PartialEq, Hash, Eq, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct GitlabUser {
    pub id: Id,
    pub username: String,
    pub name: String,
    pub state: UserState,
    pub locked: bool,
    pub avatar_url: String,
    pub web_url: String,
}
