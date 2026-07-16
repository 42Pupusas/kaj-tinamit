//! Parse-throughput benchmarks for every top-level wire model, via `divan`.
//!
//! One representative JSON blob per model, parsed in a black-boxed loop. Run
//! with `cargo bench -p gitlab-model`. These guard against parse-path
//! regressions in `json-bourne` and in our derive usage.

use gitlab_model::{
    AwardEmoji, Blob, BlameRange, Board, BoardList, Branch, Changelog, CommitStatus,
    CommitWithDiffs, Contributor, CreatedDeployToken, DeployKey, DeployToken, Deployment,
    Discussion, Environment,
    Epic, EpicIssue, FileMutationResult, GitlabCommit, GitlabEvent, GitlabGroup, GitlabIssue,
    GitlabNote, GitlabProject, GitlabUser, IssueLink, IssueLinkResult, IssueStatistics, Iteration,
    IterationCadence, Job, Label, Member, MergeRequest, MergeRequestApprovals,
    MergeRequestChanges, Metadata, Milestone, NamespaceListing, PipelineDetail, PipelineSchedule,
    PipelineSummary, PipelineVariable, ProtectedBranch, ProtectedEnvironment, ProtectedTag,
    RefCommit, Release, RepositoryFile, ResourceLabelEvent, ResourceMilestoneEvent,
    ResourceStateEvent, Runner, RunnerAuthToken, RunnerDetail, RunnerManager, SearchBlob, Snippet,
    SnippetUserAgentDetail, Tag, Todo, TreeEntry, WikiPage, WikiPageList,
};
use json_bourne::parse_str;

fn main() {
    divan::main();
}

/// Declare a `#[divan::bench]` that parses `$json` into `$ty`.
macro_rules! bench_parse {
    ($name:ident, $ty:ty, $json:expr) => {
        #[divan::bench]
        fn $name(bencher: divan::Bencher) {
            bencher.bench(|| {
                let v: $ty = parse_str(divan::black_box($json)).unwrap();
                divan::black_box(v);
            });
        }
    };
}

/// Declare a `#[divan::bench]` that parses a JSON array of `count` copies of
/// `$json` into `Vec<$ty>` — the paginated hot path.
macro_rules! bench_parse_list {
    ($name:ident, $ty:ty, $count:expr, $json:expr) => {
        #[divan::bench]
        fn $name(bencher: divan::Bencher) {
            let arr = {
                let mut s = String::from("[");
                for i in 0..$count {
                    if i > 0 {
                        s.push(',');
                    }
                    s.push_str($json);
                }
                s.push(']');
                s
            };
            bencher.bench(|| {
                let v: Vec<$ty> = parse_str(divan::black_box(&arr)).unwrap();
                divan::black_box(v);
            });
        }
    };
}

bench_parse!(
    user,
    GitlabUser,
    r#"{"id":42,"username":"alice","name":"Alice Example","state":"active",
    "locked":false,"avatar_url":"https://example.com/a.png","web_url":"https://example.com/alice"}"#
);

bench_parse!(
    project,
    GitlabProject,
    r#"{"id":42,"description":"A project","visibility":"private","name":"widget",
    "name_with_namespace":"Group / widget","path":"widget","path_with_namespace":"group/widget",
    "open_issues_count":3,"created_at":"2021-01-01T00:00:00Z","last_activity_at":"2022-01-01T00:00:00Z",
    "creator_id":1,"namespace":{"id":5,"name":"Group","path":"group","kind":"group","full_path":"group"},
    "archived":false}"#
);

bench_parse!(
    issue,
    GitlabIssue,
    r#"{"id":1,"iid":7,"project_id":3,"title":"Something broke",
    "description":"A fairly long description spanning\nmultiple lines with detail.",
    "state":"opened","type":"ISSUE","labels":["bug","backend","priority::high"],
    "author":{"id":2,"username":"alice","name":"Alice","state":"active"},
    "assignees":[{"id":3,"username":"bob","name":"Bob","state":"active"}],
    "upvotes":5,"downvotes":0,"user_notes_count":12,
    "web_url":"https://gitlab.example.com/g/p/-/issues/7",
    "created_at":"2022-01-01T00:00:00Z","updated_at":"2022-02-01T00:00:00Z"}"#
);

