//! # kaj-tinamit-http
//!
//! Blocking HTTP transport over [`xibalba-client`](xibalba_client) with a
//! rustls connector. No tokio, no reqwest — one OS thread per caller,
//! plain `Read`/`Write` sockets underneath.
//!
//! The primary entry point is [`Request`]: a tiny builder that opens a
//! fresh connection per call (a handful of requests per run — keep-alive
//! plumbing isn't worth its state).
//!
//! ```no_run
//! # fn run() -> Result<(), kaj_tinamit_http::Error> {
//! let resp = kaj_tinamit_http::get("https://example.com/api/v4/user")
//!     .header("Authorization", "Bearer token")
//!     .send()?;
//! assert_eq!(resp.status, 200);
//! println!("{}", resp.body);
//! # Ok(()) }
//! ```

mod connector;

use std::sync::{Arc, OnceLock};
use std::time::Duration;

use connector::TlsConnector;
use xibalba_client::client::{Client, Config};

pub use xibalba_proto::method::Method;

/// Errors from an HTTP request.
#[derive(Debug)]
pub enum Error {
    /// URL failed to parse.
    Url(String),
    /// Transport failure: DNS, TCP, TLS, or protocol decode.
    Transport(String),
    /// The response body was not valid UTF-8.
    Body(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Url(m) => write!(f, "invalid url: {m}"),
            Self::Transport(m) => write!(f, "transport error: {m}"),
            Self::Body(m) => write!(f, "body error: {m}"),
        }
    }
}

impl std::error::Error for Error {}

/// A completed HTTP exchange: status and full UTF-8 body.
#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub body: String,
}

impl Response {
    /// Whether the status is 2xx.
    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }
}

/// The shared TLS configuration, built once per process.
fn tls_config() -> Result<Arc<rustls::ClientConfig>, Error> {
    static TLS: OnceLock<Arc<rustls::ClientConfig>> = OnceLock::new();
    if let Some(cfg) = TLS.get() {
        return Ok(Arc::clone(cfg));
    }
    let cfg = connector::build_tls_config().map_err(|e| Error::Transport(e.to_string()))?;
    Ok(Arc::clone(TLS.get_or_init(|| cfg)))
}

/// A blocking request builder. Construct via [`get`] / [`post`] /
/// [`request`], chain headers/body, then [`send`](Request::send).
#[derive(Debug)]
pub struct Request {
    method: Method,
    url: String,
    headers: Vec<(String, String)>,
    body: Option<Vec<u8>>,
    read_timeout: Duration,
}

impl Request {
    #[must_use]
    pub fn new(method: Method, url: impl Into<String>) -> Self {
        Self {
            method,
            url: url.into(),
            headers: Vec::new(),
            body: None,
            read_timeout: Duration::from_secs(30),
        }
    }

    /// Add a header.
    #[must_use]
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    /// Set a raw body.
    #[must_use]
    pub fn body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Set a JSON body (adds the `Content-Type` header).
    #[must_use]
    pub fn json(self, json: impl Into<Vec<u8>>) -> Self {
        self.header("Content-Type", "application/json").body(json)
    }

    /// Override the per-read timeout (default 30 s).
    #[must_use]
    pub const fn read_timeout(mut self, dur: Duration) -> Self {
        self.read_timeout = dur;
        self
    }

    /// Execute the request on a fresh connection, following redirects,
    /// and buffer the full response.
    ///
    /// # Errors
    ///
    /// [`Error::Url`] on a malformed URL, [`Error::Transport`] on any
    /// connection/protocol failure, [`Error::Body`] on non-UTF-8 body.
    /// Non-2xx statuses are NOT errors — check [`Response::status`].
    pub fn send(self) -> Result<Response, Error> {
        let tls = tls_config()?;

        // Split the URL into origin + path&query for xibalba: connect
        // wants the origin, build wants the path.
        let url_bytes = self.url.as_bytes().to_vec();
        let parsed = xibalba_proto::url::Url::parse(&url_bytes)
            .map_err(|e| Error::Url(format!("{}: {e}", self.url)))?;
        let path = if parsed.path.is_empty() {
            b"/".to_vec()
        } else {
            parsed.path.to_vec()
        };
        let query = parsed.query.map(<[u8]>::to_vec);

        let config = Config {
            read_timeout: Some(self.read_timeout),
            ..Config::default()
        };
        let mut client = Client::<TlsConnector>::connect(&url_bytes, tls, config)
            .map_err(|e| Error::Transport(e.to_string()))?;

        let mut builder = client.build(self.method, &path);
        if let Some(q) = &query {
            builder = builder.query(q);
        }
        for (name, value) in &self.headers {
            builder = builder.header(name.as_bytes(), value.as_bytes());
        }
        if let Some(body) = &self.body {
            builder = builder.body(body);
        }

        let response = client
            .send(builder)
            .map_err(|e| Error::Transport(e.to_string()))?;

        let status = response.status.as_u16();
        let body = response.text().map_err(|e| Error::Body(e.to_string()))?;
        Ok(Response { status, body })
    }
}

/// Begin a GET request.
#[must_use]
pub fn get(url: impl Into<String>) -> Request {
    Request::new(Method::Get, url)
}

/// Begin a POST request.
#[must_use]
pub fn post(url: impl Into<String>) -> Request {
    Request::new(Method::Post, url)
}

/// Begin a PUT request.
#[must_use]
pub fn put(url: impl Into<String>) -> Request {
    Request::new(Method::Put, url)
}

/// Begin a DELETE request.
#[must_use]
pub fn delete(url: impl Into<String>) -> Request {
    Request::new(Method::Delete, url)
}

/// Begin a request with an arbitrary method.
#[must_use]
pub fn request(method: Method, url: impl Into<String>) -> Request {
    Request::new(method, url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_success_ranges() {
        let ok = |s| {
            Response {
                status: s,
                body: String::new(),
            }
            .is_success()
        };
        assert!(ok(200));
        assert!(ok(204));
        assert!(!ok(301));
        assert!(!ok(404));
        assert!(!ok(500));
    }

    #[test]
    fn builder_collects_parts() {
        let req = post("https://example.com/api")
            .header("Authorization", "Bearer x")
            .json(br#"{"a":1}"#.to_vec());
        assert_eq!(req.headers.len(), 2); // auth + content-type
        assert!(req.body.is_some());
    }
}
