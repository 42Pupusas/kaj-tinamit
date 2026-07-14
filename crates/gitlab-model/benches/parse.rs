//! Parse-throughput benchmarks for every top-level wire model, via `divan`.
//!
//! One representative JSON blob per model, parsed in a black-boxed loop. Run
//! with `cargo bench -p gitlab-model`. These guard against parse-path
//! regressions in `json-bourne` and in our derive usage.

use gitlab_model::{
    Blob, Branch, GitlabCommit, GitlabEvent, GitlabGroup, GitlabIssue, GitlabProject, GitlabUser,
    Job, Label, Member, MergeRequest, PipelineDetail, PipelineSummary, RepositoryFile, Tag,
    TreeEntry, WikiPage,
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
