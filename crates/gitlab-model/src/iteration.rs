//! Wire shapes for the **Iterations** API category — GitLab's native
//! sprints. Iterations live on a group (and are inherited by its projects);
//! they're grouped into *cadences* that define the sprint schedule.

use json_bourne::{FromJson, ToJson};

use crate::Id;

/// Iteration lifecycle state. GitLab reports this as an integer code, not a
/// string: 1 = upcoming, 2 = current, 3 = closed. We map the closed set and
/// fall back to [`IterationState::Unknown`] for anything new.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[non_exhaustive]
pub enum IterationState {
    Upcoming,
    Current,
    Closed,
    Unknown,
}

impl<'input> FromJson<'input> for IterationState {
    fn from_lex(lex: &mut json_bourne::Lexer<'input>) -> Result<Self, json_bourne::Error> {
        let code = i64::from_lex(lex)?;
        Ok(match code {
            1 => Self::Upcoming,
            2 => Self::Current,
            3 => Self::Closed,
            _ => Self::Unknown,
        })
    }
}

impl ToJson for IterationState {
    fn write_json<W: json_bourne::JsonWrite + ?Sized>(&self, w: &mut W) -> Result<(), W::Error> {
        let code: i64 = match self {
            Self::Upcoming => 1,
            Self::Current => 2,
            Self::Closed => 3,
            Self::Unknown => 0,
        };
        code.write_json(w)
    }
}

/// An iteration (sprint). Returned by the group and project iteration
/// listings.
#[derive(Debug, FromJson, ToJson, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct Iteration {
    pub id: Id,
    #[bourne(default)]
    pub iid: Id,
    /// The owning group; iterations are never project-scoped in GitLab even
    /// when queried through a project.
    pub group_id: Option<Id>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub state: Option<IterationState>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    /// The cadence this iteration belongs to.
    pub iteration_cadence_id: Option<Id>,
    pub web_url: Option<String>,
    pub sequence: Option<i64>,
}

/// An iteration cadence — the recurring schedule iterations are generated
/// from (duration, automatic rollover, etc.).
#[derive(Debug, FromJson, ToJson, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct IterationCadence {
    pub id: Id,
    pub title: Option<String>,
    pub description: Option<String>,
    #[bourne(default)]
    pub active: bool,
    #[bourne(default)]
    pub automatic: bool,
    pub start_date: Option<String>,
    /// Length of each iteration in weeks.
    pub duration_in_weeks: Option<i64>,
    /// How many upcoming iterations are kept scheduled ahead of time.
    pub iterations_in_advance: Option<i64>,
    #[bourne(default)]
    pub roll_over: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_iteration_with_integer_state() {
        let json = r#"{
            "id": 53,
            "iid": 13,
            "group_id": 5,
            "title": "Sprint 42",
            "state": 2,
            "start_date": "2021-12-06",
            "due_date": "2021-12-17",
            "web_url": "https://example.com/groups/g/-/iterations/13"
        }"#;
        let it: Iteration = parse_str(json).unwrap();
        assert_eq!(it.iid, 13);
        assert_eq!(it.state, Some(IterationState::Current));
    }

    #[test]
    fn unknown_state_code_falls_back() {
        let json = r#"{"id":1,"state":99}"#;
        let it: Iteration = parse_str(json).unwrap();
        assert_eq!(it.state, Some(IterationState::Unknown));
    }
}
