//! Board endpoints: the **Issue boards** and **Group issue boards** API
//! categories.
//!
//! Reads list/fetch boards and their lists; writes create/delete boards and
//! add/remove lists (columns) backed by a label.

use gitlab_model::{Board, BoardList};
use json_bourne::ToJson;

use crate::client::GitlabClient;
use crate::error::Error;

/// Body for creating a board: `{"name": "…"}`.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
struct BoardName<'a> {
    name: &'a str,
}

/// Body for adding a board list backed by a label: `{"label_id": N}`.
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
struct BoardListBody {
    label_id: i64,
}

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
    fn project_board_lists(&self, project_id: i64, board_id: i64) -> Result<Vec<BoardList>, Error>;

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

    // --- Writes ---

    /// Create a project issue board. Returns the created board.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_project_board(&self, project_id: i64, name: &str) -> Result<Board, Error>;

    /// Delete a project issue board.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_project_board(&self, project_id: i64, board_id: i64) -> Result<(), Error>;

    /// Add a list (column) backed by `label_id` to a project board. Returns
    /// the created list.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_project_board_list(
        &self,
        project_id: i64,
        board_id: i64,
        label_id: i64,
    ) -> Result<BoardList, Error>;

    /// Remove a list from a project board.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_project_board_list(
        &self,
        project_id: i64,
        board_id: i64,
        list_id: i64,
    ) -> Result<(), Error>;

    /// Create a group issue board. Returns the created board.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn create_group_board(&self, group_id: i64, name: &str) -> Result<Board, Error>;

    /// Delete a group issue board.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn delete_group_board(&self, group_id: i64, board_id: i64) -> Result<(), Error>;
}

impl BoardEndpoints for GitlabClient {
    fn project_boards(&self, project_id: i64) -> Result<Vec<Board>, Error> {
        self.get_paginated(&format!("api/v4/projects/{project_id}/boards"))
    }

    fn project_board(&self, project_id: i64, board_id: i64) -> Result<Board, Error> {
        self.get(&format!("api/v4/projects/{project_id}/boards/{board_id}"))
    }

    fn project_board_lists(&self, project_id: i64, board_id: i64) -> Result<Vec<BoardList>, Error> {
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
        self.get_paginated(&format!("api/v4/groups/{group_id}/boards/{board_id}/lists"))
    }

    fn create_project_board(&self, project_id: i64, name: &str) -> Result<Board, Error> {
        self.post(
            &format!("api/v4/projects/{project_id}/boards"),
            &BoardName { name },
        )
    }

    fn delete_project_board(&self, project_id: i64, board_id: i64) -> Result<(), Error> {
        self.delete(&format!("api/v4/projects/{project_id}/boards/{board_id}"))
    }

    fn create_project_board_list(
        &self,
        project_id: i64,
        board_id: i64,
        label_id: i64,
    ) -> Result<BoardList, Error> {
        self.post(
            &format!("api/v4/projects/{project_id}/boards/{board_id}/lists"),
            &BoardListBody { label_id },
        )
    }

    fn delete_project_board_list(
        &self,
        project_id: i64,
        board_id: i64,
        list_id: i64,
    ) -> Result<(), Error> {
        self.delete(&format!(
            "api/v4/projects/{project_id}/boards/{board_id}/lists/{list_id}"
        ))
    }

    fn create_group_board(&self, group_id: i64, name: &str) -> Result<Board, Error> {
        self.post(
            &format!("api/v4/groups/{group_id}/boards"),
            &BoardName { name },
        )
    }

    fn delete_group_board(&self, group_id: i64, board_id: i64) -> Result<(), Error> {
        self.delete(&format!("api/v4/groups/{group_id}/boards/{board_id}"))
    }
}
