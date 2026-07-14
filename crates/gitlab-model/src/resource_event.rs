//! Wire shapes for the **Resource * events** API categories: label, state,
//! and milestone change events on issues and merge requests. These are the
//! audit trail a card leaves as it moves across a board — the raw material
//! for cycle-time and flow metrics (when did it enter "Doing", when closed).

use json_bourne::{FromJson, ToJson};

use crate::issue::Author;
use crate::label::Label;

/// Whether a label was added or removed in a resource label event.
#[derive(Debug, FromJson, ToJson, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(rename_all = "lowercase")]
pub enum LabelEventAction {
    Add,
    Remove,
}

/// A resource label event: a label added to or removed from an issue/MR at a
/// point in time. `label` may be null if the label was later deleted.
#[derive(Debug, FromJson, ToJson, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct ResourceLabelEvent {
    pub id: i64,
    pub user: Option<Author>,
    pub created_at: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<i64>,
    pub label: Option<Label>,
    pub action: Option<LabelEventAction>,
}

/// The state transition captured by a resource state event (`opened`,
/// `closed`, `reopened`, `merged`).
#[derive(Debug, FromJson, ToJson, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(rename_all = "lowercase")]
pub enum StateEventState {
    Opened,
    Closed,
    Reopened,
    Merged,
}

/// A resource state event: when an issue/MR was opened, closed, reopened, or
/// merged. The backbone of "time to close" measurements.
#[derive(Debug, FromJson, ToJson, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct ResourceStateEvent {
    pub id: i64,
    pub user: Option<Author>,
    pub created_at: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<i64>,
    pub state: Option<StateEventState>,
}

/// Whether a milestone was assigned or removed in a milestone event.
#[derive(Debug, FromJson, ToJson, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(rename_all = "lowercase")]
pub enum MilestoneEventAction {
    Add,
    Remove,
}

/// A resource milestone event: an issue/MR being added to or removed from a
/// milestone. Lets you reconstruct scope changes within a sprint.
#[derive(Debug, FromJson, ToJson, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct ResourceMilestoneEvent {
    pub id: i64,
    pub user: Option<Author>,
    pub created_at: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<i64>,
    pub action: Option<MilestoneEventAction>,
    pub milestone: Option<crate::issue::Milestone>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_label_event() {
        let json = r##"{
            "id": 142,
            "created_at": "2021-01-02T09:00:00Z",
            "resource_type": "Issue",
            "resource_id": 11,
            "label": {"id": 5, "name": "Doing", "color": "#cc0033"},
            "action": "add"
        }"##;
        let e: ResourceLabelEvent = parse_str(json).unwrap();
        assert_eq!(e.action, Some(LabelEventAction::Add));
        assert_eq!(e.label.unwrap().name, "Doing");
    }

    #[test]
    fn parse_state_event() {
        let json = r#"{
            "id": 200,
            "created_at": "2021-01-05T12:00:00Z",
            "resource_type": "Issue",
            "resource_id": 11,
            "state": "closed"
        }"#;
        let e: ResourceStateEvent = parse_str(json).unwrap();
        assert_eq!(e.state, Some(StateEventState::Closed));
    }
}
