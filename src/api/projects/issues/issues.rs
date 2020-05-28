// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeSet;

use chrono::{DateTime, Utc};
use derive_builder::Builder;
use itertools::Itertools;

use crate::api::common::{NameOrId, SortOrder};
use crate::api::endpoint_prelude::*;
use crate::api::ParamValue;

/// Filters for issue states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueState {
    /// Filter issues that are open.
    Opened,
    /// Filter issues that are closed.
    Closed,
}

impl IssueState {
    fn as_str(self) -> &'static str {
        match self {
            IssueState::Opened => "opened",
            IssueState::Closed => "closed",
        }
    }
}

impl ParamValue<'static> for IssueState {
    fn as_value(self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

#[derive(Debug, Clone)]
enum Labels<'a> {
    Any,
    None,
    AllOf(BTreeSet<Cow<'a, str>>),
}

impl<'a> Labels<'a> {
    fn as_str(&self) -> Cow<'static, str> {
        match self {
            Labels::Any => "Any".into(),
            Labels::None => "None".into(),
            Labels::AllOf(labels) => format!("{}", labels.iter().format(",")).into(),
        }
    }
}

impl<'a, 'b: 'a> ParamValue<'static> for &'b Labels<'a> {
    fn as_value(self) -> Cow<'static, str> {
        self.as_str()
    }
}

#[derive(Debug, Clone)]
enum Milestone<'a> {
    None,
    Any,
    Named(Cow<'a, str>),
}

impl<'a> Milestone<'a> {
    fn as_str(&self) -> &str {
        match self {
            Milestone::None => "None",
            Milestone::Any => "Any",
            Milestone::Named(name) => name.as_ref(),
        }
    }
}

impl<'a, 'b: 'a> ParamValue<'a> for &'b Milestone<'a> {
    fn as_value(self) -> Cow<'a, str> {
        self.as_str().into()
    }
}

/// Filter issues by a scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueScope {
    /// Filter issues created by the API caller.
    CreatedByMe,
    /// Filter issues assigned to the API caller.
    AssignedToMe,
    /// Return all issues.
    All,
}

impl IssueScope {
    fn as_str(self) -> &'static str {
        match self {
            IssueScope::CreatedByMe => "created_by_me",
            IssueScope::AssignedToMe => "assigned_to_me",
            IssueScope::All => "all",
        }
    }
}

impl ParamValue<'static> for IssueScope {
    fn as_value(self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

#[derive(Debug, Clone)]
enum Assignee<'a> {
    Assigned,
    Unassigned,
    Id(u64),
    Usernames(BTreeSet<Cow<'a, str>>),
}

impl<'a> Assignee<'a> {
    fn add_params<'b>(&'b self, params: &mut QueryParams<'b>) {
        match self {
            Assignee::Assigned => {
                params.push("assignee_id", "Any");
            },
            Assignee::Unassigned => {
                params.push("assignee_id", "None");
            },
            Assignee::Id(id) => {
                params.push("assignee_id", *id);
            },
            Assignee::Usernames(usernames) => {
                params.extend(usernames.iter().map(|value| ("assignee_username[]", value)));
            },
        }
    }
}

#[derive(Debug, Clone)]
enum ReactionEmoji<'a> {
    None,
    Any,
    Emoji(Cow<'a, str>),
}

impl<'a> ReactionEmoji<'a> {
    fn as_str(&self) -> &str {
        match self {
            ReactionEmoji::None => "None",
            ReactionEmoji::Any => "Any",
            ReactionEmoji::Emoji(name) => name.as_ref(),
        }
    }
}

impl<'a, 'b: 'a> ParamValue<'a> for &'b ReactionEmoji<'a> {
    fn as_value(self) -> Cow<'a, str> {
        self.as_str().into()
    }
}

/// Filter issues by weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueWeight {
    /// Filter issues with any weight.
    Any,
    /// Filter issues with no weight assigned.
    None,
    /// Filter issues with a specific weight.
    Weight(u64),
}

impl IssueWeight {
    fn as_str(self) -> Cow<'static, str> {
        match self {
            IssueWeight::Any => "Any".into(),
            IssueWeight::None => "None".into(),
            IssueWeight::Weight(weight) => format!("{}", weight).into(),
        }
    }
}

