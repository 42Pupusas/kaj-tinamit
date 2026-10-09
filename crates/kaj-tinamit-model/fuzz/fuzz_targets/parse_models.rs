#![no_main]

//! Typed parses of arbitrary bytes into every wire model must never panic.
//!
//! Our structs set `deny_unknown_fields = false` and ingest real, untrusted
//! GitLab JSON, so robustness against malformed input matters. Any panic is a
//! bug; any `Result` (Ok or Err) is acceptable.
//!
//! Run: `cargo +nightly fuzz run parse_models`

use json_bourne::parse;
use kaj_tinamit::{
    AwardEmoji, BlameRange, Blob, Board, BoardList, Branch, Changelog, CiLintResult, CiVariable,
    CommitDiff, CommitStatus, CommitWithDiffs, DeployKey, DeployToken, Deployment, Discussion,
    Environment, Epic, EpicIssue, FileMutationResult, GitlabCommit, GitlabEvent, GitlabGroup,
    GitlabIssue, GitlabProject, GitlabUser, IssueLink, IssueLinkResult, IssueStatistics, Iteration,
    IterationCadence, Job, Label, Member, MergeRequest, MergeRequestApprovals, MergeRequestChanges,
    Metadata, NamespaceListing, PipelineDetail, PipelineSchedule, PipelineSummary, ProjectHook,
    ProtectedBranch, ProtectedEnvironment, ProtectedTag, RefCommit, Release, RepositoryFile,
    ResourceLabelEvent, ResourceMilestoneEvent, ResourceStateEvent, Runner, RunnerDetail,
    SearchBlob, Snippet, Tag, TestReport, TestReportSummary, Todo, TreeEntry, WikiPage,
};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Aggregating many typed parses per input multiplies coverage without
    // growing the corpus. Each covers a distinct field/enum/nesting mix.
    let _ = parse::<GitlabUser>(data);
    let _ = parse::<GitlabProject>(data);
    let _ = parse::<GitlabIssue>(data);
    let _ = parse::<GitlabCommit>(data);
    let _ = parse::<GitlabGroup>(data);
    let _ = parse::<Member>(data);
    let _ = parse::<Label>(data);
    let _ = parse::<Branch>(data);
    let _ = parse::<Tag>(data);
    let _ = parse::<TreeEntry>(data);
    let _ = parse::<RepositoryFile>(data);
    let _ = parse::<PipelineSummary>(data);
    let _ = parse::<PipelineDetail>(data);
    let _ = parse::<Job>(data);
    let _ = parse::<MergeRequest>(data);
    let _ = parse::<GitlabEvent>(data);
    let _ = parse::<WikiPage>(data);
    let _ = parse::<Blob>(data);

    // Metadata, governance, planning, and reaction/audit categories.
    let _ = parse::<Metadata>(data);
    let _ = parse::<NamespaceListing>(data);
    let _ = parse::<ProtectedBranch>(data);
    let _ = parse::<CommitStatus>(data);
    let _ = parse::<PipelineSchedule>(data);
    let _ = parse::<AwardEmoji>(data);
    let _ = parse::<Todo>(data);
    let _ = parse::<SearchBlob>(data);
    let _ = parse::<IssueStatistics>(data);
    let _ = parse::<ResourceLabelEvent>(data);
    let _ = parse::<ResourceStateEvent>(data);
    let _ = parse::<Epic>(data);
    let _ = parse::<Board>(data);
    let _ = parse::<Iteration>(data);

    // Round two: release/snippet/deployment/runner/deploy-credential
    // categories, plus nested MR shapes, dependency links, resource events,
    // and repository shapes not reachable from the top-level types above.
    let _ = parse::<Release>(data);
    let _ = parse::<Snippet>(data);
    let _ = parse::<Deployment>(data);
    let _ = parse::<Environment>(data);
    let _ = parse::<Runner>(data);
    let _ = parse::<RunnerDetail>(data);
    let _ = parse::<DeployKey>(data);
    let _ = parse::<DeployToken>(data);
    let _ = parse::<IssueLink>(data);
    let _ = parse::<IssueLinkResult>(data);
    let _ = parse::<EpicIssue>(data);
    let _ = parse::<BoardList>(data);
    let _ = parse::<IterationCadence>(data);
    let _ = parse::<ResourceMilestoneEvent>(data);
    let _ = parse::<ProtectedTag>(data);
    let _ = parse::<ProtectedEnvironment>(data);
    let _ = parse::<Changelog>(data);
    let _ = parse::<RefCommit>(data);
    let _ = parse::<BlameRange>(data);
    let _ = parse::<FileMutationResult>(data);
    let _ = parse::<CommitDiff>(data);
    let _ = parse::<CommitWithDiffs>(data);
    let _ = parse::<Discussion>(data);
    let _ = parse::<MergeRequestChanges>(data);
    let _ = parse::<MergeRequestApprovals>(data);
    let _ = parse::<CiVariable>(data);
    let _ = parse::<ProjectHook>(data);
    let _ = parse::<TestReport>(data);
    let _ = parse::<TestReportSummary>(data);
    let _ = parse::<CiLintResult>(data);

    // Arrays of the most common list types, as GitLab returns them.
    let _ = parse::<Vec<GitlabProject>>(data);
    let _ = parse::<Vec<GitlabIssue>>(data);
    let _ = parse::<Vec<Job>>(data);
});
