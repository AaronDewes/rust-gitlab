// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! API types common to many endpoints.
//!
//! Usually these are enumerations or other simple wrappers around structures present in
//! GitLab's REST API.

use std::borrow::Cow;
use std::fmt;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

use crate::api::ParamValue;

/// Access levels for groups and projects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AccessLevel {
    /// Anonymous access.
    Anonymous,
    /// Minimal access.
    Minimal,
    /// Guest access (can see the project).
    Guest,
    /// Reporter access (can open issues).
    Reporter,
    /// Developer access (can push branches, handle issues and merge requests).
    Developer,
    /// Maintainer access (can push to protected branches).
    Maintainer,
    /// Owner access (full rights).
    Owner,
    /// Admin access (full rights).
    Admin,
}

impl AccessLevel {
    /// The string representation of the access level.
    pub fn as_str(self) -> &'static str {
        match self {
            AccessLevel::Admin => "admin",
            AccessLevel::Owner => "owner",
            AccessLevel::Maintainer => "maintainer",
            AccessLevel::Developer => "developer",
            AccessLevel::Reporter => "reporter",
            AccessLevel::Guest => "guest",
            AccessLevel::Minimal => "minimal",
            AccessLevel::Anonymous => "anonymous",
        }
    }

    /// The integer representation of the access level.
    pub fn as_u64(self) -> u64 {
        match self {
            AccessLevel::Admin => 60,
            AccessLevel::Owner => 50,
            AccessLevel::Maintainer => 40,
            AccessLevel::Developer => 30,
            AccessLevel::Reporter => 20,
            AccessLevel::Guest => 10,
            AccessLevel::Minimal => 5,
            AccessLevel::Anonymous => 0,
        }
    }
}

/// Orderings for sorted results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    /// Values should be sorted with "higher" values after "lower" values.
    Ascending,
    /// Values should be sorted with "lower" values after "higher" values.
    Descending,
}

impl Default for SortOrder {
    fn default() -> Self {
        SortOrder::Descending
    }
}

impl SortOrder {
    /// The string representation of the sort order.
    pub fn as_str(self) -> &'static str {
        match self {
            SortOrder::Ascending => "asc",
            SortOrder::Descending => "desc",
        }
    }
}

impl ParamValue<'static> for SortOrder {
    fn as_value(self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// States for features or flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnableState {
    /// The feature or flag is enabled.
    Enabled,
    /// The feature or flag is disabled.
    Disabled,
}

impl EnableState {
    /// The string representation of the enabled state.
    pub fn as_str(self) -> &'static str {
        match self {
            EnableState::Enabled => "enabled",
            EnableState::Disabled => "disabled",
        }
    }
}

impl From<bool> for EnableState {
    fn from(b: bool) -> Self {
        if b {
            EnableState::Enabled
        } else {
            EnableState::Disabled
        }
    }
}

impl ParamValue<'static> for EnableState {
    fn as_value(self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// A strucutre for storing a name or ID where either is allowed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NameOrId<'a> {
    /// The name of the entity.
    ///
    /// Note that numeric names are ambiguous to GitLab. There is nothing done with this crate
    /// which attempts to resolve this ambiguity.
    Name(Cow<'a, str>),
    /// The ID of the entity.
    Id(u64),
}

const PATH_SEGMENT_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'`')
    .add(b'?')
    .add(b'{')
    .add(b'}')
    .add(b'%')
    .add(b'/');

/// Escape a string for usage as a single URL path component.
pub fn path_escaped(input: &'_ str) -> impl fmt::Display + '_ {
    utf8_percent_encode(input, PATH_SEGMENT_ENCODE_SET)
}

impl<'a> fmt::Display for NameOrId<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NameOrId::Name(name) => write!(f, "{}", path_escaped(name)),
            NameOrId::Id(id) => write!(f, "{}", id),
        }
    }
}

impl<'a> From<u64> for NameOrId<'a> {
    fn from(id: u64) -> Self {
        NameOrId::Id(id)
    }
}

impl<'a> From<&'a str> for NameOrId<'a> {
    fn from(name: &'a str) -> Self {
        NameOrId::Name(name.into())
    }
}

impl<'a> From<String> for NameOrId<'a> {
    fn from(name: String) -> Self {
        NameOrId::Name(name.into())
    }
}

/// Visibility levels of projects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisibilityLevel {
    /// The project is visible to anonymous users.
    Public,
    /// The project is visible to logged in users.
    Internal,
    /// The project is visible only to users with explicit access.
    Private,
}

impl VisibilityLevel {
    /// The string representation of the visibility level.
    pub fn as_str(self) -> &'static str {
        match self {
            VisibilityLevel::Public => "public",
            VisibilityLevel::Internal => "internal",
            VisibilityLevel::Private => "private",
        }
    }
}

impl ParamValue<'static> for VisibilityLevel {
    fn as_value(self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// A `yes` or `no`.
///
/// Some endpoints use this terminology.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YesNo {
    /// A `yes` response.
    Yes,
    /// A `no` response.
    No,
}

impl YesNo {
    /// The string representation of the option.
    pub fn as_str(self) -> &'static str {
        match self {
            YesNo::Yes => "yes",
            YesNo::No => "no",
        }
    }
}

impl From<bool> for YesNo {
    fn from(b: bool) -> Self {
        if b {
            YesNo::Yes
        } else {
            YesNo::No
        }
    }
}