impl ParamValue<'static> for IssueWeight {
    fn as_value(self) -> Cow<'static, str> {
        self.as_str()
    }
}

/// Keys issue results may be ordered by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueOrderBy {
    /// Sort by creation date.
    CreatedAt,
    /// Sort by last updated date.
    UpdatedAt,
    /// Sort by priority.
    Priority,
    /// Sort by due date.
    DueDate,
    /// Sort by relative position.
    ///
    /// TOOD: position within what?
    RelativePosition,
    /// Sort by priority labels.
    LabelPriority,
    /// Sort by milestone due date.
    MilestoneDue,
    /// Sort by popularity.
    Popularity,
    /// Sort by weight.
    WeightFields,
}

impl Default for IssueOrderBy {
    fn default() -> Self {
        IssueOrderBy::CreatedAt
    }
}

impl IssueOrderBy {
    fn as_str(self) -> &'static str {
        match self {
            IssueOrderBy::CreatedAt => "created_at",
            IssueOrderBy::UpdatedAt => "updated_at",
            IssueOrderBy::Priority => "priority",
            IssueOrderBy::DueDate => "due_date",
            IssueOrderBy::RelativePosition => "relative_position",
            IssueOrderBy::LabelPriority => "label_priority",
            IssueOrderBy::MilestoneDue => "milestone_due",
            IssueOrderBy::Popularity => "popularity",
            IssueOrderBy::WeightFields => "weight_fields",
        }
    }
}

impl ParamValue<'static> for IssueOrderBy {
    fn as_value(self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

/// Query for issues within a project.
///
/// TODO: Negation (not) filters are not yet supported.
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct Issues<'a> {
    /// The project to query for issues.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// Filter issues with specific internal IDs.
    #[builder(setter(name = "_iids"), default, private)]
    iids: BTreeSet<u64>,
    /// Filter issues based on state.
    #[builder(default)]
    state: Option<IssueState>,
    /// Filter issues based on labels.
    #[builder(setter(name = "_labels"), default, private)]
    labels: Option<Labels<'a>>,
    /// Include label details in the result.
    #[builder(default)]
    with_labels_details: Option<bool>,
    /// Filter issues with a milestone.
    #[builder(setter(name = "_milestone"), default, private)]
    milestone: Option<Milestone<'a>>,
    /// Filter issues within a scope.
    #[builder(default)]
    scope: Option<IssueScope>,
    /// Filter issues by author.
    #[builder(setter(into), default)]
    author: Option<NameOrId<'a>>,
    /// Filter issues by assignees.
    #[builder(setter(name = "_assignee"), default, private)]
    assignee: Option<Assignee<'a>>,
    /// Filter issues by the API caller's reactions.
    #[builder(setter(name = "_my_reaction_emoji"), default, private)]
    my_reaction_emoji: Option<ReactionEmoji<'a>>,
    /// Filter issues by weight.
    #[builder(default)]
    weight: Option<IssueWeight>,

    /// Filter issues with a search query.
    #[builder(setter(into), default)]
    search: Option<Cow<'a, str>>,
    /// Filter issues created after a point in time.
    #[builder(default)]
    created_after: Option<DateTime<Utc>>,
    /// Filter issues created before a point in time.
    #[builder(default)]
    created_before: Option<DateTime<Utc>>,
    /// Filter issues last updated after a point in time.
    #[builder(default)]
    updated_after: Option<DateTime<Utc>>,
    /// Filter issues last updated before a point in time.
    #[builder(default)]
    updated_before: Option<DateTime<Utc>>,
    /// Filter issues by confidentiality.
    #[builder(default)]
    confidential: Option<bool>,

    // TODO: How best to support this parameter?
    // not
    /// Order results by a given key.
    #[builder(default)]
    order_by: Option<IssueOrderBy>,
    /// The sort order for return results.
    #[builder(default)]
    sort: Option<SortOrder>,
}

impl<'a> Issues<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> IssuesBuilder<'a> {
        IssuesBuilder::default()
    }
}

impl<'a> IssuesBuilder<'a> {
    /// Return an issue with an internal ID.
    pub fn iid(&mut self, iid: u64) -> &mut Self {
        self.iids.get_or_insert_with(BTreeSet::new).insert(iid);
        self
    }

