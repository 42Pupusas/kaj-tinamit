# kaj-tinamit

*Kaj Tinamit* — K'iche' (Maya) for "sky citadel / castle in the sky." ☁️🏰

A Rust workspace of tools for talking to a self-hosted GitLab instance,
built on an in-house, near-zero-dependency stack (no tokio, no reqwest,
no serde).

## Crates

| Crate | Role |
|-------|------|
| [`gitlab-http`](crates/gitlab-http)   | Blocking HTTP transport over the in-house `xibalba` client with a rustls connector. |
| [`gitlab-model`](crates/gitlab-model) | Type-safe wire shapes for the GitLab REST API v4, (de)serialized via `json-bourne`. |
| [`gitlab-client`](crates/gitlab-client) | Ties transport + models together: auth, pagination, and endpoint methods. |

Dependency direction: `gitlab-client → { gitlab-http, gitlab-model }`.
Wire shapes are kept in their own crate so the transport stays free of
API-surface churn.

## In-house / external dependencies

- **In-house** (path deps in `../`): `json-bourne` (JSON), `xibalba-client`
  + `xibalba-proto` (HTTP).
- **External** (crates.io, unavoidable for TLS): `rustls`, `webpki-roots`,
  `rustls-rustcrypto`.

## Quick start

```rust
use gitlab_client::GitlabClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Reads GITLAB_URL and GITLAB_PAT from the environment.
    let client = GitlabClient::from_env()?;
    let me = client.current_user()?;
    println!("authenticated as {}", me.username);
    Ok(())
}
```

## Environment

- `GITLAB_URL` — base URL of the instance (e.g. `https://gitlab.illuminodes.com`)
- `GITLAB_PAT` — personal access token

## Build

```sh
cargo build
cargo test
```
