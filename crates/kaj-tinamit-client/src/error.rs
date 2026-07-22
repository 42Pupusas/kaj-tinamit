//! Error types for the GitLab client.
//!
//! Hand-rolled — no `thiserror`, no stringly-typed messages. Every failure
//! mode is a typed enum variant so callers can `match` exhaustively instead
//! of parsing text.

use std::fmt;

/// The top-level error type for every fallible client operation.
#[derive(Debug)]
pub enum Error {
    /// The underlying HTTP transport failed.
    Http(kaj_tinamit_http::Error),

    /// Failed to (de)serialize a JSON payload.
    Json(json_bourne::Error),

    /// GitLab returned a non-2xx status for a request.
    Api {
        /// The HTTP method used for the request.
        method: HttpMethod,
        /// The HTTP status code GitLab responded with.
        status: u16,
    },

    /// A resource was requested but GitLab reported it does not exist.
    NotFound(Resource),

    /// The client was misconfigured (bad URL, missing env var, bad token).
    Config(ConfigError),
}

/// The HTTP verb used for a request, for `Api` error context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
        })
    }
}

/// A GitLab resource that could not be located.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resource {
    User,
    Project,
    Issue,
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::User => "user",
            Self::Project => "project",
            Self::Issue => "issue",
        })
    }
}

/// A client-configuration failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigError {
    /// The provided base URL was empty or whitespace.
    EmptyBaseUrl,
    /// The token was empty or contained control characters.
    InvalidToken,
    /// `GITLAB_URL` was not set in the environment.
    MissingUrlEnv,
    /// `GITLAB_PAT` was not set in the environment.
    MissingTokenEnv,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::EmptyBaseUrl => "GitLab base URL is empty",
            Self::InvalidToken => "GitLab token is empty or malformed",
            Self::MissingUrlEnv => "GITLAB_URL environment variable is not set",
            Self::MissingTokenEnv => "GITLAB_PAT environment variable is not set",
        })
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(e) => write!(f, "HTTP request failed: {e}"),
            Self::Json(e) => write!(f, "JSON (de)serialization failed: {e}"),
            Self::Api { method, status } => {
                write!(f, "GitLab API error: {method} request returned {status}")
            }
            Self::NotFound(resource) => write!(f, "{resource} not found"),
            Self::Config(e) => write!(f, "configuration error: {e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Http(e) => Some(e),
            Self::Json(e) => Some(e),
            _ => None,
        }
    }
}

impl From<kaj_tinamit_http::Error> for Error {
    fn from(e: kaj_tinamit_http::Error) -> Self {
        Self::Http(e)
    }
}

impl From<json_bourne::Error> for Error {
    fn from(e: json_bourne::Error) -> Self {
        Self::Json(e)
    }
}
