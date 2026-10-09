# Changelog

All notable changes to this workspace are documented here. The three crates
(`kaj-tinamit`, `kaj-tinamit-http`, `kaj-tinamit-client`) share one version.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [0.2.0] - 2026-10-08

### Added

- **Projects**
  - `GitlabProject` gains `default_branch`, `web_url`, `http_url_to_repo`,
    `ssh_url_to_repo`, `readme_url`, `avatar_url`, `topics`, `star_count`,
    `forks_count`, `empty_repo`, `ci_config_path`, `merge_method`, the
    per-feature `*_access_level` fields, `forked_from_project`, and
    `permissions`. New models: `FeatureAccess`, `MergeMethod`, `ProjectRef`,
    `ProjectPermissions`, `AccessGrant`.
  - `ProjectQuery` filters for `projects()`: membership, owned, starred,
    search, archived, visibility, topic, minimum access level, and ordering
    (`ProjectOrder`, `SortDirection`).
  - `forks`, `fork_project` (`ForkProject`), `unlink_fork`,
    `transfer_project`, `share_project_with_group`,
    `unshare_project_with_group`.
  - `CreateProject` / `UpdateProject` set topics, `ci_config_path`, merge
    method, feature access levels, and merge rules.
  - `HookEndpoints`: list, get, add, edit, delete, and test project webhooks
    (`ProjectHook`, `HookSettings`, `HookEvent`).
- **Groups:** `create_group` (`CreateGroup`, including subgroups),
  `update_group` (`UpdateGroup`), `delete_group`.
- **CI**
  - `JobQuery` for `jobs`, `pipeline_jobs`, and `pipeline_trigger_jobs`:
    filter by one or more `CiStatus` values, and `include_retried`.
  - `play_job_with` runs a manual job with `JobVariable`s.
  - `Job::downstream_pipeline` (`DownstreamPipeline`) on trigger jobs.
  - `TestReportEndpoints`: `pipeline_test_report` and
    `pipeline_test_report_summary` (`TestReport`, `TestSuite`, `TestCase`,
    `TestReportSummary`, `TestTotals`); `TestSuite::broken_cases`.
  - `LintEndpoints`: `lint_ci_config` (`CiLint`, optionally simulated on a
    ref) and `lint_project_ci` (`CiLintResult`).
  - `CiStatus::as_str`, `Visibility::as_str`.

### Changed

- **Breaking:** every project, issue, milestone, group, wiki, and user id
  parameter is `i64`. `project`, `update_project`, `delete_project`,
  `archive_project`, `unarchive_project`, `star_project`, `unstar_project`,
  the issue, milestone, and wiki endpoints, and `user_by_id` took `i32`.
- **Breaking:** `projects()` takes a `&ProjectQuery`. The old call listed
  member projects only; pass `ProjectQuery::new().membership()` for the same
  result.
- **Breaking:** `jobs`, `pipeline_jobs`, and `pipeline_trigger_jobs` take a
  `&JobQuery`; `JobQuery::new()` keeps the old behaviour.
- **Breaking:** `CreateProject` / `UpdateProject` take `Visibility` instead
  of a string, and moved into their own module (still re-exported from the
  crate root).

- **Breaking:** `json-bourne` is now 0.3 (was 0.2). Its types are part of the
  public API: `kaj_tinamit_client::Error::Json` wraps `json_bourne::Error`, and
  every wire model implements `json_bourne::FromJson` / `ToJson`. Downstream
  crates that name those types must move to `json-bourne` 0.3 as well, or they
  will see two incompatible copies in their dependency graph.

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

[0.2.0]: https://github.com/42Pupusas/kaj-tinamit/releases/tag/v0.2.0
[0.1.1]: https://github.com/42Pupusas/kaj-tinamit/releases/tag/v0.1.1
[0.1.0]: https://github.com/42Pupusas/kaj-tinamit/releases/tag/v0.1.0
