//! Filters for the job listings, and the body for playing a manual job.

use json_bourne::ToJson;
use kaj_tinamit::CiStatus;

use crate::query::QueryParams;

const SCOPE_KEY: &str = "scope%5B%5D";

/// Filters for [`JobEndpoints::jobs`](super::JobEndpoints::jobs),
/// [`JobEndpoints::pipeline_jobs`](super::JobEndpoints::pipeline_jobs), and
/// [`JobEndpoints::pipeline_trigger_jobs`](super::JobEndpoints::pipeline_trigger_jobs).
/// Empty by default, which returns jobs in every status.
#[derive(Debug, Default, Clone)]
pub struct JobQuery {
    params: QueryParams,
}

impl JobQuery {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Only jobs in `status`. Repeat to accept several statuses.
    #[must_use]
    pub fn with_status(mut self, status: CiStatus) -> Self {
        self.params.push(SCOPE_KEY, status.as_str());
        self
    }

    /// Include jobs that were superseded by a retry. Honoured only by the
    /// pipeline-scoped listings.
    #[must_use]
    pub fn include_retried(mut self) -> Self {
        self.params.push_flag("include_retried", true);
        self
    }

    pub(crate) fn apply(&self, path: String) -> String {
        self.params.apply(path)
    }
}

/// One variable passed to a manual job when it is played.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct JobVariable {
    pub key: String,
    pub value: String,
}

impl JobVariable {
    #[must_use]
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub(crate) struct PlayJobBody {
    pub(crate) job_variables_attributes: Vec<JobVariable>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn several_statuses_use_the_array_form() {
        let q = JobQuery::new()
            .with_status(CiStatus::Failed)
            .with_status(CiStatus::Running);
        assert_eq!(
            q.apply("jobs".into()),
            "jobs?scope%5B%5D=failed&scope%5B%5D=running"
        );
    }

    #[test]
    fn play_body_matches_the_documented_shape() {
        let body = PlayJobBody {
            job_variables_attributes: vec![JobVariable::new("TEST_VAR_1", "test1")],
        };
        assert_eq!(
            json_bourne::to_string(&body).unwrap(),
            r#"{"job_variables_attributes":[{"key":"TEST_VAR_1","value":"test1"}]}"#
        );
    }
}