bench_parse!(
    merge_request,
    MergeRequest,
    r#"{"id":99,"iid":12,"title":"Add feature","description":"Implements the thing",
    "state":"opened","created_at":"2022-01-01T00:00:00Z","updated_at":"2022-01-02T00:00:00Z",
    "target_branch":"main","source_branch":"feature/x",
    "author":{"id":2,"username":"alice","name":"Alice"},
    "labels":["backend"],"merge_status":"can_be_merged","sha":"abc123",
    "web_url":"https://example.com/-/merge_requests/12"}"#
);

bench_parse!(
    commit,
    GitlabCommit,
    r#"{"id":"0ff3ae198f8601a285adcf5c0fff204ee6fba5fd","short_id":"0ff3ae19",
    "title":"Fix the thing","message":"Fix the thing\n\nDetail here","author_name":"Alice",
    "author_email":"alice@example.com","authored_date":"2022-01-01T00:00:00Z",
    "committer_name":"Alice","committer_email":"alice@example.com",
    "committed_date":"2022-01-01T00:00:00Z","created_at":"2022-01-01T00:00:00Z",
    "parent_ids":["a1b2c3"],"web_url":"https://example.com/-/commit/0ff3ae19"}"#
);

bench_parse!(
    event,
    GitlabEvent,
    r#"{"id":101,"action_name":"pushed to","target_type":"Note",
    "created_at":"2022-01-01T00:00:00Z","author_username":"alice",
    "author":{"id":2,"username":"alice","name":"Alice"}}"#
);

bench_parse!(
    wiki_page,
    WikiPage,
    r##"{"slug":"home","title":"Home","content":"# Welcome\n\nSome wiki content here.",
    "format":"markdown","encoding":"UTF-8"}"##
);

bench_parse!(
    branch,
    Branch,
    r#"{"name":"main","merged":false,"protected":true,"default":true,"can_push":true,
    "web_url":"https://example.com/-/tree/main",
    "commit":{"id":"7b5c3cc8","short_id":"7b5c3cc","title":"init"}}"#
);

bench_parse!(
    tag,
    Tag,
    r#"{"name":"v1.0.0","message":"release","target":"2695effb","protected":true,
    "created_at":"2022-01-01T00:00:00Z","commit":{"id":"2695effb","short_id":"2695effb"},
    "release":{"tag_name":"v1.0.0","description":"Amazing release"}}"#
);

bench_parse!(
    tree_entry,
    TreeEntry,
    r#"{"id":"a1e8f8d7","name":"src","type":"tree","path":"src","mode":"040000"}"#
);

bench_parse!(
    blob,
    Blob,
    r#"{"size":1476,"encoding":"base64","content":"VGhpcyBpcyBhIGJpbmFyeSBmaWxl","sha":"79f7bbd2"}"#
);

bench_parse!(
    repository_file,
    RepositoryFile,
    r#"{"file_name":"key.rb","file_path":"app/models/key.rb","size":1476,"encoding":"base64",
    "content":"IyBzY2hlbWE=","content_sha256":"4c294617","ref":"main","blob_id":"79f7bbd2",
    "commit_id":"d5a3ff13","last_commit_id":"570e7b2a","execute_filemode":false}"#
);

bench_parse!(
    group,
    GitlabGroup,
    r#"{"id":4,"name":"Twitter","path":"twitter","description":"desc","visibility":"public",
    "full_path":"twitter","parent_id":null,"created_at":"2020-01-15T12:36:29Z",
    "shared_with_groups":[{"group_id":28,"group_name":"H5bp","group_access_level":20,"expires_at":null}]}"#
);

bench_parse!(
    member,
    Member,
    r#"{"id":1,"username":"raymond_smith","name":"Raymond Smith","state":"active",
    "avatar_url":"https://example.com/a.png","web_url":"https://example.com/raymond",
    "access_level":30,"expires_at":"2025-10-22","created_at":"2012-09-22T14:13:35Z"}"#
);

bench_parse!(
    label,
    Label,
    r##"{"id":1,"name":"bug","color":"#d9534f","text_color":"#FFFFFF",
    "description":"Bug reported by user","open_issues_count":1,"closed_issues_count":0,
    "open_merge_requests_count":1,"subscribed":false,"priority":10,"is_project_label":true,"archived":false}"##
);

