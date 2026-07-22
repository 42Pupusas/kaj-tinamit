# kaj-tinamit

Type-safe wire shapes for the **GitLab REST API v4**, (de)serialized via the
in-house [`json-bourne`] codec. Pure data — no transport, no I/O. Pair it with
`kaj-tinamit-client` for the HTTP side, or use it standalone to parse GitLab JSON.

## Design

The whole crate is built around one goal: **GitLab evolving its API never
breaks your parse.** Two independent failure modes, two guarantees:

- **New fields.** Every struct is `#[bourne(deny_unknown_fields = false)]` and
  `#[non_exhaustive]`. GitLab adding a JSON field is ignored; we adding a Rust
  field is a non-breaking change.
- **New enum values.** Every closed-set enum (`Visibility`, `UserState`,
  `MergeStatus`, `PipelineStatus`, `IssueState`, …) carries an `Unknown`
  fallback variant and is `#[non_exhaustive]`. A status GitLab ships next
  quarter deserializes to `Unknown` instead of failing the entire object.

Object identifiers are the [`Id`] newtype — a transparent `i64`, never a bare
integer and never `i32` (GitLab.com ids left 2³¹ behind years ago). `Id`
serializes as a plain JSON number, compares directly against `i64`
(`assert_eq!(user.id, 42)`), and converts both ways via `From`.

## Example

```rust
use kaj_tinamit::{GitlabUser, UserState, Id};
use json_bourne::parse_str;

let json = r#"{
    "id": 42,
    "username": "alice",
    "name": "Alice",
    "state": "active",
    "locked": false,
    "avatar_url": "https://example.com/a.png",
    "web_url": "https://example.com/alice"
}"#;

let user: GitlabUser = parse_str(json).unwrap();
assert_eq!(user.id, Id::new(42));
assert_eq!(user.id, 42);              // Id compares against i64
assert_eq!(user.state, UserState::Active);

// A state GitLab hasn't shipped yet parses, it doesn't panic:
let future = r#"{ "id": 1, "username": "x", "name": "X", "state": "hibernating",
                  "locked": false, "avatar_url": "", "web_url": "" }"#;
let u: GitlabUser = parse_str(future).unwrap();
assert_eq!(u.state, UserState::Unknown);
```

## Coverage

Wire shapes for the read (and, via `kaj-tinamit-client`, write) surface of ~20 API
categories: users, projects, issues, merge requests, commits, milestones,
events, wikis, repository (trees/blobs/branches/tags/blame), groups & members,
labels, CI (pipelines/jobs), releases, snippets, environments & deployments,
runners, notes, deploy keys/tokens, boards, epics, iterations, issue links,
resource events, todos, and more.

## Features

- `proptest` *(off by default)* — adds `proptest::Arbitrary` impls to every
  model, used by the property-based round-trip test suite. Off for normal
  builds so it adds no dependencies.

## Testing

```sh
cargo test -p kaj-tinamit          # unit + property round-trips + doctests
```

The property suite (`tests/properties.rs`) round-trips every type through
serialize → parse and asserts equality, so the hand-rolled enum
`FromJson`/`ToJson` impls can't silently drift from each other.

## License

MIT OR Apache-2.0.

[`json-bourne`]: https://docs.rs/json-bourne
[`Id`]: https://docs.rs/kaj-tinamit/latest/kaj_tinamit/id/struct.Id.html
