//! Issue-related wire shapes.

use json_bourne::{FromJson, Lexer, ToJson};

use crate::user::UserState;

#[derive(Debug, FromJson, ToJson, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[bourne(rename_all = "lowercase")]
pub enum IssueState {
    Opened,
    Closed,
}

/// Issue type. GitLab's REST API reports this in SCREAMING_SNAKE_CASE via
/// the top-level `type` field (e.g. `ISSUE`, `TEST_CASE`) but lowercase via
/// `issue_type`. We accept both casings and fall back to [`IssueType::Unknown`]
/// for values GitLab may add later, so parsing never fails on a new kind.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum IssueType {
    Issue,
    Incident,
    TestCase,
    Task,
    Unknown,
}

impl<'input> FromJson<'input> for IssueType {
    fn from_lex(lex: &mut Lexer<'input>) -> Result<Self, json_bourne::Error> {
        let s = String::from_lex(lex)?;
        Ok(match s.as_str() {
            "ISSUE" | "issue" => Self::Issue,
            "INCIDENT" | "incident" => Self::Incident,
            "TEST_CASE" | "test_case" => Self::TestCase,
            "TASK" | "task" => Self::Task,
            _ => Self::Unknown,
        })
    }
}

impl ToJson for IssueType {
    fn write_json<W: json_bourne::JsonWrite + ?Sized>(&self, w: &mut W) -> Result<(), W::Error> {
        let s = match self {
            Self::Issue => "ISSUE",
            Self::Incident => "INCIDENT",
            Self::TestCase => "TEST_CASE",
            Self::Task => "TASK",
            Self::Unknown => "UNKNOWN",
        };
        s.write_json(w)
    }
}

#[derive(Debug, FromJson, ToJson, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[bourne(rename_all = "UPPERCASE")]
pub enum Severity {
    Unknown,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(FromJson, ToJson, Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
#[bourne(deny_unknown_fields = false)]
pub struct Author {
    pub state: Option<UserState>,
    pub id: i32,
    pub web_url: Option<String>,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, FromJson, ToJson, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[bourne(rename_all = "lowercase")]
pub enum MilestoneState {
    Active,
    Closed,
}

#[derive(FromJson, ToJson, Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
#[bourne(deny_unknown_fields = false)]
pub struct Milestone {
    pub project_id: Option<i32>,
    pub description: Option<String>,
    pub state: Option<MilestoneState>,
    pub due_date: Option<String>,
    #[bourne(default)]
    pub iid: i32,
    pub created_at: Option<String>,
    pub title: Option<String>,
    #[bourne(default)]
    pub id: i32,
    pub updated_at: Option<String>,
}

#[derive(FromJson, ToJson, Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
#[bourne(deny_unknown_fields = false)]
pub struct Assignee {
    pub state: Option<UserState>,
    pub id: i32,
    pub name: Option<String>,
    pub web_url: Option<String>,
    pub avatar_url: Option<String>,
    pub username: Option<String>,
}

#[derive(FromJson, ToJson, Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Default)]
#[bourne(deny_unknown_fields = false)]
pub struct TimeStats {
    #[bourne(default)]
    pub time_estimate: i32,
    #[bourne(default)]
    pub total_time_spent: i32,
    pub human_time_estimate: Option<String>,
    pub human_total_time_spent: Option<String>,
}

#[derive(FromJson, ToJson, Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Default)]
#[bourne(deny_unknown_fields = false)]
pub struct References {
    pub short: Option<String>,
    pub relative: Option<String>,
    pub full: Option<String>,
}

#[derive(FromJson, ToJson, Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Default)]
#[bourne(deny_unknown_fields = false)]
pub struct IssueLinks {
    #[bourne(rename = "self")]
    pub self_link: Option<String>,
    pub notes: Option<String>,
    pub award_emoji: Option<String>,
    pub project: Option<String>,
    pub closed_as_duplicate_of: Option<String>,
}

#[derive(FromJson, ToJson, Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Default)]
#[bourne(deny_unknown_fields = false)]
pub struct TaskCompletionStatus {
    #[bourne(default)]
    pub count: i32,
    #[bourne(default)]
    pub completed_count: i32,
}

#[derive(FromJson, ToJson, Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
#[bourne(deny_unknown_fields = false)]
pub struct GitlabIssue {
    pub state: Option<IssueState>,
    pub description: Option<String>,
    pub author: Option<Author>,
    pub milestone: Option<Milestone>,
    pub project_id: Option<i32>,
    #[bourne(default)]
    pub assignees: Vec<Assignee>,
    pub assignee: Option<Assignee>,
    #[bourne(rename = "type")]
    pub issue_type: Option<IssueType>,
    pub updated_at: Option<String>,
    pub closed_at: Option<String>,
    pub id: i32,
    pub title: Option<String>,
    pub created_at: Option<String>,
    pub moved_to_id: Option<i32>,
    pub iid: i32,
    #[bourne(default)]
    pub labels: Vec<String>,
    #[bourne(default)]
    pub upvotes: i32,
    #[bourne(default)]
    pub downvotes: i32,
    #[bourne(default)]
    pub merge_requests_count: i32,
    #[bourne(default)]
    pub user_notes_count: i32,
    pub due_date: Option<String>,
    pub imported: Option<bool>,
    pub imported_from: Option<String>,
    pub web_url: Option<String>,
    #[bourne(default)]
    pub references: References,
    #[bourne(default)]
    pub time_stats: TimeStats,
    pub has_tasks: Option<bool>,
    pub task_status: Option<String>,
    pub confidential: Option<bool>,
    pub discussion_locked: Option<bool>,
    pub severity: Option<Severity>,
    #[bourne(default, rename = "_links")]
    pub _links: IssueLinks,
    #[bourne(default)]
    pub task_completion_status: TaskCompletionStatus,
}