bench_parse!(
    pipeline_summary,
    PipelineSummary,
    r#"{"id":47,"iid":12,"project_id":1,"status":"pending","source":"push","ref":"main",
    "sha":"a91957a8","name":"Build","web_url":"https://example.com/-/pipelines/47",
    "created_at":"2022-01-01T00:00:00Z","updated_at":"2022-01-01T00:00:00Z"}"#
);

bench_parse!(
    pipeline_detail,
    PipelineDetail,
    r#"{"id":287,"iid":144,"project_id":21,"name":"Build pipeline","sha":"50f0acb7","ref":"main",
    "status":"success","source":"push","created_at":"2022-09-21T01:05:07Z","updated_at":"2022-09-21T01:05:50Z",
    "web_url":"http://example/-/pipelines/287","before_sha":"8a24fb3c","tag":false,"duration":34,
    "queued_duration":6,"archived":false,"user":{"id":1,"username":"root","name":"Admin","state":"active"}}"#
);

bench_parse!(
    job,
    Job,
    r#"{"id":7,"name":"teaspoon","stage":"test","status":"failed","ref":"main",
    "failure_reason":"script_failure","duration":0.173,"tag_list":["docker runner","ubuntu18"],
    "artifacts":[{"file_type":"archive","size":1000,"filename":"artifacts.zip","file_format":"zip"}],
    "pipeline":{"id":6,"project_id":1,"ref":"main","sha":"0ff3ae19","status":"pending"},
    "commit":{"id":"0ff3ae19","short_id":"0ff3","title":"Fix","message":"Fix the thing"},
    "web_url":"https://example.com/foo/bar/-/jobs/7"}"#
);

// ---------------------------------------------------------------------------
// Other direct-return types (parsed straight off an endpoint, not just as a
// nested field).
// ---------------------------------------------------------------------------

bench_parse!(
    milestone,
    Milestone,
    r#"{"id":12,"iid":3,"project_id":7,"title":"v1.0","description":"First release",
    "state":"active","due_date":"2022-06-01","created_at":"2022-01-01T00:00:00Z",
    "updated_at":"2022-02-01T00:00:00Z"}"#
);

bench_parse!(
    contributor,
    Contributor,
    r#"{"name":"Example User","email":"example@example.com","commits":117,
    "additions":0,"deletions":0}"#
);

bench_parse!(
    pipeline_variable,
    PipelineVariable,
    r#"{"key":"RUN_NIGHTLY_BUILD","variable_type":"env_var","value":"true"}"#
);

bench_parse!(
    wiki_page_list,
    WikiPageList,
    r#"{"slug":"home","title":"Home"}"#
);

bench_parse!(
    discussion,
    Discussion,
    r#"{"id":"abc123","individual_note":false,"notes":[
    {"id":1,"body":"A comment","author":{"id":2,"username":"alice","name":"Alice"},
    "created_at":"2022-01-01T00:00:00Z","updated_at":"2022-01-01T00:00:00Z","system":false,
    "noteable_type":"MergeRequest","resolvable":true,"resolved":false}]}"#
);

bench_parse!(
    merge_request_changes,
    MergeRequestChanges,
    r#"{"id":99,"iid":12,"changes_count":"1","overflow":false,"changes":[
    {"old_path":"a.rs","new_path":"a.rs","a_mode":"100644","b_mode":"100644",
    "new_file":false,"renamed_file":false,"deleted_file":false,
    "diff":"@@ -1,3 +1,4 @@\n line\n+added\n line\n"}]}"#
);

bench_parse!(
    commit_with_diffs,
    CommitWithDiffs,
    r#"{"id":"0ff3ae19","short_id":"0ff3","title":"Fix","message":"Fix the thing",
    "author_name":"Alice","author_email":"alice@example.com","authored_date":"2022-01-01T00:00:00Z",
    "committer_name":"Alice","committer_email":"alice@example.com","committed_date":"2022-01-01T00:00:00Z",
    "parent_ids":["a1b2c3"],"web_url":"https://example.com/-/commit/0ff3ae19",
    "stats":{"additions":10,"deletions":2,"total":12}}"#
);

bench_parse!(
    release,
    Release,
    r###"{"tag_name":"v0.2","name":"Awesome app v0.2","description":"## CHANGELOG\r\n- stuff",
    "created_at":"2019-01-03T01:56:19Z","released_at":"2019-01-03T01:56:19Z",
    "author":{"id":1,"name":"Admin","username":"root","state":"active"},
    "commit":{"id":"079e9010","short_id":"079e9010","title":"Update README"},
    "milestones":[{"id":51,"iid":1,"project_id":24,"title":"v1.0-rc","state":"closed",
    "issue_stats":{"total":98,"closed":76}}],
    "assets":{"count":6,"sources":[{"format":"zip","url":"https://example.com/x.zip"}],
    "links":[{"id":2,"name":"asset.msi","url":"https://example.com/msi","link_type":"other"}]}}"###
);

