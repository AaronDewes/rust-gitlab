// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::api::{ApiError, Client, Endpoint, Query};

/// A query modifier that returns the raw data from the endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Raw<E> {
    endpoint: E,
}

/// Return the raw data from the endpoint.
pub fn raw<E>(endpoint: E) -> Raw<E> {
    Raw {
        endpoint,
    }
}

impl<E, C> Query<Vec<u8>, C> for Raw<E>
where
    E: Endpoint,
    C: Client,
{
    fn query(&self, client: &C) -> Result<Vec<u8>, ApiError<C::Error>> {
        let mut url = client.rest_endpoint(&self.endpoint.endpoint())?;
        self.endpoint.add_parameters(url.query_pairs_mut());

        let req = client
            .build_rest(self.endpoint.method(), url)
            .form(&self.endpoint.form_data());
        let rsp = client.rest(req)?;
        if !rsp.status().is_success() {
            let v = serde_json::from_reader(rsp)?;
            return Err(ApiError::from_gitlab(v));
        }

        Ok(rsp.bytes().unwrap().as_ref().into())
    }
}
