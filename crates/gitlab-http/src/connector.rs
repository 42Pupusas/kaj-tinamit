//! rustls-backed connector for `xibalba-client`.
//!
//! Plain HTTP is kept alongside TLS so tests can target a local mock
//! server. Adapted from xibalba's `tcp-rustls` example.

use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::Arc;
use std::time::Duration;

use rustls::pki_types::ServerName;
use rustls::{ClientConfig, ClientConnection, StreamOwned};
use xibalba_client::connector::{Connector, SetReadTimeout};
use xibalba_proto::error::{ConnectionError, Error, TlsError};
use xibalba_proto::scheme::Scheme;
use xibalba_proto::url::Url;

// A TCP connect (and the TLS handshake / request write that follow it)
// must be bounded, or a dead/flaky network wedges the caller forever.
// The per-read ceiling on the client only covers reads *after* the
// socket is up; connect and write need their own bound.
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const WRITE_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug)]
pub enum Stream {
    Plain(TcpStream),
    Tls(Box<StreamOwned<ClientConnection, TcpStream>>),
}

impl SetReadTimeout for Stream {
    fn set_read_timeout(&self, dur: Option<Duration>) -> std::io::Result<()> {
        match self {
            Self::Plain(tcp) => tcp.set_read_timeout(dur),
            Self::Tls(tls) => tls.get_ref().set_read_timeout(dur),
        }
    }
}

impl Read for Stream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Self::Plain(tcp) => tcp.read(buf),
            Self::Tls(tls) => tls.read(buf),
        }
    }
}

impl Write for Stream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Self::Plain(tcp) => tcp.write(buf),
            Self::Tls(tls) => tls.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Self::Plain(tcp) => tcp.flush(),
            Self::Tls(tls) => tls.flush(),
        }
    }
}

#[derive(Debug)]
pub struct TlsConnector;

impl Connector for TlsConnector {
    type Stream = Stream;
    type TlsConfig = Arc<ClientConfig>;

    fn connect(url: &Url<'_>, tls_config: &Arc<ClientConfig>) -> Result<Stream, Error> {
        let host_str = std::str::from_utf8(url.host).map_err(|_| {
            Error::Connection(ConnectionError::Other("invalid UTF-8 in host".into()))
        })?;

        let port = url.effective_port();
        let addrs = (host_str, port).to_socket_addrs().map_err(|e| {
            Error::Connection(ConnectionError::Other(format!("DNS resolution failed: {e}")))
        })?;

        // Try each resolved address with a bounded connect, so a single
        // dead IP (common with dual-stack A/AAAA records) falls through
        // to the next instead of hanging on the OS default timeout.
        let mut tcp = None;
        let mut last_err = None;
        for addr in addrs {
            match TcpStream::connect_timeout(&addr, CONNECT_TIMEOUT) {
                Ok(stream) => {
                    tcp = Some(stream);
                    break;
                }
                Err(e) => last_err = Some(e),
            }
        }
        let tcp = tcp.ok_or_else(|| {
            last_err.map_or_else(
                || Error::Connection(ConnectionError::Other("DNS returned no addresses".into())),
                Error::from,
            )
        })?;

        // Bound writes too: the TLS handshake and request send happen on
        // this socket, and a half-open connection would otherwise block
        // `write_all` indefinitely.
        let _ = tcp.set_write_timeout(Some(WRITE_TIMEOUT));

        match url.scheme {
            Scheme::Https => {
                let server_name = ServerName::try_from(host_str.to_owned()).map_err(|e| {
                    Error::Connection(ConnectionError::Other(format!("invalid server name: {e}")))
                })?;
                let conn =
                    ClientConnection::new(Arc::clone(tls_config), server_name).map_err(|e| {
                        TlsError {
                            message: e.to_string(),
                        }
                    })?;
                Ok(Stream::Tls(Box::new(StreamOwned::new(conn, tcp))))
            }
            Scheme::Http => Ok(Stream::Plain(tcp)),
        }
    }
}

/// Build a rustls client config trusting the webpki root set.
///
/// # Errors
///
/// Returns `Error` if rustls rejects the protocol-version setup.
pub fn build_tls_config() -> Result<Arc<ClientConfig>, Error> {
    let provider = rustls_rustcrypto::provider();
    let mut root_store = rustls::RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let config = ClientConfig::builder_with_provider(Arc::new(provider))
        .with_safe_default_protocol_versions()
        .map_err(|e| TlsError {
            message: e.to_string(),
        })?
        .with_root_certificates(root_store)
        .with_no_client_auth();
    Ok(Arc::new(config))
}
