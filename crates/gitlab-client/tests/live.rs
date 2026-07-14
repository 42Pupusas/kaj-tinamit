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

/// Exercises the Groups + Members categories: list groups, fetch one, its
/// members (direct + inherited), subgroups, and group projects.
#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn groups_and_members_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let groups = client.groups().expect("groups request failed");
    eprintln!("visible groups: {}", groups.len());
    for g in groups.iter().take(10) {
        eprintln!("  #{} {}", g.id, g.full_path.as_deref().unwrap_or(&g.path));
    }

    let Some(group) = groups.first() else {
        eprintln!("no groups visible - skipping detail checks");
        return;
    };
    let gid = group.id;

    let one = client.group(gid).expect("group request failed");
    assert_eq!(one.id, gid);

    let members = client.group_members(gid).expect("group_members failed");
    eprintln!("  direct members: {}", members.len());
    for m in members.iter().take(10) {
        eprintln!("    @{} [{:?}]", m.username, m.role());
    }

    let all = client
        .group_members_all(gid)
        .expect("group_members_all failed");
    eprintln!("  members incl. inherited: {}", all.len());

    let subs = client.subgroups(gid).expect("subgroups failed");
    eprintln!("  subgroups: {}", subs.len());

    let gprojects = client.group_projects(gid).expect("group_projects failed");
    eprintln!("  group projects: {}", gprojects.len());

    // If the group has a direct member, round-trip a single fetch.
    if let Some(first) = members.first() {
        let single = client
            .group_member(gid, first.id)
            .expect("group_member failed");
        assert_eq!(single.id, first.id);
    }
}

/// Exercises the Repository category (tree, branches, tags, file, blame,
/// contributors) against the first project that has a populated repository.
#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn repository_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let projects = client.projects().expect("projects request failed");

    // Find a project whose default branch actually has a tree.
    let mut chosen = None;
    for p in &projects {
        let pid = i64::from(p.id);
        if let Ok(tree) = client.repository_tree(pid, None, None, false)
            && !tree.is_empty()
        {
            chosen = Some((pid, p, tree));
            break;
        }
    }
    let (pid, project, tree) = chosen.expect("need one project with a repository");
    eprintln!(
        "repository of #{} {}",
        pid,
        project.path_with_namespace.as_deref().unwrap_or("?")
    );
    eprintln!("  tree entries (root): {}", tree.len());
    for e in tree.iter().take(5) {
        eprintln!("    [{:?}] {}", e.entry_type, e.path);
    }

    let branches = client.branches(pid).expect("branches request failed");
    eprintln!("  branches: {}", branches.len());
    let default_branch = branches
        .iter()
        .find(|b| b.default)
        .or_else(|| branches.first())
        .expect("repository has at least one branch");
    eprintln!("  default branch: {}", default_branch.name);

    // Round-trip a single branch fetch.
    let one = client
        .branch(pid, &default_branch.name)
        .expect("branch request failed");
    assert_eq!(one.name, default_branch.name);

    let tags = client.tags(pid).expect("tags request failed");
    eprintln!("  tags: {}", tags.len());

    let contributors = client.contributors(pid).expect("contributors request failed");
    eprintln!("  contributors: {}", contributors.len());

    // Fetch the first blob-type entry as a file (metadata + raw).
    if let Some(blob) = tree.iter().find(|e| {
        matches!(e.entry_type, gitlab_client::model::TreeEntryType::Blob)
    }) {
        let file = client
            .file(pid, &blob.path, &default_branch.name)
            .expect("file request failed");
        eprintln!("  file {}: {} bytes", file.file_path, file.size);
        let raw = client
            .file_raw(pid, &blob.path, &default_branch.name)
            .expect("file_raw request failed");
        eprintln!("  file_raw {}: {} chars", blob.path, raw.len());

        let blame = client
            .file_blame(pid, &blob.path, &default_branch.name)
            .expect("file_blame request failed");
        eprintln!("  blame ranges: {}", blame.len());
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
