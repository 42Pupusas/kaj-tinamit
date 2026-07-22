//! Wire shapes for the **Pipeline schedules** API category
//! (`GET /projects/:id/pipeline_schedules`).

use json_bourne::{FromJson, ToJson};

use crate::Id;
use crate::ci::CiUser;

/// The most recent pipeline a schedule triggered (concise subset).
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct SchedulePipeline {
    pub id: Id,
    pub sha: Option<String>,
    #[bourne(rename = "ref")]
    pub ref_name: Option<String>,
    pub status: Option<String>,
    pub web_url: Option<String>,
}

/// A CI/CD pipeline schedule.
#[derive(Debug, FromJson, ToJson, Clone)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
#[non_exhaustive]
pub struct PipelineSchedule {
    pub id: Id,
    pub description: Option<String>,
    #[bourne(rename = "ref")]
    pub ref_name: Option<String>,
    /// The cron expression driving the schedule.
    pub cron: Option<String>,
    pub cron_timezone: Option<String>,
    pub next_run_at: Option<String>,
    #[bourne(default)]
    pub active: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub owner: Option<CiUser>,
    /// Present on the single-schedule endpoint; absent from the list.
    pub last_pipeline: Option<SchedulePipeline>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_pipeline_schedule() {
        let json = r#"{
            "id": 13,
            "description": "Test schedule pipeline",
            "ref": "refs/heads/main",
            "cron": "* * * * *",
            "cron_timezone": "Asia/Tokyo",
            "next_run_at": "2017-05-19T13:41:00.000Z",
            "active": true
        }"#;
        let s: PipelineSchedule = parse_str(json).unwrap();
        assert_eq!(s.id, 13);
        assert_eq!(s.cron.as_deref(), Some("* * * * *"));
        assert!(s.active);
    }
}
