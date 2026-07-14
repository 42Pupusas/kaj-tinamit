//! Group and membership endpoints: the **Groups** and **Members** API
//! categories (group and project members, including inherited).
//!
//! Group reads list/fetch groups and their projects; member reads list
//! direct/inherited members. Writes add, update, and remove members on
//! both groups and projects.

use gitlab_model::{AccessLevel, GitlabGroup, GitlabProject, Member};
use json_bourne::ToJson;

use crate::client::GitlabClient;
use crate::encode::PercentEncode;
use crate::error::Error;

/// Body for adding a member (`POST .../members`).
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
struct AddMemberBody {
    user_id: i64,
    access_level: i32,
    #[bourne(skip_if_none)]
    expires_at: Option<String>,
}

/// Body for updating a member's access (`PUT .../members/:id`).
#[derive(Debug, Clone, ToJson)]
#[bourne(deny_unknown_fields = false)]
struct UpdateMemberBody {
    access_level: i32,
    #[bourne(skip_if_none)]
    expires_at: Option<String>,
}

/// Read endpoints for groups.
pub trait GroupEndpoints {
    /// List all groups visible to the authenticated user.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn groups(&self) -> Result<Vec<GitlabGroup>, Error>;

    /// Search groups by name or path.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn search_groups(&self, search: &str) -> Result<Vec<GitlabGroup>, Error>;

    /// Retrieve a single group by ID or URL-encoded path.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group(&self, id: i64) -> Result<GitlabGroup, Error>;

    /// List a group's direct subgroups.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn subgroups(&self, id: i64) -> Result<Vec<GitlabGroup>, Error>;

    /// List all descendant groups (subgroups at any depth).
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn descendant_groups(&self, id: i64) -> Result<Vec<GitlabGroup>, Error>;

    /// List the projects belonging to a group.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_projects(&self, id: i64) -> Result<Vec<GitlabProject>, Error>;

    /// List the projects shared with a group.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_shared_projects(&self, id: i64) -> Result<Vec<GitlabProject>, Error>;
}

impl GroupEndpoints for GitlabClient {
    fn groups(&self) -> Result<Vec<GitlabGroup>, Error> {
        self.get_paginated("api/v4/groups")
    }

    fn search_groups(&self, search: &str) -> Result<Vec<GitlabGroup>, Error> {
        self.get_paginated(&format!("api/v4/groups?search={}", search.percent_encode()))
    }

    fn group(&self, id: i64) -> Result<GitlabGroup, Error> {
        self.get(&format!("api/v4/groups/{id}"))
    }

    fn subgroups(&self, id: i64) -> Result<Vec<GitlabGroup>, Error> {
        self.get_paginated(&format!("api/v4/groups/{id}/subgroups"))
    }

    fn descendant_groups(&self, id: i64) -> Result<Vec<GitlabGroup>, Error> {
        self.get_paginated(&format!("api/v4/groups/{id}/descendant_groups"))
    }

    fn group_projects(&self, id: i64) -> Result<Vec<GitlabProject>, Error> {
        self.get_paginated(&format!("api/v4/groups/{id}/projects"))
    }

    fn group_shared_projects(&self, id: i64) -> Result<Vec<GitlabProject>, Error> {
        self.get_paginated(&format!("api/v4/groups/{id}/projects/shared"))
    }
}

/// Read endpoints for group and project members.
///
/// The `*_direct` methods list only members assigned to the group/project
/// itself; the plain listing methods (`/all`) include members inherited
/// from ancestor groups and invited groups.
pub trait MemberEndpoints {
    /// List a group's direct members.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_members(&self, group_id: i64) -> Result<Vec<Member>, Error>;

    /// List a group's members including inherited and invited members.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_members_all(&self, group_id: i64) -> Result<Vec<Member>, Error>;

    /// Retrieve a single direct group member by user ID.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_member(&self, group_id: i64, user_id: i64) -> Result<Member, Error>;

    /// Retrieve a single group member, including inherited/invited.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn group_member_all(&self, group_id: i64, user_id: i64) -> Result<Member, Error>;

    /// List a project's direct members.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_members(&self, project_id: i64) -> Result<Vec<Member>, Error>;

    /// List a project's members including inherited and invited members.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_members_all(&self, project_id: i64) -> Result<Vec<Member>, Error>;

    /// Retrieve a single direct project member by user ID.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_member(&self, project_id: i64, user_id: i64) -> Result<Member, Error>;

    /// Retrieve a single project member, including inherited/invited.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn project_member_all(&self, project_id: i64, user_id: i64) -> Result<Member, Error>;

    // --- Writes ---

