//! Board read endpoints: the **Issue boards** and **Group issue boards**
//! API categories.
//!
//! All read-only (GET). Create/update/delete of boards and lists are
//! intentionally omitted.

use gitlab_model::{Board, BoardList};

use crate::client::GitlabClient;
use crate::error::Error;

/// Read endpoints for project and group issue boards.
pub trait BoardEndpoints {
    /// List a project's issue boards.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_boards(&self, project_id: i64) -> Result<Vec<Board>, Error>;

    /// Retrieve a single project board by ID.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_board(&self, project_id: i64, board_id: i64) -> Result<Board, Error>;

    /// List the lists (columns) of a project board.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_board_lists(&self, project_id: i64, board_id: i64)
    -> Result<Vec<BoardList>, Error>;

    /// List a group's issue boards.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_boards(&self, group_id: i64) -> Result<Vec<Board>, Error>;

    /// Retrieve a single group board by ID.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_board(&self, group_id: i64, board_id: i64) -> Result<Board, Error>;

    /// List the lists (columns) of a group board.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_board_lists(&self, group_id: i64, board_id: i64) -> Result<Vec<BoardList>, Error>;
}

impl BoardEndpoints for GitlabClient {
    fn project_boards(&self, project_id: i64) -> Result<Vec<Board>, Error> {
        self.get_paginated(&format!("api/v4/projects/{project_id}/boards"))
    }

    fn project_board(&self, project_id: i64, board_id: i64) -> Result<Board, Error> {
        self.get(&format!("api/v4/projects/{project_id}/boards/{board_id}"))
    }

    fn project_board_lists(
        &self,
        project_id: i64,
        board_id: i64,
    ) -> Result<Vec<BoardList>, Error> {
        self.get_paginated(&format!(
            "api/v4/projects/{project_id}/boards/{board_id}/lists"
        ))
    }

    fn group_boards(&self, group_id: i64) -> Result<Vec<Board>, Error> {
        self.get_paginated(&format!("api/v4/groups/{group_id}/boards"))
    }

    fn group_board(&self, group_id: i64, board_id: i64) -> Result<Board, Error> {
        self.get(&format!("api/v4/groups/{group_id}/boards/{board_id}"))
    }

    fn group_board_lists(&self, group_id: i64, board_id: i64) -> Result<Vec<BoardList>, Error> {
        self.get_paginated(&format!(
            "api/v4/groups/{group_id}/boards/{board_id}/lists"
        ))
    }
}
