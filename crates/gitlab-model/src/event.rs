//! Activity-event wire shapes.

use json_bourne::{FromJson, ToJson};

use crate::Id;

/// A GitLab activity event (push, merge request, comment, issue action, …).
///
/// Returned by the events API. The fields here are the stable subset most
/// callers rely on for activity discovery; GitLab sends many more.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct GitlabEvent {
    /// Numeric event id.
    pub id: Option<Id>,
    /// Human action name, e.g. `pushed to`, `opened`, `commented on`.
    pub action_name: Option<String>,
    /// Kind of the thing acted on, e.g. `Issue`, `MergeRequest`, `Note`.
    pub target_type: Option<String>,
    /// ISO 8601 timestamp of the event.
    pub created_at: Option<String>,
    /// Username of the actor (top-level convenience field).
    pub author_username: Option<String>,
    /// Full author object (present on most events).
    pub author: Option<EventAuthor>,
}

/// The actor behind an event.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct EventAuthor {
    pub id: Option<Id>,
    pub username: Option<String>,
    pub name: Option<String>,
}

impl GitlabEvent {
    /// Best-effort username of the actor, preferring the top-level field.
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        self.author_username
            .as_deref()
            .or_else(|| self.author.as_ref().and_then(|a| a.username.as_deref()))
    }
}
