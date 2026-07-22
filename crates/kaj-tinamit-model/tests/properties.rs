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

use json_bourne::{parse_str, to_string};
use kaj_tinamit::*;
use proptest::prelude::*;

/// Generate a value, serialize it, parse it back, and assert the
/// re-serialized form is byte-identical. Works for any model that is
/// `Arbitrary + ToJson + FromJson`.
///
/// The body runs on a worker thread with a large (32 MiB) stack:
/// proptest-derive builds a deeply nested tuple strategy for our biggest
/// composite models (e.g. `Environment`, which embeds the whole
/// `Deployable` tree), and generating those value trees can exceed the
/// 2 MiB default test-thread stack.
macro_rules! round_trip {
    ($name:ident, $ty:ty) => {
        #[test]
        fn $name() {
            std::thread::Builder::new()
                .stack_size(32 * 1024 * 1024)
                .spawn(|| {
                    proptest!(|(x in any::<$ty>())| {
                        let once = to_string(&x).expect("serialize");
                        let back: $ty = parse_str(&once).expect("parse");
                        let twice = to_string(&back).expect("re-serialize");
                        prop_assert_eq!(once, twice);
                    });
                })
                .expect("spawn property-test thread")
                .join()
                .expect("property-test thread panicked");
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

// issue_link
round_trip!(issue_link_type, IssueLinkType);
round_trip!(issue_link, IssueLink);
round_trip!(issue_link_result, IssueLinkResult);

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
round_trip!(file_mutation_result, FileMutationResult);
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

// release
round_trip!(release_author, ReleaseAuthor);
round_trip!(release_commit, ReleaseCommit);
round_trip!(milestone_issue_stats, MilestoneIssueStats);
round_trip!(release_milestone, ReleaseMilestone);
round_trip!(release_source, ReleaseSource);
round_trip!(release_link, ReleaseLink);
round_trip!(release_assets, ReleaseAssets);
round_trip!(release_evidence, ReleaseEvidence);
round_trip!(release, Release);

// snippet
round_trip!(snippet_author, SnippetAuthor);
round_trip!(snippet_file, SnippetFile);
round_trip!(snippet, Snippet);
round_trip!(snippet_user_agent_detail, SnippetUserAgentDetail);

// deployment + environment
round_trip!(deployment_user, DeploymentUser);
round_trip!(deployable_commit, DeployableCommit);
round_trip!(deployable_pipeline, DeployablePipeline);
round_trip!(deployable, Deployable);
round_trip!(environment_ref, EnvironmentRef);
round_trip!(deployment, Deployment);
round_trip!(last_deployment, LastDeployment);
round_trip!(environment, Environment);

// runner
round_trip!(runner, Runner);
round_trip!(runner_project, RunnerProject);
round_trip!(runner_detail, RunnerDetail);
round_trip!(runner_manager, RunnerManager);
round_trip!(runner_auth_token, RunnerAuthToken);

// note
round_trip!(note_author, NoteAuthor);
round_trip!(gitlab_note, GitlabNote);

// deploy keys + tokens
round_trip!(deploy_key_project, DeployKeyProject);
round_trip!(deploy_key, DeployKey);
round_trip!(deploy_token, DeployToken);
round_trip!(created_deploy_token, CreatedDeployToken);

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

// metadata + namespaces
round_trip!(kas_metadata, KasMetadata);
round_trip!(metadata, Metadata);
round_trip!(namespace_listing, NamespaceListing);

// protected branches / tags / environments
round_trip!(access_rule, AccessRule);
round_trip!(protected_branch, ProtectedBranch);
round_trip!(protected_tag, ProtectedTag);
round_trip!(protected_environment, ProtectedEnvironment);

// award emoji
round_trip!(award_emoji, AwardEmoji);

// pipeline schedules
round_trip!(schedule_pipeline, SchedulePipeline);
round_trip!(pipeline_schedule, PipelineSchedule);

// commit status
round_trip!(commit_status, CommitStatus);

// todos
round_trip!(todo_action_name, TodoActionName);
round_trip!(todo_state, TodoState);
round_trip!(todo_project, TodoProject);
round_trip!(todo, Todo);

// search
round_trip!(search_blob, SearchBlob);

// issue statistics
round_trip!(issue_counts, IssueCounts);
round_trip!(issue_statistics_inner, IssueStatisticsInner);
round_trip!(issue_statistics, IssueStatistics);

// resource events
round_trip!(label_event_action, LabelEventAction);
round_trip!(resource_label_event, ResourceLabelEvent);
round_trip!(state_event_state, StateEventState);
round_trip!(resource_state_event, ResourceStateEvent);
round_trip!(milestone_event_action, MilestoneEventAction);
round_trip!(resource_milestone_event, ResourceMilestoneEvent);

// epics
round_trip!(epic_state, EpicState);
round_trip!(epic, Epic);
round_trip!(epic_issue, EpicIssue);

// boards
round_trip!(board_list, BoardList);
round_trip!(board, Board);

// iterations
round_trip!(iteration_state, IterationState);
round_trip!(iteration, Iteration);
round_trip!(iteration_cadence, IterationCadence);

// The numeric <-> named access-level mapping is total and invertible.
proptest! {
    #[test]
    fn access_level_is_invertible(raw in any::<i32>()) {
        prop_assert_eq!(AccessLevel::from_raw(raw).as_raw(), raw);
    }
}
