//! Project webhook endpoints: list, fetch, add, edit, delete, and fire a
//! test event.

use json_bourne::ToJson;
use kaj_tinamit::ProjectHook;

use crate::client::GitlabClient;
use crate::error::Error;

/// The event kinds a webhook can subscribe to, and the kind
/// [`HookEndpoints::test_project_hook`] can fire.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookEvent {
    Push,
    TagPush,
    Issues,
    ConfidentialIssues,
    Note,
    MergeRequests,
    Job,
    Pipeline,
    WikiPage,
    Releases,
}

impl HookEvent {
    /// The trigger name used by `POST /projects/:id/hooks/:hook_id/test/:trigger`.
    const fn trigger(self) -> &'static str {
        match self {
            Self::Push => "push_events",
            Self::TagPush => "tag_push_events",
            Self::Issues => "issues_events",
            Self::ConfidentialIssues => "confidential_issues_events",
            Self::Note => "note_events",
            Self::MergeRequests => "merge_requests_events",
            Self::Job => "job_events",
            Self::Pipeline => "pipeline_events",
            Self::WikiPage => "wiki_page_events",
            Self::Releases => "releases_events",
        }
    }
}

/// Body for adding (`POST`) or editing (`PUT`) a project webhook. GitLab
/// requires `url` on both; every other field is sent only when set.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
pub struct HookSettings {
    pub url: String,
    #[bourne(skip_if_none)]
    pub name: Option<String>,
    #[bourne(skip_if_none)]
    pub description: Option<String>,
    /// Sent in the `X-Gitlab-Token` header of every delivery.
    #[bourne(skip_if_none)]
    pub token: Option<String>,
    #[bourne(skip_if_none)]
    pub enable_ssl_verification: Option<bool>,
    #[bourne(skip_if_none)]
    pub push_events_branch_filter: Option<String>,
    #[bourne(skip_if_none)]
    pub push_events: Option<bool>,
    #[bourne(skip_if_none)]
    pub tag_push_events: Option<bool>,
    #[bourne(skip_if_none)]
    pub issues_events: Option<bool>,
    #[bourne(skip_if_none)]
    pub confidential_issues_events: Option<bool>,
    #[bourne(skip_if_none)]
    pub note_events: Option<bool>,
    #[bourne(skip_if_none)]
    pub merge_requests_events: Option<bool>,
    #[bourne(skip_if_none)]
    pub job_events: Option<bool>,
    #[bourne(skip_if_none)]
    pub pipeline_events: Option<bool>,
    #[bourne(skip_if_none)]
    pub wiki_page_events: Option<bool>,
    #[bourne(skip_if_none)]
    pub releases_events: Option<bool>,
}

impl HookSettings {
    #[must_use]
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            name: None,
            description: None,
            token: None,
            enable_ssl_verification: None,
            push_events_branch_filter: None,
            push_events: None,
            tag_push_events: None,
            issues_events: None,
            confidential_issues_events: None,
            note_events: None,
            merge_requests_events: None,
            job_events: None,
            pipeline_events: None,
            wiki_page_events: None,
            releases_events: None,
        }
    }

    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn with_secret_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    #[must_use]
    pub fn verify_ssl(mut self, verify: bool) -> Self {
        self.enable_ssl_verification = Some(verify);
        self
    }

    /// Only deliver push events for branches matching this wildcard.
    #[must_use]
    pub fn with_branch_filter(mut self, filter: impl Into<String>) -> Self {
        self.push_events_branch_filter = Some(filter.into());
        self
    }

    /// Turn delivery of `event` on or off.
    #[must_use]
    pub fn on(mut self, event: HookEvent, enabled: bool) -> Self {
        let slot = match event {
            HookEvent::Push => &mut self.push_events,
            HookEvent::TagPush => &mut self.tag_push_events,
            HookEvent::Issues => &mut self.issues_events,
            HookEvent::ConfidentialIssues => &mut self.confidential_issues_events,
            HookEvent::Note => &mut self.note_events,
            HookEvent::MergeRequests => &mut self.merge_requests_events,
            HookEvent::Job => &mut self.job_events,
            HookEvent::Pipeline => &mut self.pipeline_events,
            HookEvent::WikiPage => &mut self.wiki_page_events,
            HookEvent::Releases => &mut self.releases_events,
        };
        *slot = Some(enabled);
        self
    }
}

