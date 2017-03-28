// Copyright 2017 Kitware, Inc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! YAML Merge Keys
//!
//! The YAML Merge Key extension is not supported by the core YAML crate, but can be implemented
//! after parsing. This crate transforms a parsed YAML document and merges dictionaries together.
//!
//! # Example
//!
//! ```yaml
//! ---
//! - &CENTER { x: 1, y: 2 }
//! - &LEFT { x: 0, y: 2 }
//! - &BIG { r: 10 }
//! - &SMALL { r: 1 }
//!
//! # All the following maps are equal:
//!
//! - # Explicit keys
//!   x: 1
//!   y: 2
//!   r: 10
//!   label: center/big
//!
//! - # Merge one map
//!   << : *CENTER
//!   r: 10
//!   label: center/big
//!
//! - # Merge multiple maps
//!   << : [ *CENTER, *BIG ]
//!   label: center/big
//!
//! - # Override
//!   << : [ *BIG, *LEFT, *SMALL ]
//!   x: 1
//!   label: center/big
//! ```

#![deny(missing_docs)]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate lazy_static;

mod error;
mod merge_keys;
#[cfg(feature="serde_yaml")]
mod serde;

pub use error::*;
pub use merge_keys::merge_keys;
#[cfg(feature="serde_yaml")]
pub use serde::merge_keys_serde;

#[cfg(test)]
mod test;
