//! Wire shapes for the **Deploy keys** and **Deploy tokens** API categories.

use json_bourne::{FromJson, ToJson};

use crate::Id;

/// A project reference embedded in an instance-level deploy key's
/// access lists.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct DeployKeyProject {
    pub id: Id,
    pub description: Option<String>,
    pub name: Option<String>,
    pub name_with_namespace: Option<String>,
    pub path: Option<String>,
    pub path_with_namespace: Option<String>,
    pub created_at: Option<String>,
}

/// A deploy key (`GET /projects/:id/deploy_keys[/:key_id]`, `/deploy_keys`).
///
/// `can_push` is present on project-scoped responses; the
/// `projects_with_*_access` arrays only on the instance-level listing.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct DeployKey {
    pub id: Id,
    pub title: Option<String>,
    pub key: Option<String>,
    pub fingerprint: Option<String>,
    pub fingerprint_sha256: Option<String>,
    pub created_at: Option<String>,
    pub expires_at: Option<String>,
    pub can_push: Option<bool>,
    #[bourne(default)]
    pub projects_with_write_access: Vec<DeployKeyProject>,
    #[bourne(default)]
    pub projects_with_readonly_access: Vec<DeployKeyProject>,
}

/// A deploy token (`GET /projects|groups/:id/deploy_tokens[/:token_id]`).
///
/// The secret `token` value is only returned on creation, never on reads, so
/// it is intentionally not modeled here.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct DeployToken {
    pub id: Id,
    pub name: Option<String>,
    pub username: Option<String>,
    pub expires_at: Option<String>,
    #[bourne(default)]
    pub revoked: bool,
    #[bourne(default)]
    pub expired: bool,
    #[bourne(default)]
    pub scopes: Vec<String>,
}

/// A deploy token as returned by **creation**
/// (`POST /projects|groups/:id/deploy_tokens`).
///
/// Unlike [`DeployToken`], this carries the secret `token` value, which
/// GitLab returns exactly once at creation time and never again.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct CreatedDeployToken {
    pub id: Id,
    pub name: Option<String>,
    pub username: Option<String>,
    pub expires_at: Option<String>,
    #[bourne(default)]
    pub revoked: bool,
    #[bourne(default)]
    pub expired: bool,
    #[bourne(default)]
    pub scopes: Vec<String>,
    /// The secret token value — present only on creation.
    pub token: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_project_deploy_key() {
        let json = r#"{
            "id": 1, "title": "Public key", "key": "ssh-rsa AAAA...",
            "fingerprint": "4a:9d:64:15", "fingerprint_sha256": "SHA256:Jrs3",
            "created_at": "2013-10-02T10:12:29Z", "expires_at": null, "can_push": false
        }"#;
        let k: DeployKey = parse_str(json).unwrap();
        assert_eq!(k.id, 1);
        assert_eq!(k.can_push, Some(false));
        assert!(k.projects_with_write_access.is_empty());
    }

    #[test]
    fn parse_deploy_token() {
        let json = r#"{
            "id": 1, "name": "MyToken", "username": "gitlab+deploy-token-1",
            "expires_at": "2020-02-14T00:00:00.000Z", "revoked": false, "expired": false,
            "scopes": ["read_repository", "read_registry"]
        }"#;
        let t: DeployToken = parse_str(json).unwrap();
        assert_eq!(t.name.as_deref(), Some("MyToken"));
        assert_eq!(t.scopes.len(), 2);
        assert!(!t.revoked);
    }
}
