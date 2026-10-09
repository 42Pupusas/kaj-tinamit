//! Request bodies for creating and updating a project.

use json_bourne::ToJson;
use kaj_tinamit::{FeatureAccess, MergeMethod, Visibility};

macro_rules! project_settings {
    () => {
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
        pub fn with_default_branch(mut self, default_branch: impl Into<String>) -> Self {
            self.default_branch = Some(default_branch.into());
            self
        }

        #[must_use]
        pub fn with_topics(mut self, topics: Vec<String>) -> Self {
            self.topics = Some(topics);
            self
        }

        #[must_use]
        pub fn with_ci_config_path(mut self, path: impl Into<String>) -> Self {
            self.ci_config_path = Some(path.into());
            self
        }

        #[must_use]
        pub fn with_merge_method(mut self, method: MergeMethod) -> Self {
            self.merge_method = Some(method);
            self
        }

        #[must_use]
        pub fn with_issues(mut self, access: FeatureAccess) -> Self {
            self.issues_access_level = Some(access);
            self
        }

        #[must_use]
        pub fn with_merge_requests(mut self, access: FeatureAccess) -> Self {
            self.merge_requests_access_level = Some(access);
            self
        }

        #[must_use]
        pub fn with_wiki(mut self, access: FeatureAccess) -> Self {
            self.wiki_access_level = Some(access);
            self
        }

        #[must_use]
        pub fn with_builds(mut self, access: FeatureAccess) -> Self {
            self.builds_access_level = Some(access);
            self
        }

        #[must_use]
        pub fn with_snippets(mut self, access: FeatureAccess) -> Self {
            self.snippets_access_level = Some(access);
            self
        }

        #[must_use]
        pub fn with_container_registry(mut self, access: FeatureAccess) -> Self {
            self.container_registry_access_level = Some(access);
            self
        }

        #[must_use]
        pub fn only_merge_if_pipeline_succeeds(mut self, required: bool) -> Self {
            self.only_allow_merge_if_pipeline_succeeds = Some(required);
            self
        }

        #[must_use]
        pub fn remove_source_branch_after_merge(mut self, remove: bool) -> Self {
            self.remove_source_branch_after_merge = Some(remove);
            self
        }
    };
}

/// Body for `POST /projects`. `name` is required; GitLab derives `path`
/// from it when omitted.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct CreateProject {
    pub name: String,
    #[bourne(skip_if_none)]
    pub path: Option<String>,
    #[bourne(skip_if_none)]
    pub namespace_id: Option<i64>,
    #[bourne(skip_if_none)]
    pub initialize_with_readme: Option<bool>,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    #[bourne(skip_if_none)]
    pub visibility: Option<Visibility>,
    #[bourne(skip_if_none)]
    pub default_branch: Option<String>,
    #[bourne(skip_if_none)]
    pub topics: Option<Vec<String>>,
    #[bourne(skip_if_none)]
    pub ci_config_path: Option<String>,
    #[bourne(skip_if_none)]
    pub merge_method: Option<MergeMethod>,
    #[bourne(skip_if_none)]
    pub issues_access_level: Option<FeatureAccess>,
    #[bourne(skip_if_none)]
    pub merge_requests_access_level: Option<FeatureAccess>,
    #[bourne(skip_if_none)]
    pub wiki_access_level: Option<FeatureAccess>,
    #[bourne(skip_if_none)]
    pub builds_access_level: Option<FeatureAccess>,
    #[bourne(skip_if_none)]
    pub snippets_access_level: Option<FeatureAccess>,
    #[bourne(skip_if_none)]
    pub container_registry_access_level: Option<FeatureAccess>,
    #[bourne(skip_if_none)]
    pub only_allow_merge_if_pipeline_succeeds: Option<bool>,
    #[bourne(skip_if_none)]
    pub remove_source_branch_after_merge: Option<bool>,
}

