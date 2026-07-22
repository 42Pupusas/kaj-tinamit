//! A newtype for GitLab object identifiers.
//!
//! Every GitLab object (project, issue, user, pipeline, …) is keyed by a
//! server-assigned integer id. GitLab.com's ids passed 2³¹ long ago, so the
//! underlying type is [`i64`] — never `i32`. Wrapping them in [`Id`] instead
//! of a bare integer keeps ids from being confused with counts, offsets, or
//! each other, and gives one place to widen or change representation later.
//!
//! # Wire format
//!
//! [`Id`] is *transparent*: it (de)serializes as a bare JSON number, exactly
//! like the `i64` it wraps. Parsing an id is byte-for-byte identical to
//! parsing an integer, so there is no size or allocation cost over the raw
//! field.
//!
//! ```
//! use kaj_tinamit::Id;
//! use json_bourne::{parse_str, to_string};
//!
//! let id: Id = parse_str("42").unwrap();
//! assert_eq!(id, Id::new(42));
//! assert_eq!(id.get(), 42);
//! assert_eq!(to_string(&id).unwrap(), "42");
//! ```

use core::fmt;
use json_bourne::{FromJson, JsonWrite, Lexer, ToJson};

/// A GitLab object identifier — a transparent `i64` newtype.
///
/// Construct with [`Id::new`] or `Id::from(i64)`; read with [`Id::get`] or
/// the `i64: From<Id>` conversion. Compares directly against `i64` for
/// ergonomic assertions (`assert_eq!(user.id, 42)`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Id(i64);

impl Id {
    /// Wrap a raw `i64` id.
    #[inline]
    pub const fn new(value: i64) -> Self {
        Self(value)
    }

    /// The raw `i64` value.
    #[inline]
    pub const fn get(self) -> i64 {
        self.0
    }
}

impl fmt::Display for Id {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl From<i64> for Id {
    #[inline]
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<Id> for i64 {
    #[inline]
    fn from(id: Id) -> Self {
        id.0
    }
}

impl PartialEq<i64> for Id {
    #[inline]
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Id> for i64 {
    #[inline]
    fn eq(&self, other: &Id) -> bool {
        *self == other.0
    }
}

impl<'input> FromJson<'input> for Id {
    #[inline]
    fn from_lex(lex: &mut Lexer<'input>) -> Result<Self, json_bourne::Error> {
        i64::from_lex(lex).map(Self)
    }
}

impl ToJson for Id {
    #[inline]
    fn write_json<W: JsonWrite + ?Sized>(&self, w: &mut W) -> Result<(), W::Error> {
        self.0.write_json(w)
    }
}

#[cfg(feature = "proptest")]
impl proptest::arbitrary::Arbitrary for Id {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::strategy::Strategy;
        proptest::arbitrary::any::<i64>().prop_map(Id).boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use json_bourne::{parse_str, to_string};

    #[test]
    fn round_trips_as_bare_integer() {
        let id: Id = parse_str("9007199254740993").unwrap();
        assert_eq!(id, Id::new(9_007_199_254_740_993));
        assert_eq!(to_string(&id).unwrap(), "9007199254740993");
    }

    #[test]
    fn compares_against_i64() {
        assert_eq!(Id::new(42), 42);
        assert_eq!(42, Id::new(42));
    }

    #[test]
    fn conversions() {
        let id = Id::from(7i64);
        assert_eq!(id.get(), 7);
        assert_eq!(i64::from(id), 7);
    }
}
