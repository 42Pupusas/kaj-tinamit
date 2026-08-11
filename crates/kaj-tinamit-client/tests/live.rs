//! Live integration tests against a real GitLab instance.
//!
//! Ignored by default — they need network + credentials. Run with:
//!
//! ```sh
//! GITLAB_URL=https://gitlab.illuminodes.com GITLAB_PAT=glpat-… \
//!     cargo test -p kaj-tinamit-client --test live -- --ignored --nocapture
//! ```

use kaj_tinamit_client::prelude::*;
use kaj_tinamit_client::{
    CommitAction, CreateCommit, CreateEnvironment, CreateIssue, CreateLabel, CreateVariable,
    GitlabClient, IssueQuery, IssueScope, IssueStateFilter, UpdateEnvironment, UpdateIssue,
    UpdateVariable, VariableFilter,
};

/// The project the write tests operate on. Override with `GITLAB_TEST_PROJECT`
/// (a numeric ID); defaults to the first membership project.
fn write_test_project(client: &GitlabClient) -> i64 {
    if let Ok(id) = std::env::var("GITLAB_TEST_PROJECT")
        && let Ok(id) = id.parse::<i64>()
    {
        return id;
    }
    let projects = client.projects().expect("projects request failed");
    let project = projects.first().expect("need at least one project");
    i64::from(project.id)
}

#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn current_user_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let me = client.current_user().expect("current_user request failed");
    eprintln!("authenticated as #{} @{} ({})", me.id, me.username, me.name);
    assert!(me.id.get() > 0);
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
        .merge_requests(pid, &kaj_tinamit_client::MergeRequestQuery::new())
        .expect("merge_requests request failed");
    eprintln!("  merge requests: {}", mrs.len());

    let milestones = client
        .project_milestones(i32::try_from(i64::from(project.id)).expect("project id fits i32"))
        .expect("project_milestones request failed");
    eprintln!("  milestones: {}", milestones.len());

    let pevents = client
        .project_events(pid, None)
        .expect("project_events request failed");
    eprintln!("  project events: {}", pevents.len());
}

/// Exercises the Deploy keys + Deploy tokens categories via the first
/// project the account can reach. Both are often empty; endpoints must at
/// least parse a valid (empty) response.
#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn deploy_keys_and_tokens_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let projects = client.projects().expect("projects request failed");

    let mut key_hit = false;
    for p in &projects {
        let pid = i64::from(p.id);
        if let Ok(keys) = client.project_deploy_keys(pid)
            && let Some(first) = keys.first()
        {
            eprintln!("project #{pid} deploy keys: {}", keys.len());
            let one = client
                .project_deploy_key(pid, first.id.into())
                .expect("project_deploy_key failed");
            assert_eq!(one.id, first.id);
            key_hit = true;
            break;
        }
    }
    if !key_hit {
        eprintln!("no project deploy keys found");
    }

    // Deploy tokens: exercise the endpoint on the first project (needs
    // Maintainer+; may 403, in which case we just report it).
    if let Some(project) = projects.first() {
        match client.project_deploy_tokens(i64::from(project.id)) {
            Ok(tokens) => eprintln!("project #{} deploy tokens: {}", project.id, tokens.len()),
            Err(e) => eprintln!("project deploy tokens not accessible: {e}"),
        }
    }
}

/// Exercises the Notes category: find an issue with notes and round-trip
/// the list + a single note.
#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn notes_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let query = IssueQuery::new()
        .with_scope(IssueScope::All)
        .with_state(IssueStateFilter::All);
    let issues = client.issues(&query).expect("issues request failed");

    let mut found = false;
    for issue in issues.iter().take(50) {
        let Some(pid) = issue.project_id else {
            continue;
        };
        let notes = client
            .issue_notes(i64::from(pid), i64::from(issue.iid))
            .expect("issue_notes failed");
        if let Some(first) = notes.first() {
            eprintln!(
                "issue {}!{} has {} notes; first #{} system={} type={:?}",
                pid,
                issue.iid,
                notes.len(),
                first.id,
                first.system,
                first.noteable_type
            );
            let one = client
                .issue_note(i64::from(pid), i64::from(issue.iid), first.id.into())
                .expect("issue_note failed");
            assert_eq!(one.id, first.id);
            found = true;
            break;
        }
    }
    if !found {
        eprintln!("no issue notes found in first 50 issues");
    }
}