impl CreateProject {
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            path: None,
            namespace_id: None,
            initialize_with_readme: None,
            description: None,
            visibility: None,
            default_branch: None,
            topics: None,
            ci_config_path: None,
            merge_method: None,
            issues_access_level: None,
            merge_requests_access_level: None,
            wiki_access_level: None,
            builds_access_level: None,
            snippets_access_level: None,
            container_registry_access_level: None,
            only_allow_merge_if_pipeline_succeeds: None,
            remove_source_branch_after_merge: None,
        }
    }

    #[must_use]
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    #[must_use]
    pub fn in_namespace(mut self, namespace_id: i64) -> Self {
        self.namespace_id = Some(namespace_id);
        self
    }

    #[must_use]
    pub fn initialize_with_readme(mut self, init: bool) -> Self {
        self.initialize_with_readme = Some(init);
        self
    }

    project_settings!();
}

/// Body for `PUT /projects/:id`. Every field optional; only set fields are
/// sent.
#[derive(Debug, Clone, Default, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct UpdateProject {
    #[bourne(skip_if_none)]
    pub name: Option<String>,
    #[bourne(skip_if_none)]
    pub path: Option<String>,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    #[bourne(skip_if_none)]
    pub visibility: Option<Visibility>,
    #[bourne(skip_if_none)]
    pub default_branch: Option<String>,
    #[bourne(skip_if_none)]
    pub topics: Option<Vec<String>>,
    #[bourne(skip_if_none)]
    pub ci_config_path: Option<String>,
    #[bourne(skip_if_none)]
    pub merge_method: Option<MergeMethod>,
    #[bourne(skip_if_none)]
    pub issues_access_level: Option<FeatureAccess>,
    #[bourne(skip_if_none)]
    pub merge_requests_access_level: Option<FeatureAccess>,
    #[bourne(skip_if_none)]
    pub wiki_access_level: Option<FeatureAccess>,
    #[bourne(skip_if_none)]
    pub builds_access_level: Option<FeatureAccess>,
    #[bourne(skip_if_none)]
    pub snippets_access_level: Option<FeatureAccess>,
    #[bourne(skip_if_none)]
    pub container_registry_access_level: Option<FeatureAccess>,
    #[bourne(skip_if_none)]
    pub only_allow_merge_if_pipeline_succeeds: Option<bool>,
    #[bourne(skip_if_none)]
    pub remove_source_branch_after_merge: Option<bool>,
}

impl UpdateProject {
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

    project_settings!();
}

/// Body for `POST /projects/:id/fork`. Every field optional; GitLab forks
/// into the caller's personal namespace when no namespace is given.
#[derive(Debug, Clone, Default, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct ForkProject {
    #[bourne(skip_if_none)]
    pub namespace_id: Option<i64>,
    #[bourne(skip_if_none)]
    pub namespace_path: Option<String>,
    #[bourne(skip_if_none)]
    pub name: Option<String>,
    #[bourne(skip_if_none)]
    pub path: Option<String>,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    #[bourne(skip_if_none)]
    pub visibility: Option<Visibility>,
    /// Only fork this branch (comma-separated list, or empty for all).
    #[bourne(skip_if_none)]
    pub branches: Option<String>,
}

impl ForkProject {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn into_namespace(mut self, namespace_id: i64) -> Self {
        self.namespace_id = Some(namespace_id);
        self
    }

    #[must_use]
    pub fn into_namespace_path(mut self, namespace_path: impl Into<String>) -> Self {
        self.namespace_path = Some(namespace_path.into());
        self
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
    pub fn only_branches(mut self, branches: &[&str]) -> Self {
        self.branches = Some(branches.join(","));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_sends_only_what_was_set_with_wire_spellings() {
        let body = CreateProject::new("demo")
            .with_visibility(Visibility::Private)
            .with_merge_method(MergeMethod::FastForward)
            .with_builds(FeatureAccess::Disabled)
            .with_topics(vec!["rust".to_string()]);
        let json = json_bourne::to_string(&body).unwrap();
        assert_eq!(
            json,
            r#"{"name":"demo","visibility":"private","topics":["rust"],"merge_method":"ff","builds_access_level":"disabled"}"#
        );
    }

    #[test]
    fn empty_update_is_an_empty_object() {
        assert_eq!(json_bourne::to_string(&UpdateProject::new()).unwrap(), "{}");
    }

    #[test]
    fn fork_joins_branches() {
        let body = ForkProject::new()
            .into_namespace(9)
            .only_branches(&["main", "dev"]);
        assert_eq!(
            json_bourne::to_string(&body).unwrap(),
            r#"{"namespace_id":9,"branches":"main,dev"}"#
        );
    }
}
