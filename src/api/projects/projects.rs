// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeMap;
use std::fmt;

use chrono::{DateTime, Utc};
use derive_builder::Builder;

use crate::query_prelude::*;
use crate::types::{AccessLevel, VisibilityLevel};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectOrderBy {
    Id,
    Name,
    Path,
    CreatedAt,
    UpdatedAt,
    LastActivityAt,
}

impl Default for ProjectOrderBy {
    fn default() -> Self {
        ProjectOrderBy::CreatedAt
    }
}

impl ProjectOrderBy {
    fn use_keyset_pagination(self) -> bool {
        self == ProjectOrderBy::Id
    }

    fn as_str(self) -> &'static str {
        match self {
            ProjectOrderBy::Id => "id",
            ProjectOrderBy::Name => "name",
            ProjectOrderBy::Path => "path",
            ProjectOrderBy::CreatedAt => "created_at",
            ProjectOrderBy::UpdatedAt => "updated_at",
            ProjectOrderBy::LastActivityAt => "last_activity_at",
        }
    }
}

impl fmt::Display for ProjectOrderBy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct Projects {
    /// Search for projects using a query string.
    ///
    /// The search query will be escaped automatically.
    #[builder(default)]
    search: Option<String>,

    #[builder(default)]
    archived: Option<bool>,
    #[builder(default)]
    visibility: Option<VisibilityLevel>,
    #[builder(default)]
    search_namespaces: Option<bool>,
    #[builder(default)]
    simple: Option<bool>,
    #[builder(default)]
    owned: Option<bool>,
    #[builder(default)]
    membership: Option<bool>,
    #[builder(default)]
    starred: Option<bool>,
    #[builder(default)]
    statistics: Option<bool>,
    /// Pagination to use for the results.
    #[builder(default)]
    pagination: Pagination,

    #[builder(default)]
    with_issues_enabled: Option<bool>,
    #[builder(default)]
    with_merge_requests_enabled: Option<bool>,
    #[builder(default)]
    with_programming_language: Option<String>,
    #[builder(default)]
    wiki_checksum_failed: Option<bool>,
    #[builder(default)]
    repository_checksum_failed: Option<bool>,
    #[builder(default)]
    min_access_level: Option<AccessLevel>,

    /// Search for users with a given custom attribute set.
    #[builder(setter(name = "_custom_attributes"), default, private)]
    custom_attributes: BTreeMap<String, String>,
    /// Search for users with custom attributes.
    #[builder(default)]
    with_custom_attributes: Option<bool>,

    #[builder(default)]
    id_after: Option<u64>,
    #[builder(default)]
    id_before: Option<u64>,
    #[builder(default)]
    last_activity_after: Option<DateTime<Utc>>,
    #[builder(default)]
    last_activity_before: Option<DateTime<Utc>>,

    /// Order results by a given key.
    #[builder(default)]
    order_by: Option<ProjectOrderBy>,
    /// The sort order for return results.
    #[builder(default)]
    sort: Option<SortOrder>,
}

impl Projects {
    pub fn builder() -> ProjectsBuilder {
        ProjectsBuilder::default()
    }
}

impl ProjectsBuilder {
    /// Clear custom attribute search parameters.
    pub fn clear_custom_attributes(&mut self) -> &mut Self {
        self.custom_attributes = None;
        self
    }

    /// Add a custom attribute search parameter.
    pub fn custom_attribute<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.custom_attributes
            .get_or_insert_with(Default::default)
            .insert(key.into(), value.into());
        self
    }

    /// Add multiple custom attribute search parameters.
    pub fn custom_attributes<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.custom_attributes
            .get_or_insert_with(Default::default)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }
}

fn bool_as_str(b: bool) -> &'static str {
    if b {
        "true"
    } else {
        "false"
    }
}

impl<T> SingleQuery<Vec<T>> for Projects
where
    T: DeserializeOwned,
{
    type FormData = ();

    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> String {
        "projects".into()
    }

    fn add_parameters(&self, mut pairs: Pairs) {
        self.search
            .as_ref()
            .map(|value| pairs.append_pair("search", value));
        self.archived
            .map(|value| pairs.append_pair("archived", bool_as_str(value)));
        self.visibility
            .map(|value| pairs.append_pair("visibility", value.as_str()));
        self.search_namespaces
            .map(|value| pairs.append_pair("search_namespaces", bool_as_str(value)));
        self.simple
            .map(|value| pairs.append_pair("simple", bool_as_str(value)));
        self.owned
            .map(|value| pairs.append_pair("owned", bool_as_str(value)));
        self.membership
            .map(|value| pairs.append_pair("membership", bool_as_str(value)));
        self.starred
            .map(|value| pairs.append_pair("starred", bool_as_str(value)));
        self.statistics
            .map(|value| pairs.append_pair("statistics", bool_as_str(value)));
        self.with_issues_enabled
            .map(|value| pairs.append_pair("with_issues_enabled", bool_as_str(value)));
        self.with_merge_requests_enabled
            .map(|value| pairs.append_pair("with_merge_requests_enabled", bool_as_str(value)));
        self.with_programming_language
            .as_ref()
            .map(|value| pairs.append_pair("with_programming_language", value));
        self.wiki_checksum_failed
            .map(|value| pairs.append_pair("wiki_checksum_failed", bool_as_str(value)));
        self.repository_checksum_failed
            .map(|value| pairs.append_pair("repository_checksum_failed", bool_as_str(value)));

        self.min_access_level
            .map(|value| pairs.append_pair("min_access_level", &format!("{}", u64::from(value))));

        self.id_after
            .map(|value| pairs.append_pair("id_after", &format!("{}", value)));
        self.id_before
            .map(|value| pairs.append_pair("id_before", &format!("{}", value)));
        self.last_activity_after.map(|value| {
            pairs.append_pair(
                "last_activity_after",
                &value.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            )
        });
        self.last_activity_before.map(|value| {
            pairs.append_pair(
                "last_activity_before",
                &value.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            )
        });

        pairs.extend_pairs(
            self.custom_attributes
                .iter()
                .map(|(key, value)| (format!("custom_attribute[{}]", key), value)),
        );
        self.with_custom_attributes
            .map(|value| pairs.append_pair("with_custom_attributes", bool_as_str(value)));

        self.order_by
            .map(|value| pairs.append_pair("order_by", value.as_str()));
        self.sort
            .map(|value| pairs.append_pair("sort", value.as_str()));
    }

    fn form_data(&self) {}
}

impl<T> PagedQuery<T, ()> for Projects
where
    T: DeserializeOwned,
{
    fn pagination(&self) -> Pagination {
        self.pagination
    }

    fn use_keyset_pagination(&self) -> bool {
        self.order_by
            .map_or(false, |order_by| order_by.use_keyset_pagination())
    }
}

impl<T> Query<Vec<T>> for Projects
where
    T: DeserializeOwned,
{
    fn query(&self, client: &Gitlab) -> Result<Vec<T>, GitlabError> {
        self.paged_query(client)
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::Projects;

    #[test]
    fn defaults_work() {
        Projects::builder().build().unwrap();
    }
}