bench_parse!(
    snippet,
    Snippet,
    r#"{"id":1,"title":"test","file_name":"add.rb","description":"Ruby test snippet",
    "visibility":"private","author":{"id":1,"username":"john_smith","email":"john@example.com",
    "name":"John Smith","state":"active"},"expires_at":null,"updated_at":"2012-06-28T10:52:04Z",
    "created_at":"2012-06-28T10:52:04Z","project_id":null,"web_url":"http://example.com/snippets/1",
    "raw_url":"http://example.com/snippets/1/raw"}"#
);

bench_parse!(
    environment,
    Environment,
    r#"{"id":1,"name":"review/fix-foo","slug":"review-fix-foo-dfjre3",
    "description":"review env","external_url":"https://x.example.com","state":"available",
    "tier":"development","created_at":"2019-05-25T18:55:13Z","updated_at":"2019-05-27T18:55:13Z",
    "auto_stop_setting":"always"}"#
);

bench_parse!(
    deployment,
    Deployment,
    r#"{"id":42,"iid":2,"ref":"main","sha":"a91957a8","status":"success",
    "created_at":"2016-08-11T11:32:35Z","updated_at":"2016-08-11T11:34:01Z",
    "environment":{"id":9,"name":"production","external_url":"https://x"},
    "user":{"id":1,"name":"Admin","username":"root","state":"active"},
    "deployable":{"id":664,"status":"success","stage":"deploy","name":"deploy","ref":"main",
    "tag":false,"commit":{"id":"a91957a8","short_id":"a91957a8","title":"Merge"},
    "pipeline":{"id":42,"ref":"main","sha":"a91957a8","status":"success"}}}"#
);

bench_parse!(
    runner,
    Runner,
    r#"{"active":true,"paused":false,"description":"test-1-20150125","id":6,"ip_address":"",
    "is_shared":false,"runner_type":"project_type","name":null,"online":true,"status":"online",
    "job_execution_status":"idle"}"#
);

bench_parse!(
    runner_detail,
    RunnerDetail,
    r#"{"id":6,"description":"test-1","active":true,"paused":false,"is_shared":false,
    "runner_type":"project_type","status":"online","job_execution_status":"idle",
    "contacted_at":"2016-01-25T16:39:48Z","access_level":"ref_protected","maximum_timeout":3600,
    "tag_list":["ruby","mysql"],
    "projects":[{"id":1,"name":"CE","path":"gitlab-foss","path_with_namespace":"gitlab-org/gitlab-foss"}]}"#
);

bench_parse!(
    note,
    GitlabNote,
    r#"{"id":302,"body":"Text of the comment",
    "author":{"id":1,"username":"pipin","email":"admin@example.com","name":"Pip","state":"active"},
    "created_at":"2013-10-02T09:22:45Z","updated_at":"2013-10-02T10:22:45Z","system":false,
    "noteable_id":377,"noteable_type":"Issue","project_id":5,"noteable_iid":377,
    "resolvable":false,"confidential":false,"internal":false}"#
);

bench_parse!(
    deploy_key,
    DeployKey,
    r#"{"id":1,"title":"Public key","key":"ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAAAgQDNJAkI",
    "fingerprint":"4a:9d:64:15:ed:3a:e6:07","fingerprint_sha256":"SHA256:Jrs3LD1Ji30xNLtTVf9N",
    "created_at":"2013-10-02T10:12:29Z","expires_at":null,"can_push":false}"#
);

bench_parse!(
    deploy_token,
    DeployToken,
    r#"{"id":1,"name":"MyToken","username":"gitlab+deploy-token-1",
    "expires_at":"2020-02-14T00:00:00.000Z","revoked":false,"expired":false,
    "scopes":["read_repository","read_registry"]}"#
);

// ---------------------------------------------------------------------------
// Remaining endpoint categories: metadata, governance, planning, and the
// lighter reaction/audit shapes. One representative blob apiece.
// ---------------------------------------------------------------------------

