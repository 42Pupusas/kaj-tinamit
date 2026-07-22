//! Emoji-reaction endpoints: the **Emoji reactions** (`award_emoji`) API
//! category for issues, merge requests, and snippets.
//!
//! Reads list reactions; writes award a new reaction or remove an existing
//! one by its award ID.

use json_bourne::ToJson;
use kaj_tinamit::AwardEmoji;

use crate::client::GitlabClient;
use crate::error::Error;

/// Request body for awarding a reaction: `{"name": "thumbsup"}`.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
struct AwardName<'a> {
    name: &'a str,
}

/// Read endpoints for emoji reactions on the main awardable objects.
pub trait AwardEmojiEndpoints {
    /// List emoji reactions on an issue.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn issue_award_emoji(&self, project_id: i64, issue_iid: i64) -> Result<Vec<AwardEmoji>, Error>;

    /// List emoji reactions on a merge request.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn merge_request_award_emoji(
        &self,
        project_id: i64,
        mr_iid: i64,
    ) -> Result<Vec<AwardEmoji>, Error>;

    /// List emoji reactions on a project snippet.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn snippet_award_emoji(
        &self,
        project_id: i64,
        snippet_id: i64,
    ) -> Result<Vec<AwardEmoji>, Error>;

    // --- Writes ---

    /// Award an emoji reaction to an issue (e.g. `"thumbsup"`). Returns the
    /// created award.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn award_issue_emoji(
        &self,
        project_id: i64,
        issue_iid: i64,
        name: &str,
    ) -> Result<AwardEmoji, Error>;

    /// Remove an emoji reaction from an issue (by award ID).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn remove_issue_award_emoji(
        &self,
        project_id: i64,
        issue_iid: i64,
        award_id: i64,
    ) -> Result<(), Error>;

    /// Award an emoji reaction to a merge request. Returns the created award.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn award_merge_request_emoji(
        &self,
        project_id: i64,
        mr_iid: i64,
        name: &str,
    ) -> Result<AwardEmoji, Error>;

    /// Remove an emoji reaction from a merge request (by award ID).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn remove_merge_request_award_emoji(
        &self,
        project_id: i64,
        mr_iid: i64,
        award_id: i64,
    ) -> Result<(), Error>;

    /// Award an emoji reaction to a project snippet. Returns the created award.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn award_snippet_emoji(
        &self,
        project_id: i64,
        snippet_id: i64,
        name: &str,
    ) -> Result<AwardEmoji, Error>;

    /// Remove an emoji reaction from a project snippet (by award ID).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn remove_snippet_award_emoji(
        &self,
        project_id: i64,
        snippet_id: i64,
        award_id: i64,
    ) -> Result<(), Error>;
}

impl AwardEmojiEndpoints for GitlabClient {
    fn issue_award_emoji(&self, project_id: i64, issue_iid: i64) -> Result<Vec<AwardEmoji>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/issues/{issue_iid}/award_emoji"
        ))
    }

    fn merge_request_award_emoji(
        &self,
        project_id: i64,
        mr_iid: i64,
    ) -> Result<Vec<AwardEmoji>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/merge_requests/{mr_iid}/award_emoji"
        ))
    }

    fn snippet_award_emoji(
        &self,
        project_id: i64,
        snippet_id: i64,
    ) -> Result<Vec<AwardEmoji>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/snippets/{snippet_id}/award_emoji"
        ))
    }

    fn award_issue_emoji(
        &self,
        project_id: i64,
        issue_iid: i64,
        name: &str,
    ) -> Result<AwardEmoji, Error> {
        self.post(
            &format!("api/v4/projects/{project_id}/issues/{issue_iid}/award_emoji"),
            &AwardName { name },
        )
    }

    fn remove_issue_award_emoji(
        &self,
        project_id: i64,
        issue_iid: i64,
        award_id: i64,
    ) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/projects/{project_id}/issues/{issue_iid}/award_emoji/{award_id}"
        ))
    }

    fn award_merge_request_emoji(
        &self,
        project_id: i64,
        mr_iid: i64,
        name: &str,
    ) -> Result<AwardEmoji, Error> {
        self.post(
            &format!("api/v4/projects/{project_id}/merge_requests/{mr_iid}/award_emoji"),
            &AwardName { name },
        )
    }

    fn remove_merge_request_award_emoji(
        &self,
        project_id: i64,
        mr_iid: i64,
        award_id: i64,
    ) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/projects/{project_id}/merge_requests/{mr_iid}/award_emoji/{award_id}"
        ))
    }

    fn award_snippet_emoji(
        &self,
        project_id: i64,
        snippet_id: i64,
        name: &str,
    ) -> Result<AwardEmoji, Error> {
        self.post(
            &format!("api/v4/projects/{project_id}/snippets/{snippet_id}/award_emoji"),
            &AwardName { name },
        )
    }

    fn remove_snippet_award_emoji(
        &self,
        project_id: i64,
        snippet_id: i64,
        award_id: i64,
    ) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/projects/{project_id}/snippets/{snippet_id}/award_emoji/{award_id}"
        ))
    }
}
