// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for awards on a merge request within a project.
#[derive(Debug, Builder)]
pub struct MergeRequestAwards<'a> {
    /// The project to query for the merge request.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The ID of the merge request.
    merge_request: u64,
}

impl<'a> MergeRequestAwards<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> MergeRequestAwardsBuilder<'a> {
        MergeRequestAwardsBuilder::default()
    }
}

impl<'a> Endpoint for MergeRequestAwards<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/merge_requests/{}/award_emoji",
            self.project, self.merge_request,
        )
        .into()
    }
}

impl<'a> Pageable for MergeRequestAwards<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::projects::merge_requests::awards::MergeRequestAwards;

    #[test]
    fn project_and_merge_request_are_necessary() {
        let err = MergeRequestAwards::builder().build().unwrap_err();
        assert_eq!(err, "`project` must be initialized");
    }

    #[test]
    fn project_is_necessary() {
        let err = MergeRequestAwards::builder()
            .merge_request(1)
            .build()
            .unwrap_err();
        assert_eq!(err, "`project` must be initialized");
    }

    #[test]
    fn merge_request_is_necessary() {
        let err = MergeRequestAwards::builder()
            .project(1)
            .build()
            .unwrap_err();
        assert_eq!(err, "`merge_request` must be initialized");
    }

    #[test]
    fn project_and_merge_request_are_sufficient() {
        MergeRequestAwards::builder()
            .project(1)
            .merge_request(1)
            .build()
            .unwrap();
    }
}
