#![no_main]

//! Typed parses of arbitrary bytes into every wire model must never panic.
//!
//! Our structs set `deny_unknown_fields = false` and ingest real, untrusted
//! GitLab JSON, so robustness against malformed input matters. Any panic is a
//! bug; any `Result` (Ok or Err) is acceptable.
//!
//! Run: `cargo +nightly fuzz run parse_models`

use gitlab_model::{
    AwardEmoji, Blob, Board, Branch, CommitStatus, Epic, GitlabCommit, GitlabEvent, GitlabGroup,
    GitlabIssue, GitlabProject, GitlabUser, IssueStatistics, Iteration, Job, Label, Member,
    MergeRequest, Metadata, NamespaceListing, PipelineDetail, PipelineSchedule, PipelineSummary,
    ProtectedBranch, RepositoryFile, ResourceLabelEvent, ResourceStateEvent, SearchBlob, Tag, Todo,
    TreeEntry, WikiPage,
};
use json_bourne::parse;
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

    // Arrays of the most common list types, as GitLab returns them.
    let _ = parse::<Vec<GitlabProject>>(data);
    let _ = parse::<Vec<GitlabIssue>>(data);
    let _ = parse::<Vec<Job>>(data);
});
