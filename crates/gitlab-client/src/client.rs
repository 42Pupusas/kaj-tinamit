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

    /// Perform a GET request and return the raw response body as text.
    ///
    /// For endpoints that don't return JSON — raw blobs, raw files, plain
    /// changelog text. Binary payloads (archives) are lossily decoded as
    /// UTF-8 by the transport and are not supported here.
    pub(crate) fn get_raw(&self, path: &str) -> Result<String, Error> {
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
        Ok(response.body)
    }

    /// Perform a POST request with a JSON body and parse the JSON response.
    pub(crate) fn post<B, T>(&self, path: &str, body: &B) -> Result<T, Error>
    where
        B: ToJson,
        T: for<'de> FromJson<'de>,
    {
        self.send_with_body(HttpMethod::Post, path, body)
    }

    /// Perform a POST request with no request body, parsing the JSON response.
    ///
    /// For action-style endpoints (e.g. cancel a pipeline, mark a to-do
    /// done) where GitLab takes the target from the path, not a payload.
    pub(crate) fn post_no_body<T>(&self, path: &str) -> Result<T, Error>
    where
        T: for<'de> FromJson<'de>,
    {
        self.send_no_body(HttpMethod::Post, path)
    }

    /// Perform a PUT request with a JSON body and parse the JSON response.
    pub(crate) fn put<B, T>(&self, path: &str, body: &B) -> Result<T, Error>
    where
        B: ToJson,
        T: for<'de> FromJson<'de>,
    {
        self.send_with_body(HttpMethod::Put, path, body)
    }

    /// Perform a bodyless POST whose response is discarded (204-style
    /// action endpoints, e.g. “mark all to-dos done”).
    pub(crate) fn post_discard(&self, path: &str) -> Result<(), Error> {
        let url = format!("{}/{}", self.base_url, path);
        let response = gitlab_http::post(&url)
            .header("Authorization", self.auth())
            .send()?;

        if !response.is_success() {
            return Err(Error::Api {
                method: HttpMethod::Post,
                status: response.status,
            });
        }
        Ok(())
    }

    /// Perform a DELETE request, discarding any response body.
    ///
    /// GitLab returns `204 No Content` for most deletes, so there is
    /// nothing to parse — a 2xx status is success.
    pub(crate) fn delete(&self, path: &str) -> Result<(), Error> {
        let url = format!("{}/{}", self.base_url, path);
        let response = gitlab_http::delete(&url)
            .header("Authorization", self.auth())
            .send()?;

        if !response.is_success() {
            return Err(Error::Api {
                method: HttpMethod::Delete,
                status: response.status,
            });
        }
        Ok(())
    }

    /// Perform a DELETE request and parse the JSON response body into `T`.
    ///
    /// A few deletes (e.g. removing an issue link) echo the affected
    /// objects instead of returning `204`.
    pub(crate) fn delete_with_response<T>(&self, path: &str) -> Result<T, Error>
    where
        T: for<'de> FromJson<'de>,
    {
        let url = format!("{}/{}", self.base_url, path);
        let response = gitlab_http::delete(&url)
            .header("Authorization", self.auth())
            .send()?;

        if !response.is_success() {
            return Err(Error::Api {
                method: HttpMethod::Delete,
                status: response.status,
            });
        }
        Ok(parse_str(&response.body)?)
    }

    /// Shared body of [`Self::post`] / [`Self::put`]: serialize `body`,
    /// send it with `method`, and parse the JSON response into `T`.
    fn send_with_body<B, T>(&self, method: HttpMethod, path: &str, body: &B) -> Result<T, Error>
    where
        B: ToJson,
        T: for<'de> FromJson<'de>,
    {
        let url = format!("{}/{}", self.base_url, path);
        let payload = json_bourne::to_vec(body)?;
        let request = match method {
            HttpMethod::Put => gitlab_http::put(&url),
            _ => gitlab_http::post(&url),
        };
        let response = request
            .header("Authorization", self.auth())
            .json(payload)
            .send()?;

        if !response.is_success() {
            return Err(Error::Api {
                method,
                status: response.status,
            });
        }
        Ok(parse_str(&response.body)?)
    }

    /// Shared body for bodyless POST/PUT actions that still return JSON.
    fn send_no_body<T>(&self, method: HttpMethod, path: &str) -> Result<T, Error>
    where
        T: for<'de> FromJson<'de>,
    {
        let url = format!("{}/{}", self.base_url, path);
        let request = match method {
            HttpMethod::Put => gitlab_http::put(&url),
            _ => gitlab_http::post(&url),
        };
        let response = request.header("Authorization", self.auth()).send()?;

        if !response.is_success() {
            return Err(Error::Api {
                method,
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
    pub(crate) fn paginate<T>(&self, url: &str, config: &PaginationConfig) -> Result<Vec<T>, Error>
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
            let paged_url = format!("{url}{separator}per_page={}&page={page}", config.per_page);

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
