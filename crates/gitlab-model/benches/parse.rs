//! Parse-throughput benchmarks for the hot wire models, via `divan`.
//!
//! Run with: `cargo bench -p gitlab-model`.

use gitlab_model::{GitlabIssue, GitlabProject, Job, Member, PipelineDetail};
use json_bourne::parse_str;

fn main() {
    divan::main();
}

const ISSUE_JSON: &str = r#"{
    "id": 1, "iid": 7, "project_id": 3, "title": "Something broke",
    "description": "A fairly long description of the problem that spans\nmultiple lines and has some detail.",
    "state": "opened", "type": "ISSUE",
    "labels": ["bug", "backend", "priority::high"],
    "author": {"id": 2, "username": "alice", "name": "Alice", "state": "active"},
    "assignees": [{"id": 3, "username": "bob", "name": "Bob", "state": "active"}],
    "upvotes": 5, "downvotes": 0, "user_notes_count": 12,
    "web_url": "https://gitlab.example.com/g/p/-/issues/7",
    "created_at": "2022-01-01T00:00:00Z", "updated_at": "2022-02-01T00:00:00Z"
}"#;

const PROJECT_JSON: &str = r#"{
    "id": 42, "description": "A project", "visibility": "private",
    "name": "widget", "name_with_namespace": "Group / widget",
    "path": "widget", "path_with_namespace": "group/widget",
    "open_issues_count": 3, "created_at": "2021-01-01T00:00:00Z",
    "last_activity_at": "2022-01-01T00:00:00Z", "creator_id": 1,
    "namespace": {"id": 5, "name": "Group", "path": "group", "kind": "group", "full_path": "group"},
    "archived": false
}"#;

const MEMBER_JSON: &str = r#"{
    "id": 1, "username": "raymond_smith", "name": "Raymond Smith",
    "state": "active", "avatar_url": "https://example.com/a.png",
    "web_url": "https://example.com/raymond", "access_level": 30,
    "expires_at": "2025-10-22", "created_at": "2012-09-22T14:13:35Z"
}"#;

const JOB_JSON: &str = r#"{
    "id": 7, "name": "teaspoon", "stage": "test", "status": "failed",
    "ref": "main", "failure_reason": "script_failure", "duration": 0.173,
    "tag_list": ["docker runner", "ubuntu18"],
    "artifacts": [{"file_type": "archive", "size": 1000, "filename": "artifacts.zip", "file_format": "zip"}],
    "pipeline": {"id": 6, "project_id": 1, "ref": "main", "sha": "0ff3ae19", "status": "pending"},
    "commit": {"id": "0ff3ae19", "short_id": "0ff3", "title": "Fix", "message": "Fix the thing"},
    "web_url": "https://example.com/foo/bar/-/jobs/7"
}"#;

const PIPELINE_JSON: &str = r#"{
    "id": 287, "iid": 144, "project_id": 21, "name": "Build pipeline",
    "sha": "50f0acb7", "ref": "main", "status": "success", "source": "push",
    "created_at": "2022-09-21T01:05:07.200Z", "updated_at": "2022-09-21T01:05:50.185Z",
    "web_url": "http://example/-/pipelines/287", "before_sha": "8a24fb3c",
    "tag": false, "duration": 34, "queued_duration": 6, "archived": false,
    "user": {"id": 1, "username": "root", "name": "Administrator", "state": "active"}
}"#;

#[divan::bench]
fn issue(bencher: divan::Bencher) {
    bencher.bench(|| {
        let v: GitlabIssue = parse_str(divan::black_box(ISSUE_JSON)).unwrap();
        divan::black_box(v);
    });
}

#[divan::bench]
fn project(bencher: divan::Bencher) {
    bencher.bench(|| {
        let v: GitlabProject = parse_str(divan::black_box(PROJECT_JSON)).unwrap();
        divan::black_box(v);
    });
}

#[divan::bench]
fn member(bencher: divan::Bencher) {
    bencher.bench(|| {
        let v: Member = parse_str(divan::black_box(MEMBER_JSON)).unwrap();
        divan::black_box(v);
    });
}

#[divan::bench]
fn job(bencher: divan::Bencher) {
    bencher.bench(|| {
        let v: Job = parse_str(divan::black_box(JOB_JSON)).unwrap();
        divan::black_box(v);
    });
}

#[divan::bench]
fn pipeline(bencher: divan::Bencher) {
    bencher.bench(|| {
        let v: PipelineDetail = parse_str(divan::black_box(PIPELINE_JSON)).unwrap();
        divan::black_box(v);
    });
}
