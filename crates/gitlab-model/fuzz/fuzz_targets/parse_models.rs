#![no_main]

//! Typed parses of arbitrary bytes into every wire model must never panic.
//!
//! Our structs set `deny_unknown_fields = false` and ingest real, untrusted
//! GitLab JSON, so robustness against malformed input matters. Any panic is a
//! bug; any `Result` (Ok or Err) is acceptable.
//!
//! Run: `cargo +nightly fuzz run parse_models`

use gitlab_model::{
    Branch, GitlabCommit, GitlabGroup, GitlabIssue, GitlabProject, GitlabUser, Job, Label, Member,
    PipelineDetail, PipelineSummary, RepositoryFile, Tag, TreeEntry,
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

    // Arrays of the most common list types, as GitLab returns them.
    let _ = parse::<Vec<GitlabProject>>(data);
    let _ = parse::<Vec<GitlabIssue>>(data);
    let _ = parse::<Vec<Job>>(data);
});