/// Exercises the Runners category: list accessible runners, then round-trip
/// runner detail + managers for the first one; also project runners.
#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn runners_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let runners = client
        .runners(&kaj_tinamit_client::RunnerQuery::new())
        .expect("runners request failed");
    eprintln!("accessible runners: {}", runners.len());
    for r in runners.iter().take(5) {
        eprintln!(
            "  #{} [{}] {}",
            r.id,
            r.status.as_deref().unwrap_or("?"),
            r.description.as_deref().unwrap_or("")
        );
    }

    if let Some(first) = runners.first() {
        let detail = client
            .runner(first.id.into())
            .expect("runner detail failed");
        assert_eq!(detail.id, first.id);
        eprintln!(
            "  runner {} type {:?}, {} projects, tags {:?}",
            detail.id,
            detail.runner_type,
            detail.projects.len(),
            detail.tag_list
        );
        let managers = client
            .runner_managers(first.id.into())
            .expect("runner_managers failed");
        eprintln!("  managers: {}", managers.len());
    }

    // Project-scoped runners for the first project.
    if let Some(project) = client.projects().expect("projects failed").first() {
        let pr = client
            .project_runners(i64::from(project.id))
            .expect("project_runners failed");
        eprintln!("project #{} runners: {}", project.id, pr.len());
    }
}

/// Exercises the Environments + Deployments categories: walk projects to
/// find one with environments/deployments, round-trip single fetches.
#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn environments_and_deployments_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let projects = client.projects().expect("projects request failed");

    let mut env_hit = false;
    let mut dep_hit = false;
    for p in &projects {
        let pid = i64::from(p.id);

        if !env_hit
            && let Ok(envs) = client.environments(pid)
            && let Some(first) = envs.first()
        {
            eprintln!(
                "environments in #{}: {} (first: {})",
                pid,
                envs.len(),
                first.name
            );
            let one = client
                .environment(pid, first.id.into())
                .expect("environment failed");
            assert_eq!(one.id, first.id);
            env_hit = true;
        }

        if !dep_hit
            && let Ok(deps) = client.deployments(pid)
            && let Some(first) = deps.first()
        {
            eprintln!(
                "deployments in #{}: {} (first id {})",
                pid,
                deps.len(),
                first.id
            );
            let one = client
                .deployment(pid, first.id.into())
                .expect("deployment failed");
            assert_eq!(one.id, first.id);
            dep_hit = true;
        }

        if env_hit && dep_hit {
            break;
        }
    }
    if !env_hit {
        eprintln!("no environments found in any project");
    }
    if !dep_hit {
        eprintln!("no deployments found in any project");
    }
}

/// Exercises the Snippets category: personal snippets (list + single + raw)
/// and public snippets. Personal snippets may be empty on the test account,
/// so the detail checks are conditional; public snippets exercise parsing.
#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn snippets_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");

    let mine = client.snippets().expect("snippets request failed");
    eprintln!("personal snippets: {}", mine.len());
    for s in mine.iter().take(5) {
        eprintln!("  #{} {}", s.id, s.title.as_deref().unwrap_or("<untitled>"));
    }
    if let Some(first) = mine.first() {
        let one = client
            .snippet(first.id.into())
            .expect("snippet request failed");
        assert_eq!(one.id, first.id);
        let raw = client
            .snippet_raw(first.id.into())
            .expect("snippet_raw failed");
        eprintln!("  snippet {} raw: {} chars", first.id, raw.len());
    }

    let public = client.public_snippets().expect("public_snippets failed");
    eprintln!("public snippets: {}", public.len());
}

