//! Wire shapes for the **Project-level** and **Group-level CI/CD variables**
//! API categories.
//!
//! Distinct from [`PipelineVariable`](crate::ci::PipelineVariable), which is
//! the read-only record of what a *single pipeline run* was given. The types
//! here are the stored, editable settings behind
//! Settings → CI/CD → Variables.

use crate::Id;
use json_bourne::{FromJson, Lexer, ToJson};

/// Whether a variable is exposed as an environment variable or written to a
/// temporary file whose path becomes the variable's value.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[non_exhaustive]
pub enum VariableType {
    #[default]
    EnvVar,
    File,
    Unknown,
}

impl<'input> FromJson<'input> for VariableType {
    fn from_lex(lex: &mut Lexer<'input>) -> Result<Self, json_bourne::Error> {
        let s = String::from_lex(lex)?;
        Ok(match s.as_str() {
            "env_var" => Self::EnvVar,
            "file" => Self::File,
            _ => Self::Unknown,
        })
    }
}

impl ToJson for VariableType {
    fn write_json<W: json_bourne::JsonWrite + ?Sized>(&self, w: &mut W) -> Result<(), W::Error> {
        let s = match self {
            Self::EnvVar => "env_var",
            Self::File => "file",
            Self::Unknown => "unknown",
        };
        s.write_json(w)
    }
}

/// How a variable's value is concealed in job logs. Prefixed to avoid
/// colliding with [`Visibility`](crate::project::Visibility), which is a
/// project's public/internal/private setting.
///
/// GitLab exposes this as two independent booleans (`masked`, `hidden`), but
/// they are not orthogonal: `hidden` requires `masked`, and a hidden
/// variable's value is never returned by the API again after creation.
/// Modelling the legal combinations as one enum makes the invalid pairing
/// unrepresentable.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[non_exhaustive]
pub enum VariableVisibility {
    #[default]
    Visible,
    Masked,
    /// Masked *and* hidden: write-only. GitLab rejects an attempt to un-hide
    /// an existing hidden variable, so this is effectively permanent.
    MaskedAndHidden,
}

impl VariableVisibility {
    #[must_use]
    pub const fn from_flags(masked: bool, hidden: bool) -> Self {
        match (masked, hidden) {
            (_, true) => Self::MaskedAndHidden,
            (true, false) => Self::Masked,
            (false, false) => Self::Visible,
        }
    }

    #[must_use]
    pub const fn masked(self) -> bool {
        matches!(self, Self::Masked | Self::MaskedAndHidden)
    }

    #[must_use]
    pub const fn hidden(self) -> bool {
        matches!(self, Self::MaskedAndHidden)
    }
}

/// A stored CI/CD variable
/// (`GET /projects/:id/variables`, `GET /groups/:id/variables`).
///
/// `value` is `None` for hidden variables, which GitLab never returns after
/// creation.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct CiVariable {
    pub key: String,
    pub value: Option<String>,
    #[bourne(default)]
    pub variable_type: VariableType,
    #[bourne(default)]
    pub protected: bool,
    #[bourne(default)]
    pub masked: bool,
    #[bourne(default)]
    pub hidden: bool,
    #[bourne(default)]
    pub raw: bool,
    pub environment_scope: Option<String>,
    pub description: Option<String>,
    /// Present on group-level variables only.
    pub group_id: Option<Id>,
}

impl CiVariable {
    /// The masking/hiding state as a single value.
    #[must_use]
    pub const fn visibility(&self) -> VariableVisibility {
        VariableVisibility::from_flags(self.masked, self.hidden)
    }

    /// Whether this variable applies to every environment (`*`). A variable
    /// scoped to one environment is invisible to jobs in the others, which is
    /// a common cause of "the variable is set but the job cannot see it".
    #[must_use]
    pub fn applies_everywhere(&self) -> bool {
        self.environment_scope
            .as_deref()
            .is_none_or(|scope| scope == "*")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parses_a_project_variable() {
        let json = r#"{
            "variable_type": "env_var",
            "key": "TEST_VARIABLE_1",
            "value": "TEST_1",
            "protected": false,
            "masked": true,
            "hidden": false,
            "raw": false,
            "environment_scope": "*",
            "description": null
        }"#;
        let v: CiVariable = parse_str(json).unwrap();
        assert_eq!(v.key, "TEST_VARIABLE_1");
        assert_eq!(v.value.as_deref(), Some("TEST_1"));
        assert_eq!(v.variable_type, VariableType::EnvVar);
        assert_eq!(v.visibility(), VariableVisibility::Masked);
        assert!(v.applies_everywhere());
    }

    #[test]
    fn a_hidden_variable_reports_no_value() {
        let json = r#"{"key":"SECRET","masked":true,"hidden":true}"#;
        let v: CiVariable = parse_str(json).unwrap();
        assert_eq!(v.value, None);
        assert_eq!(v.visibility(), VariableVisibility::MaskedAndHidden);
        assert!(v.visibility().masked());
    }

    #[test]
    fn a_scoped_variable_does_not_apply_everywhere() {
        let json = r#"{"key":"K","value":"v","environment_scope":"production"}"#;
        let v: CiVariable = parse_str(json).unwrap();
        assert!(!v.applies_everywhere());
    }

    #[test]
    fn a_file_variable_keeps_its_type() {
        let json = r#"{"key":"K","value":"v","variable_type":"file"}"#;
        let v: CiVariable = parse_str(json).unwrap();
        assert_eq!(v.variable_type, VariableType::File);
    }

    #[test]
    fn an_unmodelled_variable_type_falls_back() {
        let json = r#"{"key":"K","value":"v","variable_type":"something_new"}"#;
        let v: CiVariable = parse_str(json).unwrap();
        assert_eq!(v.variable_type, VariableType::Unknown);
    }

    #[test]
    fn hidden_implies_masked_whatever_the_flags_say() {
        assert_eq!(
            VariableVisibility::from_flags(false, true),
            VariableVisibility::MaskedAndHidden
        );
        assert!(VariableVisibility::from_flags(false, true).masked());
    }
}
