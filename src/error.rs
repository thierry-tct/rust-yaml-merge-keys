// Copyright 2017 Kitware, Inc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

error_chain! {
    errors {
        /// A non-hash value was given as a value to merge into a hash.
        ///
        /// This happens with a document such as:
        ///
        /// ```yaml
        /// -
        ///   <<: 4
        ///   x: 1
        /// ```
        InvalidMergeValue {
            description("only mappings and arrays of mappings may be merged")
        }
    }
}