    /// Add a user to a group at `access_level`. Returns the new member.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn add_group_member(
        &self,
        group_id: i64,
        user_id: i64,
        access_level: AccessLevel,
        expires_at: Option<&str>,
    ) -> Result<Member, Error>;

    /// Update a group member's `access_level`. Returns the member.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn update_group_member(
        &self,
        group_id: i64,
        user_id: i64,
        access_level: AccessLevel,
        expires_at: Option<&str>,
    ) -> Result<Member, Error>;

    /// Remove a member from a group.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn remove_group_member(&self, group_id: i64, user_id: i64) -> Result<(), Error>;

    /// Add a user to a project at `access_level`. Returns the new member.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn add_project_member(
        &self,
        project_id: i64,
        user_id: i64,
        access_level: AccessLevel,
        expires_at: Option<&str>,
    ) -> Result<Member, Error>;

    /// Update a project member's `access_level`. Returns the member.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn update_project_member(
        &self,
        project_id: i64,
        user_id: i64,
        access_level: AccessLevel,
        expires_at: Option<&str>,
    ) -> Result<Member, Error>;

    /// Remove a member from a project.
    ///
    /// # Errors
    ///
    /// Propagates transport, API-status, and JSON errors.
    fn remove_project_member(&self, project_id: i64, user_id: i64) -> Result<(), Error>;
}

impl MemberEndpoints for GitlabClient {
    fn group_members(&self, group_id: i64) -> Result<Vec<Member>, Error> {
        self.get_paginated(&format!("api/v4/groups/{group_id}/members"))
    }

    fn group_members_all(&self, group_id: i64) -> Result<Vec<Member>, Error> {
        self.get_paginated(&format!("api/v4/groups/{group_id}/members/all"))
    }

    fn group_member(&self, group_id: i64, user_id: i64) -> Result<Member, Error> {
        self.get(&format!("api/v4/groups/{group_id}/members/{user_id}"))
    }

    fn group_member_all(&self, group_id: i64, user_id: i64) -> Result<Member, Error> {
        self.get(&format!("api/v4/groups/{group_id}/members/all/{user_id}"))
    }

    fn project_members(&self, project_id: i64) -> Result<Vec<Member>, Error> {
        self.get_paginated(&format!("api/v4/projects/{project_id}/members"))
    }

    fn project_members_all(&self, project_id: i64) -> Result<Vec<Member>, Error> {
        self.get_paginated(&format!("api/v4/projects/{project_id}/members/all"))
    }

    fn project_member(&self, project_id: i64, user_id: i64) -> Result<Member, Error> {
        self.get(&format!("api/v4/projects/{project_id}/members/{user_id}"))
    }

    fn project_member_all(&self, project_id: i64, user_id: i64) -> Result<Member, Error> {
        self.get(&format!(
            "api/v4/projects/{project_id}/members/all/{user_id}"
        ))
    }

    fn add_group_member(
        &self,
        group_id: i64,
        user_id: i64,
        access_level: AccessLevel,
        expires_at: Option<&str>,
    ) -> Result<Member, Error> {
        self.post(
            &format!("api/v4/groups/{group_id}/members"),
            &AddMemberBody {
                user_id,
                access_level: access_level.as_raw(),
                expires_at: expires_at.map(str::to_string),
            },
        )
    }

    fn update_group_member(
        &self,
        group_id: i64,
        user_id: i64,
        access_level: AccessLevel,
        expires_at: Option<&str>,
    ) -> Result<Member, Error> {
        self.put(
            &format!("api/v4/groups/{group_id}/members/{user_id}"),
            &UpdateMemberBody {
                access_level: access_level.as_raw(),
                expires_at: expires_at.map(str::to_string),
            },
        )
    }

    fn remove_group_member(&self, group_id: i64, user_id: i64) -> Result<(), Error> {
        self.delete(&format!("api/v4/groups/{group_id}/members/{user_id}"))
    }

    fn add_project_member(
        &self,
        project_id: i64,
        user_id: i64,
        access_level: AccessLevel,
        expires_at: Option<&str>,
    ) -> Result<Member, Error> {
        self.post(
            &format!("api/v4/projects/{project_id}/members"),
            &AddMemberBody {
                user_id,
                access_level: access_level.as_raw(),
                expires_at: expires_at.map(str::to_string),
            },
        )
    }

    fn update_project_member(
        &self,
        project_id: i64,
        user_id: i64,
        access_level: AccessLevel,
        expires_at: Option<&str>,
    ) -> Result<Member, Error> {
        self.put(
            &format!("api/v4/projects/{project_id}/members/{user_id}"),
            &UpdateMemberBody {
                access_level: access_level.as_raw(),
                expires_at: expires_at.map(str::to_string),
            },
        )
    }

    fn remove_project_member(&self, project_id: i64, user_id: i64) -> Result<(), Error> {
        self.delete(&format!("api/v4/projects/{project_id}/members/{user_id}"))
    }
}
