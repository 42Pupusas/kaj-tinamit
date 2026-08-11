//! Endpoints for the **Project-level** and **Group-level CI/CD variables**
//! API categories.
//!
//! Reads list/fetch stored variables; writes create, update, and delete them.
//! These are the settings behind Settings → CI/CD → Variables, not the
//! per-run values exposed by
//! [`PipelineEndpoints`](crate::PipelineEndpoints).

use json_bourne::ToJson;
use kaj_tinamit::{CiVariable, VariableType};

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

/// Selects between variables that share a key but differ in environment
/// scope. GitLab allows the same key many times as long as the scopes
/// differ, so fetch/update/delete are ambiguous without this.
#[derive(Debug, Clone, Default)]
pub struct VariableFilter {
    environment_scope: Option<String>,
}

impl VariableFilter {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_environment_scope(mut self, scope: impl Into<String>) -> Self {
        self.environment_scope = Some(scope.into());
        self
    }

    /// The `filter[environment_scope]=...` query suffix, or an empty string.
    fn query_suffix(&self) -> String {
        self.environment_scope
            .as_ref()
            .map_or_else(String::new, |scope| {
                format!("?filter[environment_scope]={}", scope.percent_encode())
            })
    }
}

/// Body for creating a CI/CD variable (`POST .../variables`).
///
/// `masked_and_hidden` is write-only and permanent: GitLab will not return
/// the value again, nor allow un-hiding it later.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct CreateVariable {
    pub key: String,
    pub value: String,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    #[bourne(skip_if_none)]
    pub environment_scope: Option<String>,
    #[bourne(skip_if_none)]
    pub masked: Option<bool>,
    #[bourne(skip_if_none)]
    pub masked_and_hidden: Option<bool>,
    #[bourne(skip_if_none)]
    pub protected: Option<bool>,
    #[bourne(skip_if_none)]
    pub raw: Option<bool>,
    #[bourne(skip_if_none)]
    pub variable_type: Option<VariableType>,
}

impl CreateVariable {
    #[must_use]
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            description: None,
            environment_scope: None,
            masked: None,
            masked_and_hidden: None,
            protected: None,
            raw: None,
            variable_type: None,
        }
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn with_environment_scope(mut self, scope: impl Into<String>) -> Self {
        self.environment_scope = Some(scope.into());
        self
    }

    /// Mask the value in job logs. GitLab rejects values that do not meet its
    /// masking requirements (single line, minimum length, limited charset).
    #[must_use]
    pub fn masked(mut self) -> Self {
        self.masked = Some(true);
        self
    }

    /// Mask the value *and* hide it from the API permanently.
    #[must_use]
    pub fn masked_and_hidden(mut self) -> Self {
        self.masked = Some(true);
        self.masked_and_hidden = Some(true);
        self
    }

    /// Restrict the variable to protected branches and tags.
    #[must_use]
    pub fn protected(mut self) -> Self {
        self.protected = Some(true);
        self
    }

    /// Treat the value as a raw string, disabling `$VARIABLE` expansion.
    #[must_use]
    pub fn raw(mut self) -> Self {
        self.raw = Some(true);
        self
    }

    #[must_use]
    pub fn with_variable_type(mut self, variable_type: VariableType) -> Self {
        self.variable_type = Some(variable_type);
        self
    }
}

/// Body for updating a CI/CD variable (`PUT .../variables/:key`).
///
/// GitLab requires `value` on update; everything else is optional and unset
/// fields are left untouched. There is deliberately no way to un-hide a
/// hidden variable, because the API does not support it.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct UpdateVariable {
    pub value: String,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    #[bourne(skip_if_none)]
    pub environment_scope: Option<String>,
    #[bourne(skip_if_none)]
    pub masked: Option<bool>,
    #[bourne(skip_if_none)]
    pub protected: Option<bool>,
    #[bourne(skip_if_none)]
    pub raw: Option<bool>,
    #[bourne(skip_if_none)]
    pub variable_type: Option<VariableType>,
}

impl UpdateVariable {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            description: None,
            environment_scope: None,
            masked: None,
            protected: None,
            raw: None,
            variable_type: None,
        }
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn with_environment_scope(mut self, scope: impl Into<String>) -> Self {
        self.environment_scope = Some(scope.into());
        self
    }

    #[must_use]
    pub fn with_masked(mut self, masked: bool) -> Self {
        self.masked = Some(masked);
        self
    }

    #[must_use]
    pub fn with_protected(mut self, protected: bool) -> Self {
        self.protected = Some(protected);
        self
    }

    #[must_use]
    pub fn with_raw(mut self, raw: bool) -> Self {
        self.raw = Some(raw);
        self
    }

    #[must_use]
    pub fn with_variable_type(mut self, variable_type: VariableType) -> Self {
        self.variable_type = Some(variable_type);
        self
    }
}

/// Read and write endpoints for project- and group-level CI/CD variables.
pub trait VariableEndpoints {
    /// List every CI/CD variable on a project.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_variables(&self, project_id: i64) -> Result<Vec<CiVariable>, Error>;

