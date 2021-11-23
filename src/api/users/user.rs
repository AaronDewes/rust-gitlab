// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;

/// Query a user by ID.
#[derive(Debug, Clone, Copy, Builder)]
pub struct User {
    /// The ID of the user.
    user: u64,
}

impl User {
    /// Create a builder for the endpoint.
    pub fn builder() -> UserBuilder {
        UserBuilder::default()
    }
}

impl Endpoint for User {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("users/{}", self.user).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::users::{User, UserBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn user_is_needed() {
        let err = User::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, UserBuilderError, "user");
    }

    #[test]
    fn user_is_sufficient() {
        User::builder().user(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder().endpoint("users/1").build().unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = User::builder().user(1).build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
