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
use gitlab_client::prelude::*; // endpoint extension traits

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Reads GITLAB_URL and GITLAB_PAT from the environment.
    let client = GitlabClient::from_env()?;
    let me = client.current_user()?;
    println!("authenticated as {}", me.username);
    Ok(())
}
```

Every endpoint lives on an extension trait implemented for `GitlabClient`
(grouped by resource). Bring them all into scope with
`use gitlab_client::prelude::*;`, or import individual traits
(`UserEndpoints`, `IssueEndpoints`, …).

## API coverage

Read (`GET`) endpoints across these API categories are implemented and
live-verified against a real instance. Write operations are intentionally
out of scope.

| Category | Endpoint trait(s) |
|----------|-------------------|
| Users | `UserEndpoints` |
| Projects | `ProjectEndpoints` |
| Issues | `IssueEndpoints` (+ `IssueQuery`) |
| Merge requests | `MergeRequestEndpoints` (+ `MergeRequestQuery`) |
| Commits | `CommitEndpoints` |
| Milestones | `MilestoneEndpoints` |
| Events | `EventEndpoints` |
| Wikis | `WikiEndpoints` |
| Repository (tree/blobs/branches/tags/files/blame) | `RepositoryEndpoints` |
| Groups & members | `GroupEndpoints`, `MemberEndpoints` |
| Labels | `LabelEndpoints` |
| CI (pipelines/jobs) | `PipelineEndpoints` (+ `PipelineQuery`), `JobEndpoints` |
| Releases | `ReleaseEndpoints` |
| Snippets | `SnippetEndpoints`, `ProjectSnippetEndpoints` |
| Environments & deployments | `EnvironmentEndpoints`, `DeploymentEndpoints` |
| Runners | `RunnerEndpoints` (+ `RunnerQuery`) |
| Notes (all noteable types) | `NoteEndpoints` |
| Deploy keys & tokens | `DeployKeyEndpoints`, `DeployTokenEndpoints` |

## Testing

- **Unit tests** — example-based parse checks against documented JSON.
- **Property tests** (`cargo test -p gitlab-model --test properties`) —
  serialize/parse round-trip idempotence for every wire model, via
  `proptest` (behind the `proptest` feature).
- **Benchmarks** (`cargo bench -p gitlab-model`) — parse throughput per
  model and for paginated `Vec<T>` lists, via `divan`.
- **Fuzzing** (`cargo +nightly fuzz run parse_models --fuzz-dir
  crates/gitlab-model/fuzz`) — typed parses of arbitrary bytes never panic.
- **Live smoke tests** (`cargo test -p gitlab-client --test live --
  --ignored`) — hit a real instance; need `GITLAB_URL` + `GITLAB_PAT`.

## Environment

- `GITLAB_URL` — base URL of the instance (e.g. `https://gitlab.illuminodes.com`)
- `GITLAB_PAT` — personal access token

## Build

```sh
cargo build
cargo test
```
