//! Wire shapes for the **Releases** API category.

use json_bourne::{FromJson, ToJson};

/// The author of a release (concise user subset).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct ReleaseAuthor {
    pub id: i64,
    pub name: Option<String>,
    pub username: Option<String>,
    pub state: Option<String>,
    pub avatar_url: Option<String>,
    pub web_url: Option<String>,
}

/// The commit a release points at (concise subset).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct ReleaseCommit {
    pub id: String,
    pub short_id: Option<String>,
    pub title: Option<String>,
    pub message: Option<String>,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub created_at: Option<String>,
    #[bourne(default)]
    pub parent_ids: Vec<String>,
}

/// Issue statistics embedded in a release milestone.
#[derive(Debug, FromJson, ToJson, Clone, Default)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct MilestoneIssueStats {
    #[bourne(default)]
    pub total: i64,
    #[bourne(default)]
    pub closed: i64,
    #[bourne(default)]
    pub opened: i64,
}

/// A milestone associated with a release. Distinct from the issue
/// [`Milestone`](crate::issue::Milestone) because releases embed
/// `issue_stats` and `start_date`.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct ReleaseMilestone {
    pub id: i64,
    #[bourne(default)]
    pub iid: i64,
    pub project_id: Option<i64>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub state: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub due_date: Option<String>,
    pub start_date: Option<String>,
    pub web_url: Option<String>,
    pub issue_stats: Option<MilestoneIssueStats>,
}

/// A downloadable source archive of a release.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct ReleaseSource {
    pub format: String,
    pub url: String,
}

/// A user-defined asset link on a release.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct ReleaseLink {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub direct_asset_url: Option<String>,
    pub link_type: Option<String>,
}

/// The assets (sources + links) attached to a release.
#[derive(Debug, FromJson, ToJson, Clone, Default)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct ReleaseAssets {
    #[bourne(default)]
    pub count: i64,
    #[bourne(default)]
    pub sources: Vec<ReleaseSource>,
    #[bourne(default)]
    pub links: Vec<ReleaseLink>,
}

/// A piece of collected release evidence.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct ReleaseEvidence {
    pub sha: Option<String>,
    pub filepath: Option<String>,
    pub collected_at: Option<String>,
}

/// A project release (`GET /projects/:id/releases[/:tag_name]`).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct Release {
    pub tag_name: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub released_at: Option<String>,
    pub author: Option<ReleaseAuthor>,
    pub commit: Option<ReleaseCommit>,
    #[bourne(default)]
    pub milestones: Vec<ReleaseMilestone>,
    pub commit_path: Option<String>,
    pub tag_path: Option<String>,
    #[bourne(default)]
    pub assets: ReleaseAssets,
    #[bourne(default)]
    pub evidences: Vec<ReleaseEvidence>,
    /// Present and `true` when the release is scheduled for a future date.
    pub upcoming_release: Option<bool>,
    /// Present and `true` when the release date is in the past.
    pub historical_release: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_release() {
        let json = r###"{
            "tag_name": "v0.1",
            "name": "Awesome app v0.1 alpha",
            "description": "## CHANGELOG\r\n- stuff",
            "created_at": "2019-01-03T01:55:18.203Z",
            "released_at": "2019-01-03T01:55:18.203Z",
            "author": {"id": 1, "name": "Administrator", "username": "root", "state": "active"},
            "commit": {"id": "f8d3d94c", "short_id": "f8d3d94c", "title": "Initial commit"},
            "assets": {
                "count": 4,
                "sources": [{"format": "zip", "url": "https://example.com/x.zip"}],
                "links": []
            }
        }"###;
        let r: Release = parse_str(json).unwrap();
        assert_eq!(r.tag_name, "v0.1");
        assert_eq!(r.assets.count, 4);
        assert_eq!(r.assets.sources.len(), 1);
        assert_eq!(r.commit.unwrap().short_id.as_deref(), Some("f8d3d94c"));
    }

    #[test]
    fn parse_release_with_milestone_stats() {
        let json = r#"{
            "tag_name": "v0.2",
            "milestones": [{
                "id": 51, "iid": 1, "project_id": 24, "title": "v1.0-rc", "state": "closed",
                "issue_stats": {"total": 98, "closed": 76}
            }]
        }"#;
        let r: Release = parse_str(json).unwrap();
        assert_eq!(r.milestones.len(), 1);
        let stats = r.milestones[0].issue_stats.as_ref().unwrap();
        assert_eq!(stats.total, 98);
        assert_eq!(stats.closed, 76);
    }
}