bench_parse!(
    metadata,
    Metadata,
    r#"{"version":"17.0.0","revision":"abcdef123","enterprise":true,
    "kas":{"enabled":true,"version":"17.0.0","external_url":"wss://kas.example.com"}}"#
);

bench_parse!(
    namespace_listing,
    NamespaceListing,
    r#"{"id":2,"name":"group1","path":"group1","kind":"group","full_path":"group1",
    "parent_id":null,"avatar_url":null,"web_url":"https://example.com/groups/group1",
    "billable_members_count":2,"plan":"default","trial":false}"#
);

bench_parse!(
    protected_branch,
    ProtectedBranch,
    r#"{"id":1,"name":"main",
    "push_access_levels":[{"access_level":40,"access_level_description":"Maintainers"}],
    "merge_access_levels":[{"access_level":40,"access_level_description":"Maintainers"}],
    "allow_force_push":false,"code_owner_approval_required":false}"#
);

bench_parse!(
    commit_status,
    CommitStatus,
    r#"{"id":91,"sha":"18f3e63d","ref":"main","status":"success","name":"bundler:audit",
    "target_url":"https://example.com/jobs/91","description":null,
    "created_at":"2022-01-01T00:00:00Z","started_at":"2022-01-01T00:00:01Z",
    "finished_at":"2022-01-01T00:00:10Z","allow_failure":true,"coverage":98.29,"pipeline_id":12}"#
);

bench_parse!(
    pipeline_schedule,
    PipelineSchedule,
    r#"{"id":13,"description":"Nightly build","ref":"refs/heads/main","cron":"0 1 * * *",
    "cron_timezone":"Asia/Tokyo","next_run_at":"2022-01-02T01:00:00Z","active":true,
    "created_at":"2021-01-01T00:00:00Z","updated_at":"2021-06-01T00:00:00Z",
    "owner":{"id":1,"username":"root","name":"Admin","state":"active"}}"#
);

bench_parse!(
    award_emoji,
    AwardEmoji,
    r#"{"id":4,"name":"rocket","awardable_type":"Issue","awardable_id":80,
    "user":{"id":2,"username":"alice","name":"Alice","state":"active"},
    "created_at":"2016-06-15T10:09:34.206Z","updated_at":"2016-06-15T10:09:34.206Z"}"#
);

bench_parse!(
    todo,
    Todo,
    r#"{"id":102,"action_name":"review_requested","target_type":"MergeRequest",
    "target_url":"https://example.com/g/p/-/merge_requests/7","body":"Please review",
    "state":"pending","created_at":"2021-01-02T09:00:00Z",
    "project":{"id":3,"name":"widget","path_with_namespace":"group/widget"},
    "author":{"id":2,"username":"alice","name":"Alice","state":"active"}}"#
);

bench_parse!(
    search_blob,
    SearchBlob,
    r#"{"basename":"main","data":"fn main() {}\n","path":"src/main.rs",
    "filename":"src/main.rs","id":null,"ref":"main","startline":1,"project_id":6}"#
);

bench_parse!(
    issue_statistics,
    IssueStatistics,
    r#"{"statistics":{"counts":{"all":30,"closed":22,"opened":8}}}"#
);

bench_parse!(
    resource_label_event,
    ResourceLabelEvent,
    r##"{"id":142,"created_at":"2021-01-02T09:00:00Z","resource_type":"Issue","resource_id":11,
    "user":{"id":2,"username":"alice","name":"Alice","state":"active"},
    "label":{"id":5,"name":"Doing","color":"#cc0033"},"action":"add"}"##
);

bench_parse!(
    resource_state_event,
    ResourceStateEvent,
    r#"{"id":200,"created_at":"2021-01-05T12:00:00Z","resource_type":"Issue","resource_id":11,
    "state":"closed"}"#
);

bench_parse!(
    resource_milestone_event,
    ResourceMilestoneEvent,
    r#"{"id":201,"created_at":"2021-01-05T12:00:00Z","resource_type":"Issue","resource_id":11,
    "action":"add","milestone":{"id":12,"iid":3,"project_id":7,"title":"v1.0","state":"active"}}"#
);

bench_parse!(
    issue_link,
    IssueLink,
    r#"{"id":84,"iid":14,"project_id":4,"title":"Login flow","state":"opened",
    "issue_link_id":2,"link_type":"is_blocked_by"}"#
);

