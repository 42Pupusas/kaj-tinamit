//! Instance read endpoints: the **Metadata** endpoint and the
//! **Namespaces** listing.
//!
//! All read-only (GET).

use gitlab_model::{Metadata, NamespaceListing};

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

/// Read endpoints for instance metadata and namespaces.
pub trait MetadataEndpoints {
    /// Retrieve instance metadata (version, revision, enterprise flag, KAS).
    ///
    /// A cheap version probe that needs no admin rights.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn metadata(&self) -> Result<Metadata, Error>;

    /// List namespaces the authenticated user can see.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn namespaces(&self) -> Result<Vec<NamespaceListing>, Error>;

    /// Search namespaces by name or path.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn search_namespaces(&self, search: &str) -> Result<Vec<NamespaceListing>, Error>;

    /// Retrieve a single namespace by ID or URL-encoded path.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn namespace(&self, id_or_path: &str) -> Result<NamespaceListing, Error>;
}

impl MetadataEndpoints for GitlabClient {
    fn metadata(&self) -> Result<Metadata, Error> {
        self.get("api/v4/metadata")
    }

    fn namespaces(&self) -> Result<Vec<NamespaceListing>, Error> {
        self.get_paginated("api/v4/namespaces")
    }

    fn search_namespaces(&self, search: &str) -> Result<Vec<NamespaceListing>, Error> {
        self.get_paginated(&format!(
            "api/v4/namespaces?search={}",
            search.percent_encode()
        ))
    }

    fn namespace(&self, id_or_path: &str) -> Result<NamespaceListing, Error> {
        self.get(&format!(
            "api/v4/namespaces/{}",
            id_or_path.percent_encode()
        ))
    }
}
