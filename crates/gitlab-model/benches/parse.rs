//! Parse-throughput benchmarks for every top-level wire model, via `divan`.
//!
//! One representative JSON blob per model, parsed in a black-boxed loop. Run
//! with `cargo bench -p gitlab-model`. These guard against parse-path
//! regressions in `json-bourne` and in our derive usage.

use gitlab_model::{
    Blob, Branch, CommitWithDiffs, Contributor, Deployment, Discussion, Environment, GitlabCommit,
    GitlabEvent, GitlabGroup, GitlabIssue, GitlabProject, GitlabUser, Job, Label, Member,
    MergeRequest, MergeRequestChanges, Milestone, PipelineDetail, PipelineSummary, PipelineVariable,
    Release, RepositoryFile, Snippet, Tag, TreeEntry, WikiPage, WikiPageList,
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
