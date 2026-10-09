//! Filters for listing projects (`GET /projects`).

use kaj_tinamit::{AccessLevel, Visibility};

use crate::query::QueryParams;

/// The field to sort a project listing by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectOrder {
    Id,
    Name,
    Path,
    CreatedAt,
    UpdatedAt,
    LastActivityAt,
    StarCount,
}

impl ProjectOrder {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Id => "id",
            Self::Name => "name",
            Self::Path => "path",
            Self::CreatedAt => "created_at",
            Self::UpdatedAt => "updated_at",
            Self::LastActivityAt => "last_activity_at",
            Self::StarCount => "star_count",
        }
    }
}

/// Sort direction for listings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl SortDirection {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Asc => "asc",
            Self::Desc => "desc",
        }
    }
}

/// Filters for [`ProjectEndpoints::projects`](super::ProjectEndpoints::projects).
/// Empty by default, which lists every project visible to the token
/// (on GitLab.com that is every public project, so narrow it).
#[derive(Debug, Default, Clone)]
pub struct ProjectQuery {
    params: QueryParams,
}

impl ProjectQuery {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Only projects the caller is a member of.
    #[must_use]
    pub fn membership(mut self) -> Self {
        self.params.push_flag("membership", true);
        self
    }

    /// Only projects the caller owns.
    #[must_use]
    pub fn owned(mut self) -> Self {
        self.params.push_flag("owned", true);
        self
    }

    /// Only projects the caller starred.
    #[must_use]
    pub fn starred(mut self) -> Self {
        self.params.push_flag("starred", true);
        self
    }

    /// Match `term` against the project name and path.
    #[must_use]
    pub fn search(mut self, term: &str) -> Self {
        self.params.push("search", term);
        self
    }

    #[must_use]
    pub fn archived(mut self, archived: bool) -> Self {
        self.params.push_flag("archived", archived);
        self
    }

    #[must_use]
    pub fn visibility(mut self, visibility: Visibility) -> Self {
        self.params.push("visibility", visibility.as_str());
        self
    }

    /// Only projects tagged with `topic`.
    #[must_use]
    pub fn topic(mut self, topic: &str) -> Self {
        self.params.push("topic", topic);
        self
    }

    /// Only projects where the caller has at least `level`.
    #[must_use]
    pub fn min_access_level(mut self, level: AccessLevel) -> Self {
        self.params
            .push("min_access_level", level.as_raw().to_string());
        self
    }

    #[must_use]
    pub fn order_by(mut self, order: ProjectOrder, direction: SortDirection) -> Self {
        self.params.push("order_by", order.as_str());
        self.params.push("sort", direction.as_str());
        self
    }

    pub(crate) fn apply(&self, path: String) -> String {
        self.params.apply(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_query_adds_nothing() {
        assert_eq!(ProjectQuery::new().apply("p".into()), "p");
    }

    #[test]
    fn filters_render_in_order() {
        let q = ProjectQuery::new()
            .owned()
            .search("my app")
            .visibility(Visibility::Internal)
            .min_access_level(AccessLevel::Maintainer)
            .order_by(ProjectOrder::LastActivityAt, SortDirection::Desc);
        assert_eq!(
            q.apply("api/v4/projects".into()),
            "api/v4/projects?owned=true&search=my%20app&visibility=internal&min_access_level=40&order_by=last_activity_at&sort=desc"
        );
    }
}