/// Exercises the Releases category: walk projects to find one with releases,
/// then round-trip a single release by tag and the latest-release permalink.
#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn releases_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let projects = client.projects().expect("projects request failed");

    let mut found = None;
    for p in &projects {
        let pid = i64::from(p.id);
        if let Ok(rels) = client.releases(pid)
            && let Some(first) = rels.first()
        {
            found = Some((pid, p, first.tag_name.clone()));
            break;
        }
    }
    let Some((pid, project, tag)) = found else {
        eprintln!("no releases found in any project - skipping release detail checks");
        return;
    };
    eprintln!(
        "releases in #{} {}: first tag {}",
        pid,
        project.path_with_namespace.as_deref().unwrap_or("?"),
        tag
    );

    let one = client.release(pid, &tag).expect("release request failed");
    assert_eq!(one.tag_name, tag);
    eprintln!(
        "  release {}: {} assets, {} milestones",
        one.tag_name,
        one.assets.count,
        one.milestones.len()
    );

    let latest = client.latest_release(pid).expect("latest_release failed");
    eprintln!("  latest release tag: {}", latest.tag_name);
}

/// Exercises the CI category: walks projects to find one with pipelines,
/// then fetches the pipeline detail, its jobs, variables, and a job trace.
#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn ci_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let projects = client.projects().expect("projects request failed");

    let mut found = None;
    for p in &projects {
        let pid = i64::from(p.id);
        if let Ok(pipes) = client.pipelines(pid, &kaj_tinamit_client::PipelineQuery::new())
            && let Some(first) = pipes.first()
        {
            found = Some((pid, p, i64::from(first.id)));
            break;
        }
    }
    let Some((pid, project, pipeline_id)) = found else {
        eprintln!("no pipelines found in any project - skipping CI detail checks");
        return;
    };
    eprintln!(
        "pipelines in #{} {}: first id {}",
        pid,
        project.path_with_namespace.as_deref().unwrap_or("?"),
        pipeline_id
    );

    let detail = client.pipeline(pid, pipeline_id).expect("pipeline failed");
    eprintln!("  pipeline {} status {:?}", detail.id, detail.status);

    let jobs = client
        .pipeline_jobs(pid, pipeline_id)
        .expect("pipeline_jobs failed");
    eprintln!("  jobs: {}", jobs.len());
    for j in jobs.iter().take(5) {
        eprintln!(
            "    {} [{:?}] stage={}",
            j.name.as_deref().unwrap_or("?"),
            j.status,
            j.stage.as_deref().unwrap_or("?")
        );
    }

    let vars = client
        .pipeline_variables(pid, pipeline_id)
        .expect("pipeline_variables failed");
    eprintln!("  pipeline variables: {}", vars.len());

    // Job trace for the first job (may be empty/expired but must not error).
    if let Some(job) = jobs.first() {
        let one = client.job(pid, job.id.into()).expect("job failed");
        assert_eq!(one.id, job.id);
        let trace = client
            .job_trace(pid, job.id.into())
            .expect("job_trace failed");
        eprintln!("  job {} trace: {} chars", job.id, trace.len());
    }
}

/// Exercises the Labels category: group labels (via the first group) and
/// project labels (via the first project), plus a single-label round-trip.
#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn labels_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");

    if let Some(group) = client.groups().expect("groups failed").first() {
        let labels = client
            .group_labels(group.id.into(), true)
            .expect("group_labels failed");
        eprintln!("group #{} labels: {}", group.id, labels.len());
        for l in labels.iter().take(5) {
            eprintln!(
                "  {} {} (open issues {})",
                l.color, l.name, l.open_issues_count
            );
        }
        if let Some(first) = labels.first() {
            let one = client
                .group_label(group.id.into(), &first.name)
                .expect("group_label failed");
            assert_eq!(one.name, first.name);
        }
    }

    let projects = client.projects().expect("projects failed");
    if let Some(project) = projects.first() {
        let pid = i64::from(project.id);
        let labels = client
            .project_labels(pid, true)
            .expect("project_labels failed");
        eprintln!("project #{pid} labels: {}", labels.len());
        for l in labels.iter().take(5) {
            eprintln!("  {} {}", l.color, l.name);
        }
    }
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
    let gid = i64::from(group.id);

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
            .group_member(gid, first.id.into())
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

    let contributors = client
        .contributors(pid)
        .expect("contributors request failed");
    eprintln!("  contributors: {}", contributors.len());

    // Fetch the first blob-type entry as a file (metadata + raw).
    if let Some(blob) = tree
        .iter()
        .find(|e| matches!(e.entry_type, kaj_tinamit_client::model::TreeEntryType::Blob))
    {
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

/// Exercises the Todos category: list the caller's pending to-dos.
#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn todos_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let todos = client.todos().expect("todos request failed");
    eprintln!("pending to-dos: {}", todos.len());
    for t in todos.iter().take(10) {
        eprintln!(
            "  #{} [{:?}] {} -> {}",
            t.id,
            t.action_name,
            t.target_type.as_deref().unwrap_or("?"),
            t.target_url.as_deref().unwrap_or("?")
        );
    }
    // The `done` filter must also parse a valid response.
    let done = client
        .todos_by_state("done")
        .expect("todos_by_state failed");
    eprintln!("done to-dos: {}", done.len());
}

