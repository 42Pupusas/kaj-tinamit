//! # gitlab-client
//!
//! A blocking, type-safe client for the GitLab REST API v4, built on the
//! in-house stack: HTTP via [`gitlab-http`](gitlab_http), wire shapes via
//! [`gitlab-model`](gitlab_model), JSON via `json-bourne`. No tokio, no
//! reqwest, no serde.
//!
//! ## Quick start
//!
//! ```no_run
//! use gitlab_client::GitlabClient;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Reads GITLAB_URL and GITLAB_PAT from the environment.
//! let client = GitlabClient::from_env()?;
//! let me = client.current_user()?;
//! println!("authenticated as {}", me.username);
//! # Ok(())
//! # }
//! ```

mod client;
mod endpoints;
mod error;
mod pagination;

pub use client::GitlabClient;
pub use error::{ConfigError, Error, HttpMethod, Resource};
pub use pagination::PaginationConfig;

// Re-export the wire shapes so callers need only depend on gitlab-client.
pub use gitlab_model as model;
