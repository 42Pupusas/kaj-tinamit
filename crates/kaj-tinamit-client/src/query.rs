//! Accumulates URL query parameters for the filter builders and renders
//! them with percent-encoded values.

use crate::encode::PercentEncode;

#[derive(Debug, Default, Clone)]
pub(crate) struct QueryParams {
    params: Vec<(&'static str, String)>,
}

impl QueryParams {
    pub(crate) fn push(&mut self, key: &'static str, value: impl Into<String>) {
        self.params.push((key, value.into()));
    }

    pub(crate) fn push_flag(&mut self, key: &'static str, value: bool) {
        self.push(key, if value { "true" } else { "false" });
    }

    /// `""` when empty, otherwise `"?k=v&…"`.
    pub(crate) fn to_suffix(&self) -> String {
        if self.params.is_empty() {
            return String::new();
        }
        let joined = self
            .params
            .iter()
            .map(|(k, v)| format!("{k}={}", v.percent_encode()))
            .collect::<Vec<_>>()
            .join("&");
        format!("?{joined}")
    }

    /// Append to a path that has no query string yet.
    pub(crate) fn apply(&self, path: String) -> String {
        path + &self.to_suffix()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_renders_nothing() {
        assert_eq!(QueryParams::default().to_suffix(), "");
    }

    #[test]
    fn values_are_encoded_and_keys_kept() {
        let mut q = QueryParams::default();
        q.push("search", "a b/c");
        q.push_flag("owned", true);
        assert_eq!(q.to_suffix(), "?search=a%20b%2Fc&owned=true");
    }

    #[test]
    fn repeated_keys_are_kept_in_order() {
        let mut q = QueryParams::default();
        q.push("scope%5B%5D", "failed");
        q.push("scope%5B%5D", "running");
        assert_eq!(
            q.apply("api/v4/x".to_string()),
            "api/v4/x?scope%5B%5D=failed&scope%5B%5D=running"
        );
    }
}
