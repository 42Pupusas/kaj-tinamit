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
