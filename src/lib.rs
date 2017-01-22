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

extern crate itertools;
use itertools::Itertools;

#[macro_use]
extern crate lazy_static;

extern crate yaml_rust;
use yaml_rust::Yaml;
use yaml_rust::yaml::{Array, Hash};

#[cfg(test)]
mod test;

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

lazy_static! {
    /// The name of the key to use for merge data.
    static ref MERGE_KEY: Yaml = Yaml::String("<<".to_string());
}

/// Merge values together.
fn merge_values(mut hash: Hash, value: Yaml) -> Result<Hash> {
    let merge_values = match value {
        Yaml::Array(arr) => {
            let init: Result<Hash> = Ok(Hash::new());

            try!(arr.into_iter()
                .fold(init, |res_hash, item| {
                    // Merge in the next item.
                    res_hash.and_then(move |mut res_hash| {
                        if let Yaml::Hash(next_hash) = item {
                            next_hash.into_iter()
                                .foreach(|(key, value)| {
                                    res_hash.entry(key).or_insert(value);
                                });
                            Ok(res_hash)
                        } else {
                            // Non-hash values at this level are not allowed.
                            bail!(ErrorKind::InvalidMergeValue)
                        }
                    })
                }))
        },
        Yaml::Hash(merge_hash) => merge_hash,
        _ => bail!(ErrorKind::InvalidMergeValue),
    };

    merge_values.into_iter()
        .foreach(|(key, value)| {
            hash.entry(key).or_insert(value);
        });

    Ok(hash)
}

/// Recurse into a hash and handle items with merge keys in them.
fn merge_hash(hash: Hash) -> Result<Yaml> {
    let mut hash = try!(hash.into_iter()
        // First handle any merge keys in the key or value...
        .map(|(key, value)| {
            merge_keys(key)
                .and_then(|key| {
                    merge_keys(value)
                        .map(|value| (key, value))
                })
        })
        .collect::<Result<Hash>>());

    if let Some(merge_value) = hash.remove(&MERGE_KEY) {
        merge_values(hash, merge_value)
            .map(Yaml::Hash)
    } else {
        Ok(Yaml::Hash(hash))
    }
}

/// Recurse into an array and handle items with merge keys in them.
fn merge_array(arr: Array) -> Result<Yaml> {
    arr.into_iter()
        .map(merge_keys)
        .collect::<Result<Array>>()
        .map(Yaml::Array)
}

/// Handle merge keys in a YAML document.
pub fn merge_keys(doc: Yaml) -> Result<Yaml> {
    match doc {
        Yaml::Hash(hash) => merge_hash(hash),
        Yaml::Array(arr) => merge_array(arr),
        _ => Ok(doc),
    }
}