impl ParamValue<'static> for YesNo {
    fn as_value(self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Access levels for protected branches and tags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProtectedAccessLevel {
    /// Developers and maintainers may perform the action.
    Developer,
    /// Maintainers may perform the action.
    Maintainer,
    /// Only administrators may perform the action.
    Admin,
    /// The action is not allowed at all.
    NoAccess,
}

impl Default for ProtectedAccessLevel {
    fn default() -> Self {
        ProtectedAccessLevel::Maintainer
    }
}

impl ProtectedAccessLevel {
    fn as_str(self) -> &'static str {
        match self {
            ProtectedAccessLevel::Developer => "30",
            ProtectedAccessLevel::Maintainer => "40",
            ProtectedAccessLevel::Admin => "60",
            ProtectedAccessLevel::NoAccess => "0",
        }
    }
}

impl ParamValue<'static> for ProtectedAccessLevel {
    fn as_value(self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

#[cfg(test)]
mod tests {
    use std::cmp;

    use crate::api::common::{
        AccessLevel, EnableState, NameOrId, ProtectedAccessLevel, SortOrder, VisibilityLevel, YesNo,
    };

    #[test]
    fn access_level_as_str() {
        let items = &[
            (AccessLevel::Anonymous, "anonymous", 0),
            (AccessLevel::Minimal, "minimal", 5),
            (AccessLevel::Guest, "guest", 10),
            (AccessLevel::Reporter, "reporter", 20),
            (AccessLevel::Developer, "developer", 30),
            (AccessLevel::Maintainer, "maintainer", 40),
            (AccessLevel::Owner, "owner", 50),
            (AccessLevel::Admin, "admin", 60),
        ];

        for (i, s, u) in items {
            assert_eq!(i.as_str(), *s);
            assert_eq!(i.as_u64(), *u);
        }
    }

    #[test]
    fn access_level_ordering() {
        let items = &[
            AccessLevel::Anonymous,
            AccessLevel::Guest,
            AccessLevel::Reporter,
            AccessLevel::Developer,
            AccessLevel::Maintainer,
            AccessLevel::Owner,
            AccessLevel::Admin,
        ];

        let mut last = None;
        for item in items {
            if let Some(prev) = last {
                assert!(prev < item);
            }
            last = Some(item);
        }
    }

    #[test]
    fn sort_order_default() {
        assert_eq!(SortOrder::default(), SortOrder::Descending);
    }

    #[test]
    fn sort_order_as_str() {
        let items = &[
            (SortOrder::Ascending, "asc"),
            (SortOrder::Descending, "desc"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn enable_state_as_str() {
        let items = &[
            (EnableState::Enabled, "enabled"),
            (EnableState::Disabled, "disabled"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn enable_state_from_bool() {
        let items = &[(EnableState::Enabled, true), (EnableState::Disabled, false)];

        for (i, s) in items {
            assert_eq!(*i, (*s).into());
        }
    }

    #[test]
    fn name_or_id_as_str() {
        let items: &[(NameOrId, _)] = &[
            ("user".into(), "user"),
            ("special/name".into(), "special%2Fname"),
            (
                "special/name?string".to_string().into(),
                "special%2Fname%3Fstring",
            ),
            (1.into(), "1"),
        ];

        for (i, s) in items {
            assert_eq!(i.to_string(), *s);
        }
    }

    #[test]
    fn visibility_level_as_str() {
        let items = &[
            (VisibilityLevel::Public, "public"),
            (VisibilityLevel::Internal, "internal"),
            (VisibilityLevel::Private, "private"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn yes_no_as_str() {
        let items = &[(YesNo::Yes, "yes"), (YesNo::No, "no")];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn yes_no_from_bool() {
        let items = &[(YesNo::Yes, true), (YesNo::No, false)];

        for (i, s) in items {
            assert_eq!(*i, (*s).into());
        }
    }

    #[test]
    fn protected_access_level_default() {
        assert_eq!(
            ProtectedAccessLevel::default(),
            ProtectedAccessLevel::Maintainer,
        );
    }

    #[test]
    fn protected_access_level_ord() {
        let items = &[
            ProtectedAccessLevel::Developer,
            ProtectedAccessLevel::Maintainer,
            ProtectedAccessLevel::Admin,
            ProtectedAccessLevel::NoAccess,
        ];

        for i in items {
            // We are asserting that `Eq` is implemented.
            #[allow(clippy::eq_op)]
            {
                assert_eq!(*i, *i);
            }
            assert_eq!(i.cmp(i), cmp::Ordering::Equal);

            let mut expect = cmp::Ordering::Greater;
            for j in items {
                let is_same = i == j;
                if is_same {
                    expect = cmp::Ordering::Equal;
                }
                assert_eq!(i.cmp(j), expect);
                if is_same {
                    expect = cmp::Ordering::Less;
                }
            }

            let mut expect = cmp::Ordering::Less;
            for j in items.iter().rev() {
                let is_same = i == j;
                if is_same {
                    expect = cmp::Ordering::Equal;
                }
                assert_eq!(i.cmp(j), expect);
                if is_same {
                    expect = cmp::Ordering::Greater;
                }
            }
        }
    }

    #[test]
    fn protected_access_level_as_str() {
        let items = &[
            (ProtectedAccessLevel::Developer, "30"),
            (ProtectedAccessLevel::Maintainer, "40"),
            (ProtectedAccessLevel::Admin, "60"),
            (ProtectedAccessLevel::NoAccess, "0"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }
}
