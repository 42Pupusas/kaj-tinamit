//! Core GitLab client: authentication, URL handling, and the private
//! GET/POST/pagination helpers the endpoint modules build on.

use json_bourne::{FromJson, ToJson, parse_str};

use crate::error::{ConfigError, Error, HttpMethod};
use crate::pagination::PaginationConfig;

/// GitLab API client — blocking HTTP over the in-house xibalba stack.
#[derive(Clone)]
pub struct GitlabClient {
    token: String,
    base_url: String,
}

impl GitlabClient {
    /// Create a new GitLab client with explicit configuration.
    ///
    /// The `base_url` is normalized: a missing scheme defaults to
    /// `https://`, and any trailing slash is trimmed so path joins don't
    /// double up.
    ///
    /// # Errors
    ///
    /// [`ConfigError::InvalidToken`] if the token is empty or contains
    /// control characters; [`ConfigError::EmptyBaseUrl`] if the URL is blank.
    pub fn new(base_url: impl Into<String>, token: impl Into<String>) -> Result<Self, Error> {
        let token = token.into();
        if token.is_empty() || token.contains(['\r', '\n']) {
            return Err(Error::Config(ConfigError::InvalidToken));
        }
        Ok(Self {
            token,
            base_url: normalize_base_url(base_url.into())?,
        })
    }

    /// Create a GitLab client from environment variables.
    ///
    /// Expects `GITLAB_URL` and `GITLAB_PAT`.
    ///
    /// # Errors
    ///
    /// [`ConfigError::MissingUrlEnv`] / [`ConfigError::MissingTokenEnv`] if
    /// either variable is unset, or the same errors as [`Self::new`].
    pub fn from_env() -> Result<Self, Error> {
        let base_url =
            std::env::var("GITLAB_URL").map_err(|_| Error::Config(ConfigError::MissingUrlEnv))?;
        let token =
            std::env::var("GITLAB_PAT").map_err(|_| Error::Config(ConfigError::MissingTokenEnv))?;
        Self::new(base_url, token)
    }

    /// The base URL, normalized (scheme present, no trailing slash).
    #[must_use]
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// The `Authorization` header value.
    fn auth(&self) -> String {
        format!("Bearer {}", self.token)
    }

    /// Perform a GET request and parse the JSON body into `T`.
    pub(crate) fn get<T>(&self, path: &str) -> Result<T, Error>
    where
        T: for<'de> FromJson<'de>,
    {
        let url = format!("{}/{}", self.base_url, path);
        let response = gitlab_http::get(&url)
            .header("Authorization", self.auth())
            .send()?;

        if !response.is_success() {
            return Err(Error::Api {
                method: HttpMethod::Get,
                status: response.status,
            });
        }
        Ok(parse_str(&response.body)?)
    }

    /// Perform a POST request with a JSON body and parse the JSON response.
    // Scaffolding for write endpoints (create issue, comment, …) — not yet
    // exercised by a public method.
    #[allow(dead_code)]
    pub(crate) fn post<B, T>(&self, path: &str, body: &B) -> Result<T, Error>
    where
        B: ToJson,
        T: for<'de> FromJson<'de>,
    {
        let url = format!("{}/{}", self.base_url, path);
        let payload = json_bourne::to_vec(body)?;
        let response = gitlab_http::post(&url)
            .header("Authorization", self.auth())
            .json(payload)
            .send()?;

        if !response.is_success() {
            return Err(Error::Api {
                method: HttpMethod::Post,
                status: response.status,
            });
        }
        Ok(parse_str(&response.body)?)
    }

    /// Perform a paginated GET request with default config.
    pub(crate) fn get_paginated<T>(&self, path: &str) -> Result<Vec<T>, Error>
    where
        T: for<'de> FromJson<'de>,
    {
        let url = format!("{}/{}", self.base_url, path);
        self.paginate(&url, &PaginationConfig::default())
    }

    /// Execute a paginated GitLab request, aggregating all pages.
    pub(crate) fn paginate<T>(
        &self,
        url: &str,
        config: &PaginationConfig,
    ) -> Result<Vec<T>, Error>
    where
        T: for<'de> FromJson<'de>,
    {
        let auth = self.auth();
        let mut all_items = Vec::new();
        let mut page = 1;

        loop {
            if let Some(max) = config.max_pages
                && page > max
            {
                break;
            }

            let separator = if url.contains('?') { "&" } else { "?" };
            let paged_url =
                format!("{url}{separator}per_page={}&page={page}", config.per_page);

            let response = gitlab_http::get(&paged_url)
                .header("Authorization", &auth)
                .send()?;

            if !response.is_success() {
                return Err(Error::Api {
                    method: HttpMethod::Get,
                    status: response.status,
                });
            }

            let items: Vec<T> = parse_str(&response.body)?;
            let is_last_page = items.len() < config.per_page;
            all_items.extend(items);

            if is_last_page {
                break;
            }
            page += 1;
        }

        Ok(all_items)
    }
}

/// Normalize a GitLab base URL: add `https://` when no scheme is present
/// (xibalba requires an explicit scheme) and trim any trailing slash.
fn normalize_base_url(raw: String) -> Result<String, Error> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(Error::Config(ConfigError::EmptyBaseUrl));
    }
    let with_scheme = if trimmed.contains("://") {
        trimmed.to_string()
    } else {
        format!("https://{trimmed}")
    };
    Ok(with_scheme.trim_end_matches('/').to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation_ok() {
        assert!(GitlabClient::new("https://gitlab.example.com", "test-token").is_ok());
    }

    #[test]
    fn default_scheme_added() {
        let c = GitlabClient::new("gitlab.illuminodes.com", "t").unwrap();
        assert_eq!(c.base_url(), "https://gitlab.illuminodes.com");
    }

    #[test]
    fn explicit_scheme_preserved() {
        let c = GitlabClient::new("http://localhost:8080", "t").unwrap();
        assert_eq!(c.base_url(), "http://localhost:8080");
    }

    #[test]
    fn trailing_slash_trimmed() {
        let c = GitlabClient::new("https://gitlab.example.com/", "t").unwrap();
        assert_eq!(c.base_url(), "https://gitlab.example.com");
    }

    #[test]
    fn empty_url_rejected() {
        assert!(GitlabClient::new("   ", "t").is_err());
    }

    #[test]
    fn bad_token_rejected() {
        assert!(GitlabClient::new("https://gitlab.example.com", "bad\ntoken").is_err());
        assert!(GitlabClient::new("https://gitlab.example.com", "").is_err());
    }
}
