//! Wire shapes for the governance categories: **Protected branches**,
//! **Protected tags**, and **Protected environments** (read side).

use json_bourne::{FromJson, ToJson};

/// One access rule inside a protected-branch/tag entry (who may push, merge,
/// or deploy). GitLab reports `access_level` as an integer plus a
/// human-readable description.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct AccessRule {
    pub id: Option<i64>,
    pub access_level: Option<i64>,
    pub access_level_description: Option<String>,
    pub user_id: Option<i64>,
    pub group_id: Option<i64>,
}

/// A protected branch and its push/merge access rules.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct ProtectedBranch {
    pub id: Option<i64>,
    pub name: String,
    #[bourne(default)]
    pub push_access_levels: Vec<AccessRule>,
    #[bourne(default)]
    pub merge_access_levels: Vec<AccessRule>,
    #[bourne(default)]
    pub allow_force_push: bool,
    #[bourne(default)]
    pub code_owner_approval_required: bool,
}

/// A protected tag and its create access rules.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct ProtectedTag {
    pub name: String,
    #[bourne(default)]
    pub create_access_levels: Vec<AccessRule>,
}

/// A protected environment and its deploy access rules.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct ProtectedEnvironment {
    pub name: String,
    #[bourne(default)]
    pub deploy_access_levels: Vec<AccessRule>,
    pub required_approval_count: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_protected_branch() {
        let json = r#"{
            "id": 1,
            "name": "main",
            "push_access_levels": [
                {"access_level": 40, "access_level_description": "Maintainers"}
            ],
            "merge_access_levels": [
                {"access_level": 40, "access_level_description": "Maintainers"}
            ],
            "allow_force_push": false,
            "code_owner_approval_required": false
        }"#;
        let p: ProtectedBranch = parse_str(json).unwrap();
        assert_eq!(p.name, "main");
        assert_eq!(p.push_access_levels.len(), 1);
        assert_eq!(p.push_access_levels[0].access_level, Some(40));
    }
}
