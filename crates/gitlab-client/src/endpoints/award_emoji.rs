//! Emoji-reaction read endpoints: the **Emoji reactions** (`award_emoji`)
//! API category for issues, merge requests, and snippets.
//!
//! All read-only (GET). Awarding/removing reactions is intentionally omitted.

use gitlab_model::AwardEmoji;

use crate::client::GitlabClient;
use crate::error::Error;

/// Read endpoints for emoji reactions on the main awardable objects.
pub trait AwardEmojiEndpoints {
    /// List emoji reactions on an issue.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn issue_award_emoji(&self, project_id: i64, issue_iid: i64)
    -> Result<Vec<AwardEmoji>, Error>;

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
}

impl AwardEmojiEndpoints for GitlabClient {
    fn issue_award_emoji(
        &self,
        project_id: i64,
        issue_iid: i64,
    ) -> Result<Vec<AwardEmoji>, Error> {
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
}