    /// Return issues with one of a set of internal IDs.
    pub fn iids<I>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = u64>,
    {
        self.iids.get_or_insert_with(BTreeSet::new).extend(iter);
        self
    }

    /// Filter unlabeled issues.
    pub fn unlabeled(&mut self) -> &mut Self {
        self.labels = Some(Some(Labels::None));
        self
    }

    /// Filter issues with any label.
    pub fn with_any_label(&mut self) -> &mut Self {
        self.labels = Some(Some(Labels::Any));
        self
    }

    /// Filter issues with a given label.
    pub fn label<L>(&mut self, label: L) -> &mut Self
    where
        L: Into<Cow<'a, str>>,
    {
        let label = label.into();
        let labels = if let Some(Some(Labels::AllOf(mut set))) = self.labels.take() {
            set.insert(label);
            set
        } else {
            let mut set = BTreeSet::new();
            set.insert(label);
            set
        };
        self.labels = Some(Some(Labels::AllOf(labels)));
        self
    }

    /// Filter issues with all of the given labels.
    pub fn labels<I, L>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = L>,
        L: Into<Cow<'a, str>>,
    {
        let iter = iter.into_iter().map(Into::into);
        let labels = if let Some(Some(Labels::AllOf(mut set))) = self.labels.take() {
            set.extend(iter);
            set
        } else {
            iter.collect()
        };
        self.labels = Some(Some(Labels::AllOf(labels)));
        self
    }

    /// Filter issues without a milestone.
    pub fn without_milestone(&mut self) -> &mut Self {
        self.milestone = Some(Some(Milestone::None));
        self
    }

    /// Filter issues with any milestone.
    pub fn any_milestone(&mut self) -> &mut Self {
        self.milestone = Some(Some(Milestone::Any));
        self
    }

    /// Filter issues with a given milestone.
    pub fn milestone<M>(&mut self, milestone: M) -> &mut Self
    where
        M: Into<Cow<'a, str>>,
    {
        self.milestone = Some(Some(Milestone::Named(milestone.into())));
        self
    }

    /// Filter unassigned issues.
    pub fn unassigned(&mut self) -> &mut Self {
        self.assignee = Some(Some(Assignee::Unassigned));
        self
    }

    /// Filter assigned issues.
    pub fn assigned(&mut self) -> &mut Self {
        self.assignee = Some(Some(Assignee::Assigned));
        self
    }

    /// Filter issues assigned to a user (by ID).
    pub fn assignee_id(&mut self, assignee: u64) -> &mut Self {
        self.assignee = Some(Some(Assignee::Id(assignee)));
        self
    }

    /// Filter issues assigned to a users (by username).
    pub fn assignee<A>(&mut self, assignee: A) -> &mut Self
    where
        A: Into<Cow<'a, str>>,
    {
        let assignee = assignee.into();
        let assignees = if let Some(Some(Assignee::Usernames(mut set))) = self.assignee.take() {
            set.insert(assignee);
            set
        } else {
            let mut set = BTreeSet::new();
            set.insert(assignee);
            set
        };
        self.assignee = Some(Some(Assignee::Usernames(assignees)));
        self
    }

    /// Filter issues assigned to a set of users.
    pub fn assignees<I, A>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = A>,
        A: Into<Cow<'a, str>>,
    {
        let iter = iter.into_iter().map(Into::into);
        let assignees = if let Some(Some(Assignee::Usernames(mut set))) = self.assignee.take() {
            set.extend(iter);
            set
        } else {
            iter.collect()
        };
        self.assignee = Some(Some(Assignee::Usernames(assignees)));
        self
    }

    /// Filter issues without a reaction by the API caller.
    pub fn no_reaction(&mut self) -> &mut Self {
        self.my_reaction_emoji = Some(Some(ReactionEmoji::None));
        self
    }

    /// Filter issues with any reaction by the API caller.
    pub fn any_reaction(&mut self) -> &mut Self {
        self.my_reaction_emoji = Some(Some(ReactionEmoji::Any));
        self
    }