/// Exercises the Search category at instance scope (projects, issues, MRs)
/// and project scope (issues, blobs) using a broad term.
#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn search_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");

    let projects = client.search_projects("a").expect("search_projects failed");
    eprintln!("global project search hits: {}", projects.len());

    let issues = client.search_issues("the").expect("search_issues failed");
    eprintln!("global issue search hits: {}", issues.len());

    let mrs = client
        .search_merge_requests("the")
        .expect("search_merge_requests failed");
    eprintln!("global MR search hits: {}", mrs.len());

    // Project-scoped code search on the first membership project.
    if let Some(project) = client.projects().expect("projects failed").first() {
        let pid = i64::from(project.id);
        let blobs = client
            .project_search_blobs(pid, "fn")
            .expect("project_search_blobs failed");
        eprintln!("project #{pid} blob search hits: {}", blobs.len());
        for b in blobs.iter().take(5) {
            eprintln!(
                "  {}:{} ",
                b.filename.as_deref().unwrap_or("?"),
                b.startline.unwrap_or(0)
            );
        }
        let pissues = client
            .project_search_issues(pid, "the")
            .expect("project_search_issues failed");
        eprintln!("project #{pid} issue search hits: {}", pissues.len());
    }
}

/// Exercises the Issues Statistics category at instance and project scope.
#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn issues_statistics_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let query = IssueQuery::new().with_scope(IssueScope::All);
    let stats = client
        .issues_statistics(&query)
        .expect("issues_statistics failed");
    let c = stats.counts();
    eprintln!(
        "instance issue stats: all={} opened={} closed={}",
        c.all, c.opened, c.closed
    );

    if let Some(project) = client.projects().expect("projects failed").first() {
        let pstats = client
            .project_issues_statistics(
                i32::try_from(i64::from(project.id)).expect("project id fits i32"),
                &IssueQuery::new(),
            )
            .expect("project_issues_statistics failed");
        let pc = pstats.counts();
        eprintln!(
            "project #{} issue stats: all={} opened={} closed={}",
            project.id, pc.all, pc.opened, pc.closed
        );
    }
}

/// Exercises the Iterations category via the first visible group and its
/// cadences, plus project-inherited iterations.
#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn iterations_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");

    if let Some(group) = client.groups().expect("groups failed").first() {
        match client.group_iterations(group.id.into()) {
            Ok(iters) => {
                eprintln!("group #{} iterations: {}", group.id, iters.len());
                for it in iters.iter().take(5) {
                    eprintln!(
                        "  #{} [{:?}] {} ({}..{})",
                        it.id,
                        it.state,
                        it.title.as_deref().unwrap_or("?"),
                        it.start_date.as_deref().unwrap_or("?"),
                        it.due_date.as_deref().unwrap_or("?")
                    );
                }
            }
            Err(e) => eprintln!("group iterations not accessible: {e}"),
        }
        match client.group_iteration_cadences(group.id.into()) {
            Ok(cadences) => eprintln!("group #{} cadences: {}", group.id, cadences.len()),
            Err(e) => eprintln!("group cadences not accessible: {e}"),
        }
    }

    if let Some(project) = client.projects().expect("projects failed").first() {
        match client.project_iterations(i64::from(project.id)) {
            Ok(iters) => eprintln!("project #{} iterations: {}", project.id, iters.len()),
            Err(e) => eprintln!("project iterations not accessible: {e}"),
        }
    }
}

