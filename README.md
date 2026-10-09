# kaj-tinamit

*Kaj Tinamit* — K'iche' (Maya) for "sky citadel / castle in the sky." ☁️🏰

A Rust workspace of tools for talking to a self-hosted GitLab instance,
built on an in-house, near-zero-dependency stack (no tokio, no reqwest,
no serde).

## Crates

| Crate | Role |
|-------|------|
| [`kaj-tinamit-http`](crates/kaj-tinamit-http)   | Blocking HTTP transport over the in-house `xibalba` client with a rustls connector. |
| [`kaj-tinamit`](crates/kaj-tinamit-model) | Type-safe wire shapes for the GitLab REST API v4, (de)serialized via `json-bourne`. |
| [`kaj-tinamit-client`](crates/kaj-tinamit-client) | Ties transport + models together: auth, pagination, and endpoint methods. |

Dependency direction: `kaj-tinamit-client → { kaj-tinamit-http, kaj-tinamit }`.
Wire shapes are kept in their own crate so the transport stays free of
API-surface churn.

## In-house / external dependencies

- **In-house** (published on crates.io): `json-bourne` (JSON),
  `xibalba-client` + `xibalba-proto` (HTTP).
- **External** (crates.io, unavoidable for TLS): `rustls`, `webpki-roots`,
  `rustls-rustcrypto`.

## Quick start

```rust
use kaj_tinamit_client::GitlabClient;
use kaj_tinamit_client::prelude::*; // endpoint extension traits

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
`use kaj_tinamit_client::prelude::*;`, or import individual traits
(`UserEndpoints`, `IssueEndpoints`, …).

## API coverage

Read (`GET`) endpoints across these API categories are implemented and
live-verified against a real instance. Write operations (`POST`/`PUT`/
`DELETE`) mirror the read surface for the categories marked ✎ below.

| Category | Endpoint trait(s) | Writes |
|----------|-------------------|:------:|
| Users | `UserEndpoints` | |
| Projects | `ProjectEndpoints` (+ `ProjectQuery`) | ✎ create/update/delete, archive, star, fork/unlink, transfer, share/unshare with group |
| Project webhooks | `HookEndpoints` | ✎ add/edit/delete, fire test event |
| Issues | `IssueEndpoints` (+ `IssueQuery`) | ✎ create/update/delete |
| Merge requests | `MergeRequestEndpoints` (+ `MergeRequestQuery`) | ✎ create/update/accept/delete |
| Commits | `CommitEndpoints` | ✎ create commit, cherry-pick, revert, set status |
| Milestones | `MilestoneEndpoints` | ✎ create/update/delete (project + group) |
| Events | `EventEndpoints` | |
| Wikis | `WikiEndpoints` | ✎ create/update/delete (project + group) |
| Repository (tree/blobs/branches/tags/files/blame) | `RepositoryEndpoints` | ✎ files, branches, tags |
| Groups & members | `GroupEndpoints`, `MemberEndpoints` | ✎ create/update/delete group; add/update/remove member |
| Labels | `LabelEndpoints` | ✎ create/update/delete (project + group) |
| CI (pipelines/jobs) | `PipelineEndpoints` (+ `PipelineQuery`), `JobEndpoints` (+ `JobQuery`) | ✎ create/retry/cancel/delete pipeline; play (with variables)/retry/cancel/erase job |
| Pipeline test reports | `TestReportEndpoints` | |
| CI lint | `LintEndpoints` | validates config (POST, no side effects) |
| Pipeline schedules | `PipelineScheduleEndpoints` | ✎ create/update/delete/play/take-ownership |
| Releases | `ReleaseEndpoints` | ✎ create/update/delete |
| Snippets | `SnippetEndpoints`, `ProjectSnippetEndpoints` | ✎ create/update/delete |
| Epics | `EpicEndpoints` | ✎ create/update/delete |
| Issue links | `IssueLinkEndpoints` | ✎ create/delete |
| Notes (all noteable types) | `NoteEndpoints` | ✎ create/update/delete |
| Emoji reactions | `AwardEmojiEndpoints` | ✎ award/remove |
| To-dos | `TodoEndpoints` | ✎ mark done / mark all done |
| Boards | `BoardEndpoints` | ✎ create/delete board + lists |
| Protected branches/tags | `ProtectedEndpoints` | ✎ protect/unprotect |
| Environments & deployments | `EnvironmentEndpoints`, `DeploymentEndpoints` | ✎ create/update/stop/delete environment; create/update/delete deployment |
| Runners | `RunnerEndpoints` (+ `RunnerQuery`) | ✎ update/pause/delete, assign/unassign project, reset token |
| Deploy keys & tokens | `DeployKeyEndpoints`, `DeployTokenEndpoints` | ✎ add/enable/delete key; create/delete token |
| CI/CD variables | `VariableEndpoints` (+ `VariableFilter`) | ✎ create/update/delete (project + group) |

Write bodies are small builder structs (`CreateIssue`, `UpdateMergeRequest`,
`CommitFile`, …) re-exported at the crate root. Optional fields are omitted
from the JSON payload when unset (via json-bourne's `skip_if_none`), so a
partial update touches only the fields you set.

## Testing

- **Unit tests** — example-based parse checks against documented JSON.
- **Property tests** (`cargo test -p kaj-tinamit --test properties`) —
  serialize/parse round-trip idempotence for every wire model, via
  `proptest` (behind the `proptest` feature).
- **Benchmarks** (`cargo bench -p kaj-tinamit`) — parse throughput per
  model and for paginated `Vec<T>` lists, via `divan`.
- **Fuzzing** (`cargo +nightly fuzz run parse_models --fuzz-dir
  crates/kaj-tinamit-model/fuzz`) — typed parses of arbitrary bytes never panic.
- **Live smoke tests** (`cargo test -p kaj-tinamit-client --test live --
  --ignored`) — hit a real instance; need `GITLAB_URL` + `GITLAB_PAT`.

## Environment

- `GITLAB_URL` — base URL of the instance (e.g. `https://gitlab.illuminodes.com`)
- `GITLAB_PAT` — personal access token

## Build

```sh
cargo build
cargo test
```
