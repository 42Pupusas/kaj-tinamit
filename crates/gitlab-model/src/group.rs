//! Wire shapes for the **Groups** and **Members** API categories.

use json_bourne::{FromJson, ToJson};

use crate::project::Visibility;

/// Another group a group is shared with
/// (`shared_with_groups[]` in a group response).
#[derive(Debug, FromJson, ToJson, Clone)]
#[bourne(deny_unknown_fields = false)]
pub struct SharedWithGroup {
    pub group_id: i64,
    pub group_name: Option<String>,
    pub group_full_path: Option<String>,
    #[bourne(default)]
    pub group_access_level: i32,
    pub expires_at: Option<String>,
}

/// A GitLab group (`GET /groups`, `GET /groups/:id`).
///
/// The deprecated inline `projects` / `shared_projects` arrays are omitted;
/// use the dedicated group-projects endpoints instead.
#[derive(Debug, FromJson, ToJson, Clone)]
#[bourne(deny_unknown_fields = false)]
pub struct GitlabGroup {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub visibility: Option<Visibility>,
    pub avatar_url: Option<String>,
    pub web_url: Option<String>,
    pub full_name: Option<String>,
    pub full_path: Option<String>,
    pub parent_id: Option<i64>,
    pub created_at: Option<String>,
    #[bourne(default)]
    pub request_access_enabled: bool,
    pub repository_storage: Option<String>,
    pub file_template_project_id: Option<i64>,
    #[bourne(default)]
    pub shared_with_groups: Vec<SharedWithGroup>,
}

/// A concise reference to who created a membership
/// (`created_by` in a member response).
#[derive(Debug, FromJson, ToJson, Clone)]
#[bourne(deny_unknown_fields = false)]
pub struct MemberCreatedBy {
    pub id: i64,
    pub username: Option<String>,
    pub name: Option<String>,
    pub web_url: Option<String>,
}

/// A group or project member
/// (`GET /groups/:id/members`, `GET /projects/:id/members`, and the
/// `/all` inherited variants).
///
/// `access_level` is GitLab's numeric role (10 Guest … 50 Owner); see
/// [`AccessLevel`] to interpret it.
#[derive(Debug, FromJson, ToJson, Clone)]
#[bourne(deny_unknown_fields = false)]
pub struct Member {
    pub id: i64,
    pub username: String,
    pub name: Option<String>,
    pub state: Option<String>,
    pub avatar_url: Option<String>,
    pub web_url: Option<String>,
    #[bourne(default)]
    pub access_level: i32,
    pub expires_at: Option<String>,
    pub created_at: Option<String>,
    pub created_by: Option<MemberCreatedBy>,
    /// Only visible to group owners for enterprise users.
    pub email: Option<String>,
    /// Present on the `/all` (inherited) endpoints: how the membership is
    /// derived — `direct` or `inherited`.
    pub membership_state: Option<String>,
}

/// GitLab's named access levels, mapped from the numeric `access_level`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AccessLevel {
    /// 0 — no access.
    NoAccess,
    /// 5 — minimal access.
    Minimal,
    /// 10 — Guest.
    Guest,
    /// 15 — Planner.
    Planner,
    /// 20 — Reporter.
    Reporter,
    /// 30 — Developer.
    Developer,
    /// 40 — Maintainer.
    Maintainer,
    /// 50 — Owner.
    Owner,
    /// Any other numeric value GitLab may define.
    Other(i32),
}

impl AccessLevel {
    /// Interpret GitLab's numeric access level.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::NoAccess,
            5 => Self::Minimal,
            10 => Self::Guest,
            15 => Self::Planner,
            20 => Self::Reporter,
            30 => Self::Developer,
            40 => Self::Maintainer,
            50 => Self::Owner,
            other => Self::Other(other),
        }
    }

    /// The numeric value GitLab uses for this level.
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        match self {
            Self::NoAccess => 0,
            Self::Minimal => 5,
            Self::Guest => 10,
            Self::Planner => 15,
            Self::Reporter => 20,
            Self::Developer => 30,
            Self::Maintainer => 40,
            Self::Owner => 50,
            Self::Other(v) => v,
        }
    }
}

impl Member {
    /// The member's role as a named [`AccessLevel`].
    #[must_use]
    pub const fn role(&self) -> AccessLevel {
        AccessLevel::from_raw(self.access_level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_group() {
        let json = r#"{
            "id": 4,
            "name": "Twitter",
            "path": "twitter",
            "visibility": "public",
            "full_path": "twitter",
            "parent_id": null,
            "shared_with_groups": [
                {"group_id": 28, "group_name": "H5bp", "group_access_level": 20, "expires_at": null}
            ]
        }"#;
        let g: GitlabGroup = parse_str(json).unwrap();
        assert_eq!(g.id, 4);
        assert_eq!(g.path, "twitter");
        assert_eq!(g.shared_with_groups.len(), 1);
        assert_eq!(g.shared_with_groups[0].group_id, 28);
    }

    #[test]
    fn parse_member_and_role() {
        let json = r#"{
            "id": 1,
            "username": "raymond_smith",
            "name": "Raymond Smith",
            "state": "active",
            "access_level": 30,
            "expires_at": "2012-10-22"
        }"#;
        let m: Member = parse_str(json).unwrap();
        assert_eq!(m.username, "raymond_smith");
        assert_eq!(m.role(), AccessLevel::Developer);
    }

    #[test]
    fn access_level_roundtrip() {
        for raw in [0, 5, 10, 15, 20, 30, 40, 50, 25] {
            assert_eq!(AccessLevel::from_raw(raw).as_raw(), raw);
        }
    }
}