/// Exercises the Boards category: project boards (+ lists) and group boards.
#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn boards_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");

    if let Some(project) = client.projects().expect("projects failed").first() {
        let pid = i64::from(project.id);
        let boards = client.project_boards(pid).expect("project_boards failed");
        eprintln!("project #{pid} boards: {}", boards.len());
        if let Some(first) = boards.first() {
            let one = client
                .project_board(pid, first.id.into())
                .expect("project_board failed");
            assert_eq!(one.id, first.id);
            let lists = client
                .project_board_lists(pid, first.id.into())
                .expect("project_board_lists failed");
            eprintln!("  board {} lists: {}", first.id, lists.len());
            for l in lists.iter().take(5) {
                eprintln!(
                    "    [{}] {}",
                    l.list_type.as_deref().unwrap_or("?"),
                    l.label.as_ref().map_or("-", |lb| lb.name.as_str())
                );
            }
        }
    }

    if let Some(group) = client.groups().expect("groups failed").first() {
        match client.group_boards(group.id.into()) {
            Ok(boards) => eprintln!("group #{} boards: {}", group.id, boards.len()),
            Err(e) => eprintln!("group boards not accessible: {e}"),
        }
    }
}

/// Exercises the Epics category (premium): group epics, a single epic, its
/// issues and child epics. Tolerates 403/404 on non-premium instances.
#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn epics_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");

    let Some(group) = client.groups().expect("groups failed").first().cloned() else {
        eprintln!("no groups visible - skipping epics");
        return;
    };
    let gid = i64::from(group.id);
    match client.group_epics(gid) {
        Ok(epics) => {
            eprintln!("group #{} epics: {}", gid, epics.len());
            if let Some(first) = epics.first() {
                let eiid = i64::from(first.iid);
                let one = client.epic(gid, eiid).expect("epic failed");
                assert_eq!(one.iid, first.iid);
                let issues = client.epic_issues(gid, eiid).expect("epic_issues failed");
                eprintln!("  epic {} issues: {}", first.iid, issues.len());
                let children = client
                    .epic_children(gid, eiid)
                    .expect("epic_children failed");
                eprintln!("  epic {} children: {}", first.iid, children.len());
            }
        }
        Err(e) => eprintln!("epics not accessible (likely non-premium): {e}"),
    }
}

/// Exercises the Issue links + Resource events categories: find an issue and
/// round-trip its links and label/state/milestone events.
#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn issue_links_and_events_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let query = IssueQuery::new()
        .with_scope(IssueScope::All)
        .with_state(IssueStateFilter::All);
    let issues = client.issues(&query).expect("issues request failed");

    let mut linked = false;
    let mut evented = false;
    for issue in issues.iter().take(50) {
        let Some(pid) = issue.project_id else {
            continue;
        };
        let pid = i64::from(pid);
        let iid = i64::from(issue.iid);

        if !linked {
            let links = client.issue_links(pid, iid).expect("issue_links failed");
            if !links.is_empty() {
                eprintln!("issue {pid}!{iid} links: {}", links.len());
                for l in links.iter().take(5) {
                    eprintln!("  [{:?}] !{}", l.link_type, l.iid);
                }
                linked = true;
            }
        }

        if !evented {
            let states = client
                .issue_state_events(pid, iid)
                .expect("issue_state_events failed");
            let labels = client
                .issue_label_events(pid, iid)
                .expect("issue_label_events failed");
            let miles = client
                .issue_milestone_events(pid, iid)
                .expect("issue_milestone_events failed");
            if !states.is_empty() || !labels.is_empty() || !miles.is_empty() {
                eprintln!(
                    "issue {pid}!{iid} events: {} state, {} label, {} milestone",
                    states.len(),
                    labels.len(),
                    miles.len()
                );
                evented = true;
            }
        }

        if linked && evented {
            break;
        }
    }
    if !linked {
        eprintln!("no issue links found in first 50 issues");
    }
    if !evented {
        eprintln!("no resource events found in first 50 issues");
    }
}

/// Exercises the Metadata endpoint + Namespaces listing.
#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn metadata_and_namespaces_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let meta = client.metadata().expect("metadata failed");
    eprintln!(
        "instance version {} (enterprise={}, kas={})",
        meta.version.as_deref().unwrap_or("?"),
        meta.enterprise,
        meta.kas.enabled
    );

    let namespaces = client.namespaces().expect("namespaces failed");
    eprintln!("visible namespaces: {}", namespaces.len());
    for n in namespaces.iter().take(5) {
        eprintln!(
            "  #{} [{}] {}",
            n.id,
            n.kind.as_deref().unwrap_or("?"),
            n.full_path.as_deref().unwrap_or("?")
        );
    }
}

