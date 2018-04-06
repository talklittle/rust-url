// Copyright 2013-2018 The rust-url developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


//! Implementation using UTF-8 only.
//! Used when building without any query encoding feature flags.

use std::borrow::Cow;

use encoding::utf8_helpers::{decode_utf8_lossy, encode_utf8};

#[derive(Copy, Clone, Debug)]
pub struct EncodingOverride;

impl EncodingOverride {
    #[inline]
    pub fn utf8() -> Self {
        EncodingOverride
    }

    pub fn decode<'a>(&self, input: Cow<'a, [u8]>) -> Cow<'a, str> {
        decode_utf8_lossy(input)
    }

    pub fn encode<'a>(&self, input: Cow<'a, str>) -> Cow<'a, [u8]> {
        encode_utf8(input)
    }
}
