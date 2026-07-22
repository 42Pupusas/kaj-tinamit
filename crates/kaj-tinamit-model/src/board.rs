//! Wire shapes for the **Boards** API category (project and group issue
//! boards). A board is an ordered set of lists; each list is usually backed
//! by a label, and issues flow left-to-right across them.

use json_bourne::{FromJson, ToJson};

use crate::Id;
use crate::issue::Milestone;
use crate::label::Label;

/// A single list (column) on a board. Most lists are label-backed; the
/// backlog and closed lists have no label. `list_type` distinguishes them
/// (`label`, `backlog`, `closed`, `assignee`, `milestone`, `iteration`).
#[derive(Debug, FromJson, ToJson, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct BoardList {
    pub id: Id,
    /// Column ordering, left to right (1-based). Absent for backlog/closed.
    pub position: Option<i64>,
    pub list_type: Option<String>,
    pub label: Option<Label>,
    #[bourne(default)]
    pub collapsed: bool,
    /// Work-in-progress limit, when one is configured on the list.
    pub max_issue_count: Option<i64>,
    pub max_issue_weight: Option<i64>,
}

/// An issue board (project- or group-scoped). The `project` / `group`
/// fields are populated depending on which endpoint returned it.
#[derive(Debug, FromJson, ToJson, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct Board {
    pub id: Id,
    pub name: Option<String>,
    #[bourne(default)]
    pub hide_backlog_list: bool,
    #[bourne(default)]
    pub hide_closed_list: bool,
    /// The board's scoping milestone, when the board is milestone-scoped.
    pub milestone: Option<Milestone>,
    /// Labels the board is scoped to (premium scoped-board feature).
    #[bourne(default)]
    pub labels: Vec<Label>,
    /// The lists (columns), in board order.
    #[bourne(default)]
    pub lists: Vec<BoardList>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_board_with_lists() {
        let json = r##"{
            "id": 1,
            "name": "Development",
            "hide_backlog_list": false,
            "hide_closed_list": false,
            "lists": [
                {
                    "id": 10,
                    "position": 1,
                    "list_type": "label",
                    "label": {"id": 5, "name": "Doing", "color": "#cc0033"}
                }
            ]
        }"##;
        let b: Board = parse_str(json).unwrap();
        assert_eq!(b.name.as_deref(), Some("Development"));
        assert_eq!(b.lists.len(), 1);
        assert_eq!(b.lists[0].position, Some(1));
        assert_eq!(b.lists[0].label.as_ref().unwrap().name, "Doing");
    }
}
