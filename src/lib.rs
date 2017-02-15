// Copyright 2016 Kitware, Inc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![warn(missing_docs)]

//! A library for communicating with Gitlab instances.

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate hyper;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

mod error;
#[macro_use]
mod macros;
mod gitlab;

pub mod systemhooks;
pub mod types;
pub mod webhooks;
pub mod hooks;

pub use error::*;
pub use gitlab::CommitStatusInfo;
pub use gitlab::MergeRequestStateFilter;
pub use gitlab::Gitlab;
pub use types::*;

#[cfg(test)]
mod test;
