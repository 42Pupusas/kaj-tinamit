//! Note read endpoints: the **Notes** API category across every noteable
//! type (issues, merge requests, snippets, epics, wiki pages).
//!
//! All read-only (GET). Create/update/delete are intentionally omitted.
//!
//! These return the comprehensive [`GitlabNote`] shape (permissive
//! `noteable_type`), distinct from the narrower issue/MR `Note` returned by
//! [`MergeRequestEndpoints`](crate::MergeRequestEndpoints).

use gitlab_model::GitlabNote;

use crate::client::GitlabClient;
use crate::error::Error;

/// Read endpoints for notes on all noteable objects.
pub trait NoteEndpoints {
    /// List all notes on an issue (by IID).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn issue_notes(&self, project_id: i64, issue_iid: i64) -> Result<Vec<GitlabNote>, Error>;

    /// Retrieve a single issue note.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn issue_note(
        &self,
        project_id: i64,
        issue_iid: i64,
        note_id: i64,
    ) -> Result<GitlabNote, Error>;

    /// List all notes on a merge request (by IID).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn merge_request_notes(&self, project_id: i64, mr_iid: i64) -> Result<Vec<GitlabNote>, Error>;

    /// Retrieve a single merge request note.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn merge_request_note(
        &self,
        project_id: i64,
        mr_iid: i64,
        note_id: i64,
    ) -> Result<GitlabNote, Error>;

    /// List all notes on a project snippet.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn snippet_notes(&self, project_id: i64, snippet_id: i64) -> Result<Vec<GitlabNote>, Error>;

    /// Retrieve a single snippet note.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn snippet_note(
        &self,
        project_id: i64,
        snippet_id: i64,
        note_id: i64,
    ) -> Result<GitlabNote, Error>;

    /// List all notes on a group epic (uses the epic **ID**, not IID).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn epic_notes(&self, group_id: i64, epic_id: i64) -> Result<Vec<GitlabNote>, Error>;

    /// Retrieve a single epic note.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn epic_note(&self, group_id: i64, epic_id: i64, note_id: i64) -> Result<GitlabNote, Error>;

    /// List all notes on a project wiki page (by wiki page meta ID).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn wiki_page_notes(
        &self,
        project_id: i64,
        wiki_page_meta_id: i64,
    ) -> Result<Vec<GitlabNote>, Error>;

    /// Retrieve a single wiki page note.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn wiki_page_note(
        &self,
        project_id: i64,
        wiki_page_meta_id: i64,
        note_id: i64,
    ) -> Result<GitlabNote, Error>;
}

impl NoteEndpoints for GitlabClient {
    fn issue_notes(&self, project_id: i64, issue_iid: i64) -> Result<Vec<GitlabNote>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/issues/{issue_iid}/notes"
        ))
    }

    fn issue_note(
        &self,
        project_id: i64,
        issue_iid: i64,
        note_id: i64,
    ) -> Result<GitlabNote, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/issues/{issue_iid}/notes/{note_id}"
        ))
    }

    fn merge_request_notes(&self, project_id: i64, mr_iid: i64) -> Result<Vec<GitlabNote>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/merge_requests/{mr_iid}/notes"
        ))
    }

    fn merge_request_note(
        &self,
        project_id: i64,
        mr_iid: i64,
        note_id: i64,
    ) -> Result<GitlabNote, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/merge_requests/{mr_iid}/notes/{note_id}"
        ))
    }

    fn snippet_notes(&self, project_id: i64, snippet_id: i64) -> Result<Vec<GitlabNote>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/snippets/{snippet_id}/notes"
        ))
    }

    fn snippet_note(
        &self,
        project_id: i64,
        snippet_id: i64,
        note_id: i64,
    ) -> Result<GitlabNote, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/snippets/{snippet_id}/notes/{note_id}"
        ))
    }

    fn epic_notes(&self, group_id: i64, epic_id: i64) -> Result<Vec<GitlabNote>, Error> {
        self.get_paginated(&format!("api/v4/groups/{group_id}/epics/{epic_id}/notes"))
    }

    fn epic_note(&self, group_id: i64, epic_id: i64, note_id: i64) -> Result<GitlabNote, Error> {
        self.get(&format!(
            "api/v4/groups/{group_id}/epics/{epic_id}/notes/{note_id}"
        ))
    }

    fn wiki_page_notes(
        &self,
        project_id: i64,
        wiki_page_meta_id: i64,
    ) -> Result<Vec<GitlabNote>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/wiki_pages/{wiki_page_meta_id}/notes"
        ))
    }

    fn wiki_page_note(
        &self,
        project_id: i64,
        wiki_page_meta_id: i64,
        note_id: i64,
    ) -> Result<GitlabNote, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/wiki_pages/{wiki_page_meta_id}/notes/{note_id}"
        ))
    }
}
