//! Wire shapes for the **Issues Statistics** API category — aggregate open/
//! closed counts without paging every issue. The cheap backbone of burndown
//! and health summaries.

use json_bourne::{FromJson, ToJson};

/// The open/closed counts inside a statistics response.
#[derive(Debug, FromJson, ToJson, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct IssueCounts {
    #[bourne(default)]
    pub all: i64,
    #[bourne(default)]
    pub closed: i64,
    #[bourne(default)]
    pub opened: i64,
}

/// The `statistics.counts` envelope GitLab wraps the counts in.
#[derive(Debug, FromJson, ToJson, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct IssueStatisticsInner {
    #[bourne(default)]
    pub counts: IssueCounts,
}

/// Top-level response of the issues-statistics endpoints.
#[derive(Debug, FromJson, ToJson, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[bourne(deny_unknown_fields = false)]
pub struct IssueStatistics {
    #[bourne(default)]
    pub statistics: IssueStatisticsInner,
}

impl IssueStatistics {
    /// The open/closed/all counts, unwrapped from the nesting.
    #[must_use]
    pub fn counts(&self) -> &IssueCounts {
        &self.statistics.counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::parse_str;

    #[test]
    fn parse_statistics() {
        let json = r#"{
            "statistics": {
                "counts": { "all": 30, "closed": 22, "opened": 8 }
            }
        }"#;
        let s: IssueStatistics = parse_str(json).unwrap();
        assert_eq!(s.counts().all, 30);
        assert_eq!(s.counts().opened, 8);
    }
}
