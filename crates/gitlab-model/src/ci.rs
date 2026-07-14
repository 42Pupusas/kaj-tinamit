//! Wire shapes for the **Pipelines** and **Jobs** (CI/CD) API categories.
//!
//! Note: a lightweight [`Pipeline`](crate::merge_request::Pipeline) already
//! exists for merge-request payloads. The types here are the fuller CI
//! shapes, prefixed to avoid collision.

use json_bourne::{FromJson, Lexer, ToJson};

/// A proptest strategy yielding `Option<f64>` restricted to finite values.
///
/// GitLab durations/coverage are always finite; excluding NaN/inf keeps the
/// JSON round-trip well-defined (those have no JSON representation).
#[cfg(feature = "proptest")]
fn finite_opt_f64() -> impl proptest::strategy::Strategy<Value = Option<f64>> {
    proptest::option::of(-1e12f64..1e12f64)
}

/// The status of a pipeline or job. A single closed-ish set shared by both;
/// unknown values fall back to [`CiStatus::Unknown`] so parsing never fails
/// on a status GitLab adds later.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
pub enum CiStatus {
    Created,
    WaitingForResource,
    Preparing,
    Pending,
    Running,
    Success,
    Failed,
    Canceled,
    Canceling,
    Skipped,
    Manual,
    Scheduled,
    WaitingForCallback,
    Unknown,
}

impl<'input> FromJson<'input> for CiStatus {
    fn from_lex(lex: &mut Lexer<'input>) -> Result<Self, json_bourne::Error> {
        let s = String::from_lex(lex)?;
        Ok(match s.as_str() {
            "created" => Self::Created,
            "waiting_for_resource" => Self::WaitingForResource,
            "preparing" => Self::Preparing,
            "pending" => Self::Pending,
            "running" => Self::Running,
            "success" => Self::Success,
            "failed" => Self::Failed,
            "canceled" => Self::Canceled,
            "canceling" => Self::Canceling,
            "skipped" => Self::Skipped,
            "manual" => Self::Manual,
            "scheduled" => Self::Scheduled,
            "waiting_for_callback" => Self::WaitingForCallback,
            _ => Self::Unknown,
        })
    }
}

impl ToJson for CiStatus {
    fn write_json<W: json_bourne::JsonWrite + ?Sized>(&self, w: &mut W) -> Result<(), W::Error> {
        let s = match self {
            Self::Created => "created",
            Self::WaitingForResource => "waiting_for_resource",
            Self::Preparing => "preparing",
            Self::Pending => "pending",
            Self::Running => "running",
            Self::Success => "success",
            Self::Failed => "failed",
            Self::Canceled => "canceled",
            Self::Canceling => "canceling",
            Self::Skipped => "skipped",
            Self::Manual => "manual",
            Self::Scheduled => "scheduled",
            Self::WaitingForCallback => "waiting_for_callback",
            Self::Unknown => "unknown",
        };
        s.write_json(w)
    }
}

/// The user who triggered a pipeline or owns a job (a concise subset).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct CiUser {
    pub id: i64,
    pub username: Option<String>,
    pub name: Option<String>,
    pub state: Option<String>,
    pub avatar_url: Option<String>,
    pub web_url: Option<String>,
}

/// A pipeline as returned by the list endpoint (`GET /projects/:id/pipelines`).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct PipelineSummary {
    pub id: i64,
    #[bourne(default)]
    pub iid: i64,
    pub project_id: Option<i64>,
    pub status: CiStatus,
    pub source: Option<String>,
    #[bourne(rename = "ref")]
    pub ref_name: Option<String>,
    pub sha: Option<String>,
    pub name: Option<String>,
    pub web_url: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

/// A pipeline as returned by the single-pipeline endpoint
/// (`GET /projects/:id/pipelines/:pipeline_id`), with timing and the
/// triggering user.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct PipelineDetail {
    pub id: i64,
    #[bourne(default)]
    pub iid: i64,
    pub project_id: Option<i64>,
    pub name: Option<String>,
    pub sha: Option<String>,
    #[bourne(rename = "ref")]
    pub ref_name: Option<String>,
    pub status: CiStatus,
    pub source: Option<String>,
    pub before_sha: Option<String>,
    #[bourne(default)]
    pub tag: bool,
    pub yaml_errors: Option<String>,
    pub user: Option<CiUser>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub committed_at: Option<String>,
    #[cfg_attr(feature = "proptest", proptest(strategy = "finite_opt_f64()"))]
    pub duration: Option<f64>,
    #[cfg_attr(feature = "proptest", proptest(strategy = "finite_opt_f64()"))]
    pub queued_duration: Option<f64>,
    pub coverage: Option<String>,
    pub web_url: Option<String>,
    #[bourne(default)]
    pub archived: bool,
}

