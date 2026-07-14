//! Wire shapes for the **Labels** API category (project and group labels).

use json_bourne::{FromJson, ToJson};

/// A project or group label. The same shape is returned by both the project
/// (`/projects/:id/labels`) and group (`/groups/:id/labels`) endpoints; the
/// count fields are only populated when `with_counts=true` is requested.
#[derive(Debug, FromJson, ToJson, Clone, PartialEq, Eq)]
#[bourne(deny_unknown_fields = false)]
pub struct Label {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub text_color: Option<String>,
    pub description: Option<String>,
    pub description_html: Option<String>,
    #[bourne(default)]
    pub open_issues_count: i64,
    #[bourne(default)]
    pub closed_issues_count: i64,
    #[bourne(default)]
    pub open_merge_requests_count: i64,
    #[bourne(default)]
    pub subscribed: bool,
    pub priority: Option<i64>,
    /// Present on project labels: whether it's project-scoped (`true`) or an
    /// inherited group label (`false`). Absent for group-label responses.
    pub is_project_label: Option<bool>,
    #[bourne(default)]
    pub archived: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_project_label() {
        let json = r##"{
            "id": 1,
            "name": "bug",
            "color": "#d9534f",
            "text_color": "#FFFFFF",
            "description": "Bug reported by user",
            "open_issues_count": 1,
            "priority": 10,
            "is_project_label": true,
            "archived": false
        }"##;
        let l: Label = parse_str(json).unwrap();
        assert_eq!(l.name, "bug");
        assert_eq!(l.priority, Some(10));
        assert_eq!(l.is_project_label, Some(true));
    }

    #[test]
    fn parse_group_label_without_counts() {
        // Group labels omit is_project_label and priority.
        let json = r##"{
            "id": 7,
            "name": "feature",
            "color": "#228B22",
            "description": null
        }"##;
        let l: Label = parse_str(json).unwrap();
        assert_eq!(l.id, 7);
        assert!(l.is_project_label.is_none());
        assert!(l.priority.is_none());
        assert!(!l.archived);
    }
}
