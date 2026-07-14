//! Property-based round-trip tests for the wire models.
//!
//! Every model derives both `ToJson` and `FromJson`. The core invariant is
//! that serializing a value and parsing it back yields an equal value:
//!
//! ```text
//! parse(to_string(x)) == x
//! ```
//!
//! This catches whole classes of bugs example tests miss: `#[bourne(rename)]`
//! mismatches, `Option`/`default` asymmetry between the two derives, and enum
//! casing errors — across every field at once.
//!
//! Only float-free, `PartialEq` models are covered here; types with `f64`
//! fields (CI durations/coverage) are excluded because JSON round-tripping of
//! arbitrary floats is a separate concern already tested in `json-bourne`.

use gitlab_model::{AccessLevel, GitlabUser, Label, Member, MemberCreatedBy, UserState};
use json_bourne::{parse_str, to_string};
use proptest::prelude::*;

/// A strategy for the small set of strings GitLab returns — including ones
/// with characters that must be JSON-escaped, to exercise the string codec.
fn any_string() -> impl Strategy<Value = String> {
    prop_oneof![
        "[a-zA-Z0-9_./ -]{0,40}",
        Just("with \"quotes\" and \\backslash".to_string()),
        Just("newline\nand\ttab".to_string()),
        Just("unicode: café ☃ 日本語".to_string()),
        Just(String::new()),
    ]
}

fn opt_string() -> impl Strategy<Value = Option<String>> {
    proptest::option::of(any_string())
}

prop_compose! {
    fn user_state()(idx in 0usize..4) -> UserState {
        [
            UserState::Active,
            UserState::Blocked,
            UserState::Deactivated,
            UserState::BlockedPendingApproval,
        ][idx]
    }
}

prop_compose! {
    fn arb_user()(
        id in any::<i32>(),
        username in any_string(),
        name in any_string(),
        state in user_state(),
        locked in any::<bool>(),
        avatar_url in any_string(),
        web_url in any_string(),
    ) -> GitlabUser {
        GitlabUser { id, username, name, state, locked, avatar_url, web_url }
    }
}

prop_compose! {
    fn arb_label()(
        id in any::<i64>(),
        name in any_string(),
        color in any_string(),
        text_color in opt_string(),
        description in opt_string(),
        description_html in opt_string(),
        open_issues_count in any::<i64>(),
        closed_issues_count in any::<i64>(),
        open_merge_requests_count in any::<i64>(),
        subscribed in any::<bool>(),
        priority in proptest::option::of(any::<i64>()),
        is_project_label in proptest::option::of(any::<bool>()),
        archived in any::<bool>(),
    ) -> Label {
        Label {
            id, name, color, text_color, description, description_html,
            open_issues_count, closed_issues_count, open_merge_requests_count,
            subscribed, priority, is_project_label, archived,
        }
    }
}

prop_compose! {
    fn arb_created_by()(
        id in any::<i64>(),
        username in opt_string(),
        name in opt_string(),
        web_url in opt_string(),
    ) -> MemberCreatedBy {
        MemberCreatedBy { id, username, name, web_url }
    }
}

prop_compose! {
    fn arb_member()(
        id in any::<i64>(),
        username in any_string(),
        name in opt_string(),
        state in opt_string(),
        avatar_url in opt_string(),
        web_url in opt_string(),
        access_level in any::<i32>(),
        expires_at in opt_string(),
        created_at in opt_string(),
        created_by in proptest::option::of(arb_created_by()),
        email in opt_string(),
        membership_state in opt_string(),
    ) -> Member {
        Member {
            id, username, name, state, avatar_url, web_url, access_level,
            expires_at, created_at, created_by, email, membership_state,
        }
    }
}

proptest! {
    #[test]
    fn user_round_trips(u in arb_user()) {
        let json = to_string(&u).expect("serialize user");
        let back: GitlabUser = parse_str(&json).expect("parse user");
        prop_assert_eq!(back, u);
    }

    #[test]
    fn label_round_trips(l in arb_label()) {
        let json = to_string(&l).expect("serialize label");
        let back: Label = parse_str(&json).expect("parse label");
        prop_assert_eq!(back, l);
    }

    #[test]
    fn member_round_trips(m in arb_member()) {
        let json = to_string(&m).expect("serialize member");
        let back: Member = parse_str(&json).expect("parse member");
        prop_assert_eq!(back, m);
    }

    /// The numeric <-> named access-level mapping is total and invertible
    /// for every i32.
    #[test]
    fn access_level_is_invertible(raw in any::<i32>()) {
        prop_assert_eq!(AccessLevel::from_raw(raw).as_raw(), raw);
    }
}
