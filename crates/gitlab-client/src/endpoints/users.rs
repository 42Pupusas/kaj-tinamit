//! User endpoints.

use gitlab_model::GitlabUser;

use crate::client::GitlabClient;
use crate::error::{Error, Resource};

impl GitlabClient {
    /// Fetch the user the token authenticates as (`GET /user`).
    ///
    /// The cheapest call to smoke-test credentials against a live instance.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    pub fn current_user(&self) -> Result<GitlabUser, Error> {
        self.get("api/v4/user")
    }

    /// Fetch all users (paginated). Requires admin on most instances.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    pub fn users(&self) -> Result<Vec<GitlabUser>, Error> {
        self.get_paginated("api/v4/users")
    }

    /// Fetch a single user by username.
    ///
    /// # Errors
    ///
    /// [`Error::NotFound`] if no user matches; otherwise transport/JSON errors.
    pub fn user_by_username(&self, username: &str) -> Result<GitlabUser, Error> {
        let path = format!("api/v4/users?username={username}");
        let users: Vec<GitlabUser> = self.get(&path)?;
        users
            .into_iter()
            .next()
            .ok_or(Error::NotFound(Resource::User))
    }

    /// Fetch a single user by ID.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    pub fn user_by_id(&self, id: i32) -> Result<GitlabUser, Error> {
        self.get(&format!("api/v4/users/{id}"))
    }
}
