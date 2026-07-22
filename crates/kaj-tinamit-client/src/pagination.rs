//! Pagination configuration.

/// Configuration for pagination behavior.
#[derive(Debug, Clone)]
pub struct PaginationConfig {
    /// Number of items per page (default: 100, GitLab max).
    pub per_page: usize,
    /// Maximum number of pages to fetch (None = unlimited).
    pub max_pages: Option<usize>,
}

impl Default for PaginationConfig {
    fn default() -> Self {
        Self {
            per_page: 100,
            max_pages: None,
        }
    }
}

impl PaginationConfig {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the per-page count, capped at GitLab's maximum of 100.
    #[must_use]
    pub fn with_per_page(mut self, per_page: usize) -> Self {
        self.per_page = per_page.min(100);
        self
    }

    #[must_use]
    pub fn with_max_pages(mut self, max_pages: usize) -> Self {
        self.max_pages = Some(max_pages);
        self
    }
}
