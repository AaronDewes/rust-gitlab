// Copyright 2016 Kitware, Inc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate serde;
use self::serde::{Deserialize, Deserializer};
use self::serde::de::Error;

extern crate serde_json;
use self::serde_json::{from_value, Value};

use super::systemhooks::SystemHook;
use super::webhooks::WebHook;

#[derive(Debug)]
pub enum GitlabHook {
    System(SystemHook),
    Web(WebHook),
}

impl Deserialize for GitlabHook {
    fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        let val = try!(Value::deserialize(deserializer));

        // Look for `object_kind` first because some web hooks also have `event_name` which would
        // cause a false match here.
        let hook_res = if val.pointer("/object_kind").is_some() {
                from_value(val).map(GitlabHook::Web)
            } else if val.pointer("/event_name").is_some() {
                from_value(val).map(GitlabHook::System)
            } else {
                return Err(D::Error::missing_field("either object_kind or event_name"));
            };

        hook_res.map_err(|err| D::Error::invalid_value(&format!("{:?}", err)))
    }
}
