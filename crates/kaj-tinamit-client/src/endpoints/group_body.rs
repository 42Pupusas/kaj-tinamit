//! Request bodies for creating and updating a group.

use json_bourne::ToJson;
use kaj_tinamit::Visibility;

/// Body for `POST /groups`. `name` and `path` are both required.
/// Set `parent_id` to create a subgroup.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct CreateGroup {
    pub name: String,
    pub path: String,
    #[bourne(skip_if_none)]
    pub parent_id: Option<i64>,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    #[bourne(skip_if_none)]
    pub visibility: Option<Visibility>,
    #[bourne(skip_if_none)]
    pub request_access_enabled: Option<bool>,
}

impl CreateGroup {
    #[must_use]
    pub fn new(name: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            parent_id: None,
            description: None,
            visibility: None,
            request_access_enabled: None,
        }
    }

    #[must_use]
    pub fn under(mut self, parent_id: i64) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = Some(visibility);
        self
    }

    #[must_use]
    pub fn request_access(mut self, enabled: bool) -> Self {
        self.request_access_enabled = Some(enabled);
        self
    }
}

/// Body for `PUT /groups/:id`. Every field optional.
#[derive(Debug, Clone, Default, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct UpdateGroup {
    #[bourne(skip_if_none)]
    pub name: Option<String>,
    #[bourne(skip_if_none)]
    pub path: Option<String>,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    #[bourne(skip_if_none)]
    pub visibility: Option<Visibility>,
    #[bourne(skip_if_none)]
    pub request_access_enabled: Option<bool>,
}

impl UpdateGroup {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    #[must_use]
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = Some(visibility);
        self
    }

    #[must_use]
    pub fn request_access(mut self, enabled: bool) -> Self {
        self.request_access_enabled = Some(enabled);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subgroup_body() {
        let body = CreateGroup::new("Team", "team")
            .under(12)
            .with_visibility(Visibility::Internal);
        assert_eq!(
            json_bourne::to_string(&body).unwrap(),
            r#"{"name":"Team","path":"team","parent_id":12,"visibility":"internal"}"#
        );
    }

    #[test]
    fn empty_update_is_empty() {
        assert_eq!(json_bourne::to_string(&UpdateGroup::new()).unwrap(), "{}");
    }
}
