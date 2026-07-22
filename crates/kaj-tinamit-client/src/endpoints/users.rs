//! User endpoints.

use kaj_tinamit::GitlabUser;

use crate::client::GitlabClient;
use crate::error::{Error, Resource};

/// User-related GitLab endpoints.
pub trait UserEndpoints {
    /// Fetch the user the token authenticates as (`GET /user`).
    ///
    /// The cheapest call to smoke-test credentials against a live instance.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn current_user(&self) -> Result<GitlabUser, Error>;

    /// Fetch all users (paginated). Requires admin on most instances.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn users(&self) -> Result<Vec<GitlabUser>, Error>;

    /// Fetch a single user by username.
    ///
    /// # Errors
    ///
    /// [`Error::NotFound`] if no user matches; otherwise transport/JSON errors.
    fn user_by_username(&self, username: &str) -> Result<GitlabUser, Error>;

    /// Fetch a single user by ID.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn user_by_id(&self, id: i32) -> Result<GitlabUser, Error>;
}

impl UserEndpoints for GitlabClient {
    fn current_user(&self) -> Result<GitlabUser, Error> {
        self.get("api/v4/user")
    }

    fn users(&self) -> Result<Vec<GitlabUser>, Error> {
        self.get_paginated("api/v4/users")
    }

    fn user_by_username(&self, username: &str) -> Result<GitlabUser, Error> {
        let path = format!("api/v4/users?username={username}");
        let users: Vec<GitlabUser> = self.get(&path)?;
        users
            .into_iter()
            .next()
            .ok_or(Error::NotFound(Resource::User))
    }

    fn user_by_id(&self, id: i32) -> Result<GitlabUser, Error> {
        self.get(&format!("api/v4/users/{id}"))
    }
}