    /// Filter issues with a specific reaction by the API caller.
    pub fn my_reaction<E>(&mut self, emoji: E) -> &mut Self
    where
        E: Into<Cow<'a, str>>,
    {
        self.my_reaction_emoji = Some(Some(ReactionEmoji::Emoji(emoji.into())));
        self
    }
}

impl<'a> Endpoint for Issues<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/issues", self.project).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .extend(self.iids.iter().map(|&value| ("iids[]", value)))
            .push_opt("state", self.state)
            .push_opt("labels", self.labels.as_ref())
            .push_opt("with_labels_details", self.with_labels_details)
            .push_opt("milestone", self.milestone.as_ref())
            .push_opt("scope", self.scope)
            .push_opt("my_reaction_emoji", self.my_reaction_emoji.as_ref())
            .push_opt("weight", self.weight)
            .push_opt("search", self.search.as_ref())
            .push_opt("created_after", self.created_after)
            .push_opt("created_before", self.created_before)
            .push_opt("updated_after", self.updated_after)
            .push_opt("updated_before", self.updated_before)
            .push_opt("confidential", self.confidential)
            .push_opt("order_by", self.order_by)
            .push_opt("sort", self.sort);

        if let Some(author) = self.author.as_ref() {
            match author {
                NameOrId::Name(name) => {
                    params.push("author_username", name);
                },
                NameOrId::Id(id) => {
                    params.push("author_id", *id);
                },
            }
        }
        if let Some(assignee) = self.assignee.as_ref() {
            assignee.add_params(&mut params);
        }

        params
    }
}

impl<'a> Pageable for Issues<'a> {}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::api::projects::issues::{IssueOrderBy, IssueScope, IssueState, IssueWeight, Issues};

    use super::{Labels, Milestone, ReactionEmoji};

    #[test]
    fn issue_state_as_str() {
        let items = &[
            (IssueState::Opened, "opened"),
            (IssueState::Closed, "closed"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn issue_labels_as_str() {
        let one_user = {
            let mut set = BTreeSet::new();
            set.insert("one".into());
            set
        };
        let two_users = {
            let mut set = BTreeSet::new();
            set.insert("one".into());
            set.insert("two".into());
            set
        };

        let items = &[
            (Labels::Any, "Any"),
            (Labels::None, "None"),
            (Labels::AllOf(one_user), "one"),
            (Labels::AllOf(two_users), "one,two"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn issue_milestone_as_str() {
        let items = &[
            (Milestone::Any, "Any"),
            (Milestone::None, "None"),
            (Milestone::Named("milestone".into()), "milestone"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn issue_scope_as_str() {
        let items = &[
            (IssueScope::CreatedByMe, "created_by_me"),
            (IssueScope::AssignedToMe, "assigned_to_me"),
            (IssueScope::All, "all"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn reaction_emoji_as_str() {
        let items = &[
            (ReactionEmoji::None, "None"),
            (ReactionEmoji::Any, "Any"),
            (ReactionEmoji::Emoji("emoji".into()), "emoji"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn issue_weight_as_str() {
        let items = &[
            (IssueWeight::Any, "Any"),
            (IssueWeight::None, "None"),
            (IssueWeight::Weight(0), "0"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn issue_order_by_default() {
        assert_eq!(IssueOrderBy::default(), IssueOrderBy::CreatedAt);
    }

    #[test]
    fn issue_order_by_as_str() {
        let items = &[
            (IssueOrderBy::CreatedAt, "created_at"),
            (IssueOrderBy::UpdatedAt, "updated_at"),
            (IssueOrderBy::Priority, "priority"),
            (IssueOrderBy::DueDate, "due_date"),
            (IssueOrderBy::RelativePosition, "relative_position"),
            (IssueOrderBy::LabelPriority, "label_priority"),
            (IssueOrderBy::MilestoneDue, "milestone_due"),
            (IssueOrderBy::Popularity, "popularity"),
            (IssueOrderBy::WeightFields, "weight_fields"),
        ];

        for (i, s) in items {
            assert_eq!(i.as_str(), *s);
        }
    }

    #[test]
    fn project_is_needed() {
        let err = Issues::builder().build().unwrap_err();
        assert_eq!(err, "`project` must be initialized");
    }

    #[test]
    fn project_is_sufficient() {
        Issues::builder().project(1).build().unwrap();
    }
}