/// Project webhook endpoints.
pub trait HookEndpoints {
    /// List a project's webhooks.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_hooks(&self, project_id: i64) -> Result<Vec<ProjectHook>, Error>;

    /// Fetch one project webhook.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_hook(&self, project_id: i64, hook_id: i64) -> Result<ProjectHook, Error>;

    /// Add a webhook to a project. Returns the created hook.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn add_project_hook(&self, project_id: i64, hook: &HookSettings) -> Result<ProjectHook, Error>;

    /// Edit a project webhook. Returns the updated hook.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn edit_project_hook(
        &self,
        project_id: i64,
        hook_id: i64,
        hook: &HookSettings,
    ) -> Result<ProjectHook, Error>;

    /// Delete a project webhook.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_project_hook(&self, project_id: i64, hook_id: i64) -> Result<(), Error>;

    /// Ask GitLab to deliver a sample `event` to the hook now.
    ///
    /// # Errors
    ///
    /// Propagates transport and API-status errors; GitLab answers 422 when
    /// the project has no data for that event kind.
    fn test_project_hook(
        &self,
        project_id: i64,
        hook_id: i64,
        event: HookEvent,
    ) -> Result<(), Error>;
}

impl HookEndpoints for GitlabClient {
    fn project_hooks(&self, project_id: i64) -> Result<Vec<ProjectHook>, Error> {
        self.get_paginated(&format!("api/v4/projects/{project_id}/hooks"))
    }

    fn project_hook(&self, project_id: i64, hook_id: i64) -> Result<ProjectHook, Error> {
        self.get(&format!("api/v4/projects/{project_id}/hooks/{hook_id}"))
    }

    fn add_project_hook(&self, project_id: i64, hook: &HookSettings) -> Result<ProjectHook, Error> {
        self.post(&format!("api/v4/projects/{project_id}/hooks"), hook)
    }

    fn edit_project_hook(
        &self,
        project_id: i64,
        hook_id: i64,
        hook: &HookSettings,
    ) -> Result<ProjectHook, Error> {
        self.put(
            &format!("api/v4/projects/{project_id}/hooks/{hook_id}"),
            hook,
        )
    }

    fn delete_project_hook(&self, project_id: i64, hook_id: i64) -> Result<(), Error> {
        self.delete(&format!("api/v4/projects/{project_id}/hooks/{hook_id}"))
    }

    fn test_project_hook(
        &self,
        project_id: i64,
        hook_id: i64,
        event: HookEvent,
    ) -> Result<(), Error> {
        self.post_discard(&format!(
            "api/v4/projects/{project_id}/hooks/{hook_id}/test/{}",
            event.trigger()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_send_only_the_chosen_events() {
        let hook = HookSettings::new("https://ci.example/hook")
            .with_secret_token("s3cret")
            .on(HookEvent::Pipeline, true)
            .on(HookEvent::Push, false);
        assert_eq!(
            json_bourne::to_string(&hook).unwrap(),
            r#"{"url":"https://ci.example/hook","token":"s3cret","push_events":false,"pipeline_events":true}"#
        );
    }

    #[test]
    fn test_triggers_match_the_subscription_fields() {
        for event in [HookEvent::Push, HookEvent::Pipeline, HookEvent::Releases] {
            let json = json_bourne::to_string(&HookSettings::new("u").on(event, true)).unwrap();
            assert!(json.contains(&format!("\"{}\":true", event.trigger())));
        }
    }
}
