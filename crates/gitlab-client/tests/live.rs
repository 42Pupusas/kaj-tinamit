//! Live integration tests against a real GitLab instance.
//!
//! Ignored by default — they need network + credentials. Run with:
//!
//! ```sh
//! GITLAB_URL=https://gitlab.illuminodes.com GITLAB_PAT=glpat-… \
//!     cargo test -p gitlab-client --test live -- --ignored --nocapture
//! ```

use gitlab_client::prelude::*;
use gitlab_client::{GitlabClient, IssueQuery, IssueScope, IssueStateFilter};

#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn current_user_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let me = client.current_user().expect("current_user request failed");
    eprintln!("authenticated as #{} @{} ({})", me.id, me.username, me.name);
    assert!(me.id > 0);
    assert!(!me.username.is_empty());
}

#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn projects_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let projects = client.projects().expect("projects request failed");
    eprintln!("visible projects: {}", projects.len());
    for p in projects.iter().take(10) {
        eprintln!(
            "  #{} {}",
            p.id,
            p.path_with_namespace.as_deref().unwrap_or("<no path>")
        );
    }
}

#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn events_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let events = client.events(None).expect("events request failed");
    eprintln!("authenticated user events: {}", events.len());
    for e in events.iter().take(5) {
        eprintln!(
            "  {} {} {}",
            e.created_at.as_deref().unwrap_or("?"),
            e.action_name.as_deref().unwrap_or("?"),
            e.target_type.as_deref().unwrap_or("")
        );
    }
}

/// Walks the first membership project and exercises the project-scoped read
/// endpoints (commits, merge requests, milestones, project events) against
/// whatever real data is there.
#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn project_scoped_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let projects = client.projects().expect("projects request failed");
    let project = projects.first().expect("need at least one project");
    let pid = i64::from(project.id);
    eprintln!(
        "exercising project #{} {}",
        pid,
        project.path_with_namespace.as_deref().unwrap_or("?")
    );

    let commits = client.commits(pid, None).expect("commits request failed");
    eprintln!("  commits: {}", commits.len());

    let mrs = client
        .merge_requests(pid, &gitlab_client::MergeRequestQuery::new())
        .expect("merge_requests request failed");
    eprintln!("  merge requests: {}", mrs.len());

    let milestones = client
        .project_milestones(project.id)
        .expect("project_milestones request failed");
    eprintln!("  milestones: {}", milestones.len());

    let pevents = client
        .project_events(pid, None)
        .expect("project_events request failed");
    eprintln!("  project events: {}", pevents.len());
}

#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn issues_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let query = IssueQuery::new()
        .with_scope(IssueScope::All)
        .with_state(IssueStateFilter::All);
    let issues = client.issues(&query).expect("issues request failed");
    eprintln!("issues visible (all scope/state): {}", issues.len());
    for i in issues.iter().take(10) {
        eprintln!(
            "  !{} [{:?}] {}",
            i.iid,
            i.state,
            i.title.as_deref().unwrap_or("<no title>")
        );
    }
}
