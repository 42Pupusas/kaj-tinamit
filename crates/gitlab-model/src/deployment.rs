//! Wire shapes for the **Environments** and **Deployments** API categories.

use json_bourne::{FromJson, ToJson};

use crate::Id;

/// The user who created a deployment or triggered an environment (concise
/// subset).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct DeploymentUser {
    pub id: Id,
    pub username: Option<String>,
    pub name: Option<String>,
    pub state: Option<String>,
    pub avatar_url: Option<String>,
    pub web_url: Option<String>,
}

/// The commit embedded in a deployable (concise subset).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct DeployableCommit {
    pub id: Option<String>,
    pub short_id: Option<String>,
    pub title: Option<String>,
    pub message: Option<String>,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub created_at: Option<String>,
}

/// The pipeline embedded in a deployable (concise subset).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct DeployablePipeline {
    pub id: Id,
    #[bourne(rename = "ref")]
    pub ref_name: Option<String>,
    pub sha: Option<String>,
    pub status: Option<String>,
    pub web_url: Option<String>,
}

/// The CI job that performed a deployment (the `deployable` object). This is
/// a job-shaped subset with the deployment-relevant fields.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct Deployable {
    pub id: Id,
    pub status: Option<String>,
    pub stage: Option<String>,
    pub name: Option<String>,
    #[bourne(rename = "ref")]
    pub ref_name: Option<String>,
    #[bourne(default)]
    pub tag: bool,
    pub created_at: Option<String>,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub user: Option<DeploymentUser>,
    pub commit: Option<DeployableCommit>,
    pub pipeline: Option<DeployablePipeline>,
}

/// The environment reference embedded in a deployment.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct EnvironmentRef {
    pub id: Id,
    pub name: Option<String>,
    pub external_url: Option<String>,
}

/// A deployment (`GET /projects/:id/deployments[/:deployment_id]`).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct Deployment {
    pub id: Id,
    #[bourne(default)]
    pub iid: Id,
    #[bourne(rename = "ref")]
    pub ref_name: Option<String>,
    pub sha: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub user: Option<DeploymentUser>,
    pub environment: Option<EnvironmentRef>,
    pub deployable: Option<Deployable>,
}

/// A deployment as embedded in an environment's `last_deployment` field.
/// Distinct from [`Deployment`] only in that every field is optional and it
/// carries no environment back-reference.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct LastDeployment {
    pub id: Id,
    #[bourne(default)]
    pub iid: Id,
    #[bourne(rename = "ref")]
    pub ref_name: Option<String>,
    pub sha: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<String>,
    pub user: Option<DeploymentUser>,
    pub deployable: Option<Deployable>,
}

/// A project environment (`GET /projects/:id/environments[/:id]`).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct Environment {
    pub id: Id,
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub external_url: Option<String>,
    pub state: Option<String>,
    pub tier: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub auto_stop_at: Option<String>,
    pub auto_stop_setting: Option<String>,
    pub kubernetes_namespace: Option<String>,
    pub flux_resource_path: Option<String>,
    pub last_deployment: Option<LastDeployment>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_environment() {
        let json = r#"{
            "id": 1, "name": "review/fix-foo", "slug": "review-fix-foo-dfjre3",
            "description": "This is review environment", "state": "available",
            "tier": "development", "external_url": "https://x.example.com",
            "created_at": "2019-05-25T18:55:13.252Z", "auto_stop_setting": "always"
        }"#;
        let e: Environment = parse_str(json).unwrap();
        assert_eq!(e.id, 1);
        assert_eq!(e.name, "review/fix-foo");
        assert_eq!(e.tier.as_deref(), Some("development"));
    }

    #[test]
    fn parse_deployment_with_deployable() {
        let json = r#"{
            "id": 42, "iid": 2, "ref": "main", "sha": "a91957a8", "status": "success",
            "created_at": "2016-08-11T11:32:35.444Z",
            "environment": {"id": 9, "name": "production", "external_url": "https://x"},
            "deployable": {"id": 664, "status": "success", "stage": "deploy", "name": "deploy",
                           "ref": "main", "tag": false}
        }"#;
        let d: Deployment = parse_str(json).unwrap();
        assert_eq!(d.id, 42);
        assert_eq!(d.environment.unwrap().name.as_deref(), Some("production"));
        assert_eq!(d.deployable.unwrap().id, 664);
    }
}
