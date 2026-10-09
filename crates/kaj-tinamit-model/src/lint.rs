//! Wire shape for the **CI Lint** API category
//! (`GET`/`POST /projects/:id/ci/lint`).

use json_bourne::{FromJson, ToJson};

/// The verdict on a `.gitlab-ci.yml`.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct CiLintResult {
    #[bourne(default)]
    pub valid: bool,
    #[bourne(default)]
    pub errors: Vec<String>,
    #[bourne(default)]
    pub warnings: Vec<String>,
    /// The configuration after `include:` and `extends:` are expanded.
    pub merged_yaml: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_invalid_config() {
        let json = r#"{
            "valid": false,
            "errors": ["jobs config should contain at least one visible job"],
            "warnings": [],
            "merged_yaml": "---\n:test_job:\n  :script: echo 1\n",
            "includes": []
        }"#;
        let r: CiLintResult = parse_str(json).unwrap();
        assert!(!r.valid);
        assert_eq!(r.errors.len(), 1);
        assert!(r.merged_yaml.is_some());
    }
}