/// Exercises Commit statuses + Job artifacts: find a project with a
/// pipeline, read the head commit's statuses and a job's artifacts.
#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn commit_statuses_and_artifacts_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let projects = client.projects().expect("projects failed");

    for p in projects.iter().take(20) {
        let pid = i64::from(p.id);
        let Ok(commits) = client.commits(pid, None) else {
            continue;
        };
        let Some(head) = commits.first() else {
            continue;
        };
        let statuses = client
            .commit_statuses(pid, &head.id)
            .expect("commit_statuses failed");
        if !statuses.is_empty() {
            eprintln!(
                "project #{pid} commit {} statuses: {}",
                &head.id[..8.min(head.id.len())],
                statuses.len()
            );
            for s in statuses.iter().take(5) {
                eprintln!(
                    "  {} [{:?}] {}",
                    s.name.as_deref().unwrap_or("?"),
                    s.status,
                    s.target_url.as_deref().unwrap_or("")
                );
            }
            return;
        }
    }
    eprintln!("no commit statuses found in first 20 projects");
}

/// Exercises Pipeline schedules across the first projects.
#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn pipeline_schedules_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let projects = client.projects().expect("projects failed");

    for p in projects.iter().take(20) {
        let pid = i64::from(p.id);
        let Ok(schedules) = client.pipeline_schedules(pid) else {
            continue;
        };
        if let Some(first) = schedules.first() {
            eprintln!("project #{pid} pipeline schedules: {}", schedules.len());
            let one = client
                .pipeline_schedule(pid, first.id.into())
                .expect("pipeline_schedule failed");
            assert_eq!(one.id, first.id);
            eprintln!(
                "  #{} '{}' cron={} active={}",
                one.id,
                one.description.as_deref().unwrap_or("?"),
                one.cron.as_deref().unwrap_or("?"),
                one.active
            );
            return;
        }
    }
    eprintln!("no pipeline schedules found in first 20 projects");
}

/// Exercises Protected branches/tags/environments on the first project.
#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn protected_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let Some(project) = client.projects().expect("projects failed").first().cloned() else {
        eprintln!("no projects - skipping");
        return;
    };
    let pid = i64::from(project.id);

    let branches = client
        .protected_branches(pid)
        .expect("protected_branches failed");
    eprintln!("project #{pid} protected branches: {}", branches.len());
    for b in branches.iter().take(5) {
        eprintln!(
            "  {} (push rules {}, merge rules {})",
            b.name,
            b.push_access_levels.len(),
            b.merge_access_levels.len()
        );
    }
    if let Some(first) = branches.first() {
        let one = client
            .protected_branch(pid, &first.name)
            .expect("protected_branch failed");
        assert_eq!(one.name, first.name);
    }

    let tags = client.protected_tags(pid).expect("protected_tags failed");
    eprintln!("  protected tags: {}", tags.len());

    match client.protected_environments(pid) {
        Ok(envs) => eprintln!("  protected environments: {}", envs.len()),
        Err(e) => eprintln!("  protected environments not accessible: {e}"),
    }
}

/// Exercises Emoji reactions: find an issue with reactions and read them.
#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn award_emoji_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let query = IssueQuery::new()
        .with_scope(IssueScope::All)
        .with_state(IssueStateFilter::All);
    let issues = client.issues(&query).expect("issues failed");

    for issue in issues.iter().take(50) {
        let Some(pid) = issue.project_id else {
            continue;
        };
        let awards = client
            .issue_award_emoji(i64::from(pid), i64::from(issue.iid))
            .expect("issue_award_emoji failed");
        if !awards.is_empty() {
            eprintln!("issue {pid}!{} reactions: {}", issue.iid, awards.len());
            for a in awards.iter().take(10) {
                eprintln!("  :{}:", a.name.as_deref().unwrap_or("?"));
            }
            return;
        }
    }
    eprintln!("no emoji reactions found in first 50 issues");
}

