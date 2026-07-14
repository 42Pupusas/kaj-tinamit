//! Property-based round-trip tests for the wire models.
//!
//! Every model derives both `ToJson` and `FromJson`, and — under the
//! `proptest` feature — `proptest::Arbitrary`. The core invariant is
//! serialize/parse **idempotence**:
//!
//! ```text
//! to_string(parse(to_string(x))) == to_string(x)
//! ```
//!
//! We assert on the *serialized form* rather than the value, so no model
//! needs a `PartialEq` impl. This still catches the whole class of bugs
//! example tests miss: `#[bourne(rename)]` mismatches, `Option`/`default`
//! asymmetry between the two derives, and enum-casing errors — across every
//! field at once.
//!
//! `f64` fields (CI durations/coverage) are generated as finite values only
//! (see `finite_opt_f64` in `ci.rs`); arbitrary NaN/inf have no JSON form.

use gitlab_model::*;
use json_bourne::{parse_str, to_string};
use proptest::prelude::*;

/// Generate a value, serialize it, parse it back, and assert the
/// re-serialized form is byte-identical. Works for any model that is
/// `Arbitrary + ToJson + FromJson`.
macro_rules! round_trip {
    ($name:ident, $ty:ty) => {
        proptest! {
            #[test]
            fn $name(x in any::<$ty>()) {
                let once = to_string(&x).expect("serialize");
                let back: $ty = parse_str(&once).expect("parse");
                let twice = to_string(&back).expect("re-serialize");
                prop_assert_eq!(once, twice);
            }
        }
    };
}

// user
round_trip!(user_state, UserState);
round_trip!(user, GitlabUser);

// project
round_trip!(visibility, Visibility);
round_trip!(namespace_kind, NamespaceKind);
round_trip!(namespace, Namespace);
round_trip!(project, GitlabProject);

// issue
round_trip!(issue_state, IssueState);
round_trip!(issue_type, IssueType);
round_trip!(severity, Severity);
round_trip!(author, Author);
round_trip!(milestone_state, MilestoneState);
round_trip!(milestone, Milestone);
round_trip!(assignee, Assignee);
round_trip!(time_stats, TimeStats);
round_trip!(references, References);
round_trip!(issue_links, IssueLinks);
round_trip!(task_completion_status, TaskCompletionStatus);
round_trip!(issue, GitlabIssue);

// commit
round_trip!(commit, GitlabCommit);
round_trip!(commit_diff, CommitDiff);
round_trip!(commit_with_diffs, CommitWithDiffs);
round_trip!(commit_stats, CommitStats);

// merge_request
round_trip!(mr_state, MergeRequestState);
round_trip!(merge_status, MergeStatus);
round_trip!(merge_request, MergeRequest);
round_trip!(mr_user, MergeRequestUser);
round_trip!(mr_milestone, MergeRequestMilestone);
round_trip!(discussion, Discussion);
round_trip!(discussion_note, DiscussionNote);
round_trip!(mr_changes, MergeRequestChanges);
round_trip!(file_change, FileChange);
round_trip!(pipeline_status, PipelineStatus);
round_trip!(pipeline_mr, Pipeline);
round_trip!(mr_approvals, MergeRequestApprovals);
round_trip!(approval, Approval);
round_trip!(noteable_type, NoteableType);
round_trip!(note, Note);

// event
round_trip!(event, GitlabEvent);
round_trip!(event_author, EventAuthor);

// wiki
round_trip!(wiki_format, WikiFormat);
round_trip!(wiki_page, WikiPage);
round_trip!(wiki_page_list, WikiPageList);
round_trip!(wiki_attachment, WikiAttachment);
round_trip!(wiki_attachment_link, WikiAttachmentLink);

// repository
round_trip!(tree_entry_type, TreeEntryType);
round_trip!(tree_entry, TreeEntry);
round_trip!(blob, Blob);
round_trip!(contributor, Contributor);
round_trip!(ref_commit, RefCommit);
round_trip!(branch, Branch);
round_trip!(tag_release, TagRelease);
round_trip!(tag, Tag);
round_trip!(repository_file, RepositoryFile);
round_trip!(blame_commit, BlameCommit);
round_trip!(blame_range, BlameRange);
round_trip!(changelog, Changelog);

// group + members
round_trip!(shared_with_group, SharedWithGroup);
round_trip!(group, GitlabGroup);
round_trip!(member_created_by, MemberCreatedBy);
round_trip!(member, Member);

// label
round_trip!(label, Label);

// ci
round_trip!(ci_status, CiStatus);
round_trip!(ci_user, CiUser);
round_trip!(pipeline_summary, PipelineSummary);
round_trip!(pipeline_detail, PipelineDetail);
round_trip!(pipeline_variable, PipelineVariable);
round_trip!(job_artifact, JobArtifact);
round_trip!(artifacts_file, ArtifactsFile);
round_trip!(job_commit, JobCommit);
round_trip!(job_pipeline, JobPipeline);
round_trip!(job_runner, JobRunner);
round_trip!(job, Job);

// The numeric <-> named access-level mapping is total and invertible.
proptest! {
    #[test]
    fn access_level_is_invertible(raw in any::<i32>()) {
        prop_assert_eq!(AccessLevel::from_raw(raw).as_raw(), raw);
    }
}
