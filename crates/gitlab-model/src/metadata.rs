//! Wire shapes for the **Metadata** endpoint (`GET /metadata`) and the
//! **Namespaces** listing (`GET /namespaces`).

use json_bourne::{FromJson, ToJson};

/// Details of the KAS (Kubernetes Agent Server), part of `/metadata`.
#[derive(Debug, FromJson, ToJson, Clone, Default)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct KasMetadata {
    #[bourne(default)]
    pub enabled: bool,
    pub version: Option<String>,
    pub external_url: Option<String>,
}

/// Instance metadata: version, revision, and enterprise flag.
#[derive(Debug, FromJson, ToJson, Clone, Default)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct Metadata {
    pub version: Option<String>,
    pub revision: Option<String>,
    #[bourne(default)]
    pub enterprise: bool,
    #[bourne(default)]
    pub kas: KasMetadata,
}

/// A namespace (user or group) as returned by `GET /namespaces`. Richer than
/// the embedded [`Namespace`](crate::Namespace) project field — it carries
/// membership and billing-plan hints.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct NamespaceListing {
    pub id: i64,
    pub name: Option<String>,
    pub path: Option<String>,
    pub kind: Option<String>,
    pub full_path: Option<String>,
    pub parent_id: Option<i64>,
    pub avatar_url: Option<String>,
    pub web_url: Option<String>,
    pub billable_members_count: Option<i64>,
    pub plan: Option<String>,
    pub trial_ends_on: Option<String>,
    #[bourne(default)]
    pub trial: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_metadata() {
        let json = r#"{
            "version": "17.0.0",
            "revision": "abcdef",
            "enterprise": true,
            "kas": {"enabled": true, "version": "17.0.0", "external_url": "wss://kas.example.com"}
        }"#;
        let m: Metadata = parse_str(json).unwrap();
        assert_eq!(m.version.as_deref(), Some("17.0.0"));
        assert!(m.enterprise);
        assert!(m.kas.enabled);
    }

    #[test]
    fn parse_namespace_listing() {
        let json = r#"{
            "id": 2,
            "name": "group1",
            "path": "group1",
            "kind": "group",
            "full_path": "group1",
            "billable_members_count": 2
        }"#;
        let n: NamespaceListing = parse_str(json).unwrap();
        assert_eq!(n.id, 2);
        assert_eq!(n.kind.as_deref(), Some("group"));
    }
}