/// Exercises Group releases + Group wikis via the first visible group.
#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn group_releases_and_wikis_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let Some(group) = client.groups().expect("groups failed").first().cloned() else {
        eprintln!("no groups - skipping");
        return;
    };

    let gid = i64::from(group.id);
    match client.group_releases(gid) {
        Ok(rels) => eprintln!("group #{} releases: {}", gid, rels.len()),
        Err(e) => eprintln!("group releases not accessible: {e}"),
    }

    match client.group_wiki_pages(gid) {
        Ok(pages) => {
            eprintln!("group #{} wiki pages: {}", gid, pages.len());
            if let Some(first) = pages.first() {
                let one = client
                    .group_wiki_page(gid, &first.slug)
                    .expect("group_wiki_page failed");
                eprintln!("  wiki '{}' fetched", one.title);
            }
        }
        Err(e) => eprintln!("group wikis not accessible: {e}"),
    }
}

// ===========================================================================
// Write smoke tests. Each performs a full create -> mutate -> delete cycle
// and cleans up after itself. They mutate the target project, so they are
// ignored by default and gated behind an explicit opt-in.
// ===========================================================================

/// Issue lifecycle: create -> comment -> react -> update/close -> delete.
#[test]
#[ignore = "mutates a real project; requires GITLAB_URL + GITLAB_PAT"]
fn write_issue_lifecycle() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let pid = write_test_project(&client);
    let pid32 = i32::try_from(pid).expect("project id fits i32");

    let created = client
        .create_issue(
            pid32,
            &CreateIssue::new("[kaj-tinamit] write smoke test")
                .with_description("Created by the kaj-tinamit-client write test.")
                .with_labels(&["kaj-tinamit-test"]),
        )
        .expect("create_issue failed");
    eprintln!("created issue !{} (id {})", created.iid, created.id);
    let iid = i64::from(created.iid);

    let note = client
        .create_issue_note(pid, iid, "Automated comment from the write test.")
        .expect("create_issue_note failed");
    eprintln!("  added note #{}", note.id);

    client
        .update_issue_note(pid, iid, note.id.into(), "Edited comment.")
        .expect("update_issue_note failed");

    let award = client
        .award_issue_emoji(pid, iid, "thumbsup")
        .expect("award_issue_emoji failed");
    eprintln!("  reacted :{}:", award.name.as_deref().unwrap_or("?"));
    client
        .remove_issue_award_emoji(pid, iid, award.id.into())
        .expect("remove_issue_award_emoji failed");

    let updated = client
        .update_issue(
            pid32,
            i32::try_from(i64::from(created.iid)).expect("issue iid fits i32"),
            &UpdateIssue::new()
                .with_title("[kaj-tinamit] write smoke test (edited)")
                .close(),
        )
        .expect("update_issue failed");
    eprintln!("  updated + closed: state {:?}", updated.state);

    client.delete_issue_note(pid, iid, note.id.into()).ok();
    // Deleting an issue requires the Owner role; tolerate a 403 when the
    // test token lacks it (the issue is already closed above).
    match client.delete_issue(
        pid32,
        i32::try_from(i64::from(created.iid)).expect("issue iid fits i32"),
    ) {
        Ok(()) => eprintln!("  deleted issue !{}", created.iid),
        Err(e) => eprintln!("  delete_issue skipped (needs Owner): {e}"),
    }
}

/// Label lifecycle: create -> update -> delete.
#[test]
#[ignore = "mutates a real project; requires GITLAB_URL + GITLAB_PAT"]
fn write_label_lifecycle() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let pid = write_test_project(&client);

    let name = "kaj-tinamit-test";
    // Best-effort cleanup of a leftover from a prior run.
    client.delete_project_label(pid, name).ok();

    let label = client
        .create_project_label(
            pid,
            &CreateLabel::new(name, "#6699cc").with_description("temp"),
        )
        .expect("create_project_label failed");
    eprintln!("created label {} {}", label.color, label.name);

    let updated = client
        .update_project_label(
            pid,
            name,
            &kaj_tinamit_client::UpdateLabel::new().with_color("#cc6699"),
        )
        .expect("update_project_label failed");
    eprintln!("  recolored to {}", updated.color);

    client
        .delete_project_label(pid, name)
        .expect("delete_project_label failed");
    eprintln!("  deleted label {name}");
}

