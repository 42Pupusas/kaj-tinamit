# Changelog

All notable changes to this workspace are documented here. The three crates
(`kaj-tinamit`, `kaj-tinamit-http`, `kaj-tinamit-client`) share one version.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [0.1.1] - 2026-08-11

### Added

- **CI/CD variables** (`kaj-tinamit`, `kaj-tinamit-client`) — the stored
  settings behind Settings → CI/CD → Variables, for both projects and groups.
  - `CiVariable`, `VariableType`, and `VariableVisibility` wire models.
  - `VariableEndpoints`: list, get, create, update, and delete.
  - `CreateVariable` / `UpdateVariable` builders and `VariableFilter`.
  - `VariableFilter` carries `filter[environment_scope]`, required because a
    key may repeat across environment scopes, making by-key calls ambiguous
    without it.
  - `VariableVisibility` collapses GitLab's `masked` / `hidden` booleans into
    the three legal states; `hidden` implies `masked`, and a hidden value is
    never returned by the API after creation, so `CiVariable::value` is
    `Option`.

### Changed

- `kaj-tinamit-http` now requires `xibalba-client` 0.3, which replaces
  retry-count caps on read paths with wall-clock silence budgets. A stalled
  read surfaces as a descriptive `TimedOut` rather than the socket's raw
  `EAGAIN` ("os error 11"), and a failed reconnect is a hard error instead of
  silently desyncing the keep-alive stream.

## [0.1.0] - 2026-07-22

### Added

- Initial release: `kaj-tinamit` (wire models), `kaj-tinamit-http` (blocking
  transport over xibalba with a rustls connector), and `kaj-tinamit-client`
  (the typed GitLab REST API v4 client) covering 26 API categories.

[0.1.1]: https://github.com/42Pupusas/kaj-tinamit/releases/tag/v0.1.1
[0.1.0]: https://github.com/42Pupusas/kaj-tinamit/releases/tag/v0.1.0
