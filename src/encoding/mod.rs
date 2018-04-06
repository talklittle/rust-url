// Copyright 2013-2018 The rust-url developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


//! Abstraction that conditionally compiles either to encoding_rs,
//! or rust-encoding (legacy), or to only support UTF-8.

mod utf8_helpers;

#[cfg(feature = "query_encoding")]
mod legacy;
#[cfg(feature = "query_encoding")]
pub use self::legacy::{EncodingOverride, EncodingRef};

#[cfg(not(feature = "query_encoding"))]
mod fallback;
#[cfg(not(feature = "query_encoding"))]
pub use self::fallback::EncodingOverride;