bench_parse!(
    issue_link_result,
    IssueLinkResult,
    r#"{"source_issue":{"id":1,"iid":7,"title":"A","state":"opened"},
    "target_issue":{"id":2,"iid":8,"title":"B","state":"opened"},"link_type":"relates_to"}"#
);

bench_parse!(
    epic_issue,
    EpicIssue,
    r#"{"id":1,"iid":7,"project_id":3,"title":"Something","state":"opened",
    "epic_issue_id":9,"relative_position":1000}"#
);

bench_parse!(
    protected_tag,
    ProtectedTag,
    r#"{"name":"v1.0.0","create_access_levels":[{"access_level":40,
    "access_level_description":"Maintainers"}]}"#
);

bench_parse!(
    protected_environment,
    ProtectedEnvironment,
    r#"{"name":"production","deploy_access_levels":[{"access_level":40,
    "access_level_description":"Maintainers"}],"required_approval_count":1}"#
);

bench_parse!(
    runner_manager,
    RunnerManager,
    r#"{"id":1,"system_id":"s_abc","version":"16.0.0","revision":"abcdef",
    "platform":"linux","architecture":"amd64","created_at":"2022-01-01T00:00:00Z",
    "contacted_at":"2022-01-02T00:00:00Z","ip_address":"1.2.3.4","status":"online",
    "job_execution_status":"idle"}"#
);

bench_parse!(
    runner_auth_token,
    RunnerAuthToken,
    r#"{"token":"glrt-abcdef1234567890","token_expires_at":null}"#
);

bench_parse!(
    created_deploy_token,
    CreatedDeployToken,
    r#"{"id":1,"name":"MyToken","username":"gitlab+deploy-token-1",
    "expires_at":"2020-02-14T00:00:00.000Z","revoked":false,"expired":false,
    "scopes":["read_repository"],"token":"gldt-abcdef1234567890"}"#
);

bench_parse!(
    changelog,
    Changelog,
    "{\"notes\":\"## v1.0.0\\n\\n- Initial release\"}"
);

bench_parse!(
    ref_commit,
    RefCommit,
    r#"{"id":"7b5c3cc8","short_id":"7b5c3cc","title":"Merge branch",
    "message":"Merge branch 'x'","author_name":"Alice","author_email":"alice@example.com",
    "authored_date":"2022-01-01T00:00:00Z","committer_name":"Alice",
    "committer_email":"alice@example.com","committed_date":"2022-01-01T00:00:00Z",
    "parent_ids":["a1b2c3","d4e5f6"]}"#
);

bench_parse!(
    blame_range,
    BlameRange,
    r#"{"commit":{"id":"7b5c3cc8","message":"Fix","parent_ids":["a1b2c3"],
    "authored_date":"2022-01-01T00:00:00Z","author_name":"Alice",
    "author_email":"alice@example.com","committed_date":"2022-01-01T00:00:00Z",
    "committer_name":"Alice","committer_email":"alice@example.com"},
    "lines":["fn main() {","    println!(\"hi\");","}"]}"#
);

bench_parse!(
    file_mutation_result,
    FileMutationResult,
    r#"{"file_path":"app/models/key.rb","branch":"main"}"#
);

bench_parse!(
    merge_request_approvals,
    MergeRequestApprovals,
    r#"{"approvals_required":2,"approvals_left":1,"approved_by":[
    {"user":{"id":1,"username":"alice","name":"Alice","avatar_url":null}}]}"#
);

bench_parse!(
    snippet_user_agent_detail,
    SnippetUserAgentDetail,
    r#"{"user_agent":"Mozilla/5.0","ip_address":"1.2.3.4","akismet_submitted":false}"#
);

bench_parse!(
    iteration_cadence,
    IterationCadence,
    r#"{"id":3,"title":"Team A cadence","description":"Bi-weekly","active":true,
    "automatic":true,"start_date":"2022-01-01","duration_in_weeks":2,
    "iterations_in_advance":2,"roll_over":false,"created_at":"2021-12-01T00:00:00Z",
    "updated_at":"2021-12-01T00:00:00Z"}"#
);

bench_parse!(
    board_list,
    BoardList,
    r##"{"id":10,"position":1,"list_type":"label",
    "label":{"id":5,"name":"Doing","color":"#cc0033"},"collapsed":false}"##
);