    /// Retrieve one project variable by key, disambiguated by `filter` when
    /// several share the key.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_variable(
        &self,
        project_id: i64,
        key: &str,
        filter: &VariableFilter,
    ) -> Result<CiVariable, Error>;

    /// Create a project variable. GitLab rejects a duplicate key unless the
    /// environment scope differs.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_project_variable(
        &self,
        project_id: i64,
        variable: &CreateVariable,
    ) -> Result<CiVariable, Error>;

    /// Update a project variable by key.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn update_project_variable(
        &self,
        project_id: i64,
        key: &str,
        update: &UpdateVariable,
        filter: &VariableFilter,
    ) -> Result<CiVariable, Error>;

    /// Delete a project variable by key.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_project_variable(
        &self,
        project_id: i64,
        key: &str,
        filter: &VariableFilter,
    ) -> Result<(), Error>;

    /// List every CI/CD variable on a group.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_variables(&self, group_id: i64) -> Result<Vec<CiVariable>, Error>;

    /// Retrieve one group variable by key.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_variable(
        &self,
        group_id: i64,
        key: &str,
        filter: &VariableFilter,
    ) -> Result<CiVariable, Error>;

    /// Create a group variable.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_group_variable(
        &self,
        group_id: i64,
        variable: &CreateVariable,
    ) -> Result<CiVariable, Error>;

    /// Update a group variable by key.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn update_group_variable(
        &self,
        group_id: i64,
        key: &str,
        update: &UpdateVariable,
        filter: &VariableFilter,
    ) -> Result<CiVariable, Error>;

    /// Delete a group variable by key.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_group_variable(
        &self,
        group_id: i64,
        key: &str,
        filter: &VariableFilter,
    ) -> Result<(), Error>;
}

impl VariableEndpoints for GitlabClient {
    fn project_variables(&self, project_id: i64) -> Result<Vec<CiVariable>, Error> {
        self.get_paginated(&format!("api/v4/projects/{project_id}/variables"))
    }

    fn project_variable(
        &self,
        project_id: i64,
        key: &str,
        filter: &VariableFilter,
    ) -> Result<CiVariable, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/variables/{}{}",
            key.percent_encode(),
            filter.query_suffix()
        ))
    }

    fn create_project_variable(
        &self,
        project_id: i64,
        variable: &CreateVariable,
    ) -> Result<CiVariable, Error> {
        self.post(&format!("api/v4/projects/{project_id}/variables"), variable)
    }

    fn update_project_variable(
        &self,
        project_id: i64,
        key: &str,
        update: &UpdateVariable,
        filter: &VariableFilter,
    ) -> Result<CiVariable, Error> {
        self.put(
            &format!(
                "api/v4/projects/{project_id}/variables/{}{}",
                key.percent_encode(),
                filter.query_suffix()
            ),
            update,
        )
    }

    fn delete_project_variable(
        &self,
        project_id: i64,
        key: &str,
        filter: &VariableFilter,
    ) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/projects/{project_id}/variables/{}{}",
            key.percent_encode(),
            filter.query_suffix()
        ))
    }

    fn group_variables(&self, group_id: i64) -> Result<Vec<CiVariable>, Error> {
        self.get_paginated(&format!("api/v4/groups/{group_id}/variables"))
    }

    fn group_variable(
        &self,
        group_id: i64,
        key: &str,
        filter: &VariableFilter,
    ) -> Result<CiVariable, Error> {
        self.get(&format!(
            "api/v4/groups/{group_id}/variables/{}{}",
            key.percent_encode(),
            filter.query_suffix()
        ))
    }

    fn create_group_variable(
        &self,
        group_id: i64,
        variable: &CreateVariable,
    ) -> Result<CiVariable, Error> {
        self.post(&format!("api/v4/groups/{group_id}/variables"), variable)
    }

    fn update_group_variable(
        &self,
        group_id: i64,
        key: &str,
        update: &UpdateVariable,
        filter: &VariableFilter,
    ) -> Result<CiVariable, Error> {
        self.put(
            &format!(
                "api/v4/groups/{group_id}/variables/{}{}",
                key.percent_encode(),
                filter.query_suffix()
            ),
            update,
        )
    }

    fn delete_group_variable(
        &self,
        group_id: i64,
        key: &str,
        filter: &VariableFilter,
    ) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/groups/{group_id}/variables/{}{}",
            key.percent_encode(),
            filter.query_suffix()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::to_string;

    #[test]
    fn an_empty_filter_adds_no_query() {
        assert_eq!(VariableFilter::new().query_suffix(), "");
    }

    #[test]
    fn a_scoped_filter_encodes_the_scope() {
        let filter = VariableFilter::new().with_environment_scope("review/*");
        assert_eq!(
            filter.query_suffix(),
            "?filter[environment_scope]=review%2F%2A"
        );
    }

    #[test]
    fn create_omits_every_unset_field() {
        let json = to_string(&CreateVariable::new("KEY", "value")).unwrap();
        assert_eq!(json, r#"{"key":"KEY","value":"value"}"#);
    }

    #[test]
    fn hiding_a_variable_also_masks_it() {
        let variable = CreateVariable::new("KEY", "value").masked_and_hidden();
        assert_eq!(variable.masked, Some(true));
        assert_eq!(variable.masked_and_hidden, Some(true));
    }

    #[test]
    fn create_serializes_the_variable_type() {
        let json =
            to_string(&CreateVariable::new("KEY", "value").with_variable_type(VariableType::File))
                .unwrap();
        assert!(json.contains(r#""variable_type":"file""#));
    }

    #[test]
    fn update_always_carries_the_value() {
        let json = to_string(&UpdateVariable::new("v")).unwrap();
        assert_eq!(json, r#"{"value":"v"}"#);
    }
}
