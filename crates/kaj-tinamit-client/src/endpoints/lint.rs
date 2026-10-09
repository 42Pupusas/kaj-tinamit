//! CI Lint endpoints: validate `.gitlab-ci.yml` in a project's context.

use json_bourne::ToJson;
use kaj_tinamit::CiLintResult;

use crate::client::GitlabClient;
use crate::error::Error;
use crate::query::QueryParams;

/// Body for `POST /projects/:id/ci/lint`.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct CiLint {
    pub content: String,
    #[bourne(skip_if_none)]
    pub dry_run: Option<bool>,
    #[bourne(skip_if_none)]
    #[bourne(rename = "ref")]
    pub ref_name: Option<String>,
}

impl CiLint {
    /// Validate `content` with a static check.
    #[must_use]
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            dry_run: None,
            ref_name: None,
        }
    }

    /// Simulate pipeline creation on `ref_name` instead of only checking
    /// syntax; catches rules and `needs:` that resolve to nothing.
    #[must_use]
    pub fn simulate_on(mut self, ref_name: impl Into<String>) -> Self {
        self.dry_run = Some(true);
        self.ref_name = Some(ref_name.into());
        self
    }
}

/// CI configuration validation.
pub trait LintEndpoints {
    /// Validate arbitrary CI/CD configuration in the context of a project
    /// (its variables and `include:local` files).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors. An invalid
    /// configuration is not an error: see [`CiLintResult::valid`].
    fn lint_ci_config(&self, project_id: i64, lint: &CiLint) -> Result<CiLintResult, Error>;

    /// Validate the `.gitlab-ci.yml` already committed at `content_ref`
    /// (the default branch head when `None`).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn lint_project_ci(
        &self,
        project_id: i64,
        content_ref: Option<&str>,
    ) -> Result<CiLintResult, Error>;
}

impl LintEndpoints for GitlabClient {
    fn lint_ci_config(&self, project_id: i64, lint: &CiLint) -> Result<CiLintResult, Error> {
        self.post(
            &format!("api/v4/projects/{project_id}/ci/lint?include_merged_yaml=true"),
            lint,
        )
    }

    fn lint_project_ci(
        &self,
        project_id: i64,
        content_ref: Option<&str>,
    ) -> Result<CiLintResult, Error> {
        let mut params = QueryParams::default();
        if let Some(r) = content_ref {
            params.push("content_ref", r);
        }
        self.get(&params.apply(format!("api/v4/projects/{project_id}/ci/lint")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_check_sends_only_content() {
        assert_eq!(
            json_bourne::to_string(&CiLint::new("a: 1")).unwrap(),
            r#"{"content":"a: 1"}"#
        );
    }

    #[test]
    fn simulation_sets_dry_run_and_ref() {
        assert_eq!(
            json_bourne::to_string(&CiLint::new("a: 1").simulate_on("main")).unwrap(),
            r#"{"content":"a: 1","dry_run":true,"ref":"main"}"#
        );
    }
}