bench_parse!(
    epic,
    Epic,
    r#"{"id":30,"iid":5,"group_id":7,"parent_id":null,"title":"Q3 Roadmap",
    "description":"Portfolio epic","state":"opened","labels":["roadmap"],
    "start_date":"2021-07-01","due_date":"2021-09-30","confidential":false,
    "upvotes":3,"downvotes":0,"web_url":"https://example.com/groups/g/-/epics/5"}"#
);

bench_parse!(
    board,
    Board,
    r##"{"id":1,"name":"Development","hide_backlog_list":false,"hide_closed_list":false,
    "lists":[{"id":10,"position":1,"list_type":"label",
    "label":{"id":5,"name":"Doing","color":"#cc0033"},"collapsed":false}]}"##
);

bench_parse!(
    iteration,
    Iteration,
    r#"{"id":53,"iid":13,"group_id":5,"title":"Sprint 42","description":"Sprint","state":2,
    "created_at":"2021-12-01T00:00:00Z","updated_at":"2021-12-01T00:00:00Z",
    "start_date":"2021-12-06","due_date":"2021-12-17","iteration_cadence_id":3,
    "web_url":"https://example.com/groups/g/-/iterations/13","sequence":42}"#
);

// ---------------------------------------------------------------------------
// Paginated list parses — `Vec<T>` of 20 elements, the real hot path for any
// endpoint that paginates. This is where parse cost actually accrues.
// ---------------------------------------------------------------------------

const LIST_LEN: usize = 20;

bench_parse_list!(
    list_project,
    GitlabProject,
    LIST_LEN,
    r#"{"id":42,"description":"A project","visibility":"private","name":"widget",
    "name_with_namespace":"Group / widget","path":"widget","path_with_namespace":"group/widget",
    "open_issues_count":3,"created_at":"2021-01-01T00:00:00Z","last_activity_at":"2022-01-01T00:00:00Z",
    "creator_id":1,"namespace":{"id":5,"name":"Group","path":"group","kind":"group","full_path":"group"},
    "archived":false}"#
);

bench_parse_list!(
    list_issue,
    GitlabIssue,
    LIST_LEN,
    r#"{"id":1,"iid":7,"project_id":3,"title":"Something broke",
    "description":"A description with detail.","state":"opened","type":"ISSUE",
    "labels":["bug","backend"],"author":{"id":2,"username":"alice","name":"Alice","state":"active"},
    "upvotes":5,"downvotes":0,"user_notes_count":12,
    "web_url":"https://gitlab.example.com/g/p/-/issues/7",
    "created_at":"2022-01-01T00:00:00Z","updated_at":"2022-02-01T00:00:00Z"}"#
);

bench_parse_list!(
    list_commit,
    GitlabCommit,
    LIST_LEN,
    r#"{"id":"0ff3ae19","short_id":"0ff3ae19","title":"Fix","message":"Fix the thing",
    "author_name":"Alice","author_email":"alice@example.com","authored_date":"2022-01-01T00:00:00Z",
    "committer_name":"Alice","committer_email":"alice@example.com","committed_date":"2022-01-01T00:00:00Z",
    "created_at":"2022-01-01T00:00:00Z","parent_ids":["a1b2c3"],"web_url":"https://example.com/-/commit/0ff3ae19"}"#
);

bench_parse_list!(
    list_job,
    Job,
    LIST_LEN,
    r#"{"id":7,"name":"teaspoon","stage":"test","status":"failed","ref":"main",
    "failure_reason":"script_failure","duration":0.173,"tag_list":["docker runner"],
    "pipeline":{"id":6,"project_id":1,"ref":"main","sha":"0ff3ae19","status":"pending"},
    "web_url":"https://example.com/foo/bar/-/jobs/7"}"#
);

bench_parse_list!(
    list_member,
    Member,
    LIST_LEN,
    r#"{"id":1,"username":"raymond_smith","name":"Raymond Smith","state":"active",
    "avatar_url":"https://example.com/a.png","web_url":"https://example.com/raymond",
    "access_level":30,"expires_at":"2025-10-22","created_at":"2012-09-22T14:13:35Z"}"#
);

bench_parse_list!(
    list_event,
    GitlabEvent,
    LIST_LEN,
    r#"{"id":101,"action_name":"pushed to","target_type":"Note",
    "created_at":"2022-01-01T00:00:00Z","author_username":"alice",
    "author":{"id":2,"username":"alice","name":"Alice"}}"#
);