/// A single CI/CD variable attached to a pipeline
/// (`GET /projects/:id/pipelines/:pipeline_id/variables`).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct PipelineVariable {
    pub key: String,
    pub value: String,
    pub variable_type: Option<String>,
}

/// A file produced by a job (`artifacts[]` in a job response).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct JobArtifact {
    pub file_type: Option<String>,
    #[bourne(default)]
    pub size: i64,
    pub filename: Option<String>,
    pub file_format: Option<String>,
}

/// The `artifacts_file` summary object on a job.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct ArtifactsFile {
    pub filename: Option<String>,
    #[bourne(default)]
    pub size: i64,
}

/// The commit a job was run against (a concise subset).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct JobCommit {
    pub id: Option<String>,
    pub short_id: Option<String>,
    pub title: Option<String>,
    pub message: Option<String>,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub created_at: Option<String>,
}

/// The pipeline reference embedded in a job.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct JobPipeline {
    pub id: i64,
    pub project_id: Option<i64>,
    #[bourne(rename = "ref")]
    pub ref_name: Option<String>,
    pub sha: Option<String>,
    pub status: Option<CiStatus>,
}

/// The runner that executed a job.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct JobRunner {
    pub id: i64,
    pub description: Option<String>,
    #[bourne(default)]
    pub active: bool,
    #[bourne(default)]
    pub is_shared: bool,
    pub name: Option<String>,
    pub online: Option<bool>,
    pub status: Option<String>,
}

/// A CI/CD job (`GET /projects/:id/jobs`, `.../jobs/:job_id`, and the
/// pipeline-scoped listings).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct Job {
    pub id: i64,
    pub name: Option<String>,
    pub stage: Option<String>,
    pub status: CiStatus,
    #[bourne(rename = "ref")]
    pub ref_name: Option<String>,
    #[bourne(default)]
    pub tag: bool,
    #[bourne(default)]
    pub allow_failure: bool,
    #[bourne(default)]
    pub archived: bool,
    pub source: Option<String>,
    pub failure_reason: Option<String>,
    #[cfg_attr(feature = "proptest", proptest(strategy = "finite_opt_f64()"))]
    pub coverage: Option<f64>,
    #[cfg_attr(feature = "proptest", proptest(strategy = "finite_opt_f64()"))]
    pub duration: Option<f64>,
    #[cfg_attr(feature = "proptest", proptest(strategy = "finite_opt_f64()"))]
    pub queued_duration: Option<f64>,
    pub created_at: Option<String>,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub erased_at: Option<String>,
    pub artifacts_expire_at: Option<String>,
    pub web_url: Option<String>,
    #[bourne(default)]
    pub tag_list: Vec<String>,
    pub commit: Option<JobCommit>,
    pub pipeline: Option<JobPipeline>,
    pub runner: Option<JobRunner>,
    pub user: Option<CiUser>,
    pub artifacts_file: Option<ArtifactsFile>,
    #[bourne(default)]
    pub artifacts: Vec<JobArtifact>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_pipeline_summary() {
        let json = r#"{
            "id": 47,
            "iid": 12,
            "project_id": 1,
            "status": "pending",
            "source": "push",
            "ref": "new-pipeline",
            "sha": "a91957a8",
            "name": "Build pipeline",
            "web_url": "https://example.com/foo/bar/pipelines/47"
        }"#;
        let p: PipelineSummary = parse_str(json).unwrap();
        assert_eq!(p.id, 47);
        assert_eq!(p.status, CiStatus::Pending);
        assert_eq!(p.ref_name.as_deref(), Some("new-pipeline"));
    }

    #[test]
    fn parse_job_with_artifacts() {
        let json = r#"{
            "id": 7,
            "name": "teaspoon",
            "stage": "test",
            "status": "failed",
            "ref": "main",
            "failure_reason": "script_failure",
            "duration": 0.173,
            "tag_list": ["docker runner", "ubuntu18"],
            "artifacts": [
                {"file_type": "archive", "size": 1000, "filename": "artifacts.zip", "file_format": "zip"}
            ],
            "pipeline": {"id": 6, "project_id": 1, "ref": "main", "sha": "0ff3ae19", "status": "pending"}
        }"#;
        let j: Job = parse_str(json).unwrap();
        assert_eq!(j.id, 7);
        assert_eq!(j.status, CiStatus::Failed);
        assert_eq!(j.artifacts.len(), 1);
        assert_eq!(j.pipeline.unwrap().id, 6);
    }

    #[test]
    fn unknown_status_falls_back() {
        let json = r#"{"id":1,"status":"some_new_state"}"#;
        let p: PipelineSummary = parse_str(json).unwrap();
        assert_eq!(p.status, CiStatus::Unknown);
    }
}