/// Exercises the CI/CD variables category read side.
#[test]
#[ignore = "requires GITLAB_URL + GITLAB_PAT and network"]
fn variables_smoke() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let pid = write_test_project(&client);

    let variables = client
        .project_variables(pid)
        .expect("project_variables failed");
    eprintln!("project #{pid} variables: {}", variables.len());
    for v in variables.iter().take(10) {
        eprintln!(
            "  {} scope={} {:?} hidden={}",
            v.key,
            v.environment_scope.as_deref().unwrap_or("*"),
            v.visibility(),
            v.hidden
        );
    }

    if let Some(first) = variables.iter().find(|v| v.applies_everywhere()) {
        let one = client
            .project_variable(pid, &first.key, &VariableFilter::new())
            .expect("project_variable failed");
        assert_eq!(one.key, first.key);
    }
}

/// CI/CD variable lifecycle: create -> update -> delete.
#[test]
#[ignore = "mutates a real project; requires GITLAB_URL + GITLAB_PAT"]
fn write_variable_lifecycle() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let pid = write_test_project(&client);

    let key = "KAJ_TINAMIT_TEST";
    let filter = VariableFilter::new();
    client.delete_project_variable(pid, key, &filter).ok();

    let created = client
        .create_project_variable(
            pid,
            &CreateVariable::new(key, "initial").with_description("temp"),
        )
        .expect("create_project_variable failed");
    eprintln!("created variable {}", created.key);
    assert_eq!(created.value.as_deref(), Some("initial"));

    let updated = client
        .update_project_variable(pid, key, &UpdateVariable::new("updated"), &filter)
        .expect("update_project_variable failed");
    assert_eq!(updated.value.as_deref(), Some("updated"));
    eprintln!("  updated value");

    client
        .delete_project_variable(pid, key, &filter)
        .expect("delete_project_variable failed");
    eprintln!("  deleted variable {key}");
}

/// Repository lifecycle: create branch -> commit a file -> delete branch.
#[test]
#[ignore = "mutates a real project; requires GITLAB_URL + GITLAB_PAT"]
fn write_repository_lifecycle() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let pid = write_test_project(&client);

    // Pick the default branch to fork from.
    let branches = client.branches(pid).expect("branches failed");
    let base = branches
        .iter()
        .find(|b| b.default)
        .or_else(|| branches.first())
        .expect("repository has a branch")
        .name
        .clone();

    let work = "kaj-tinamit-test-branch";
    client.delete_branch(pid, work).ok();

    let created = client
        .create_branch(pid, work, &base)
        .expect("create_branch failed");
    eprintln!("created branch {} from {}", created.name, base);

    let commit = client
        .create_commit(
            pid,
            &CreateCommit::new(
                work,
                "Add kaj-tinamit test file",
                vec![CommitAction::create(
                    "kaj-tinamit-test.txt",
                    "hello from the write test\n",
                )],
            ),
        )
        .expect("create_commit failed");
    eprintln!("  committed {}", commit.id);

    client
        .delete_branch(pid, work)
        .expect("delete_branch failed");
    eprintln!("  deleted branch {work}");
}

/// Environment lifecycle: create -> update -> stop -> delete.
#[test]
#[ignore = "mutates a real project; requires GITLAB_URL + GITLAB_PAT"]
fn write_environment_lifecycle() {
    let client = GitlabClient::from_env().expect("GITLAB_URL/GITLAB_PAT must be set");
    let pid = write_test_project(&client);

    let created = client
        .create_environment(
            pid,
            &CreateEnvironment::new("kaj-tinamit-test-env")
                .with_external_url("https://kaj-tinamit.example.com")
                .with_tier("other"),
        )
        .expect("create_environment failed");
    eprintln!("created environment {} (id {})", created.name, created.id);

    let updated = client
        .update_environment(
            pid,
            created.id.into(),
            &UpdateEnvironment::new().with_description("edited by the write test"),
        )
        .expect("update_environment failed");
    eprintln!(
        "  updated: description {:?}",
        updated.description.as_deref()
    );

    // Stopping is a prerequisite for deletion.
    client
        .stop_environment(pid, created.id.into())
        .expect("stop_environment failed");
    eprintln!("  stopped environment {}", created.id);

    client
        .delete_environment(pid, created.id.into())
        .expect("delete_environment failed");
    eprintln!("  deleted environment {}", created.id);
}
