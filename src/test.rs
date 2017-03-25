// Copyright 2017 Kitware, Inc.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate yaml_rust;
use self::yaml_rust::Yaml;

use merge_keys;

fn assert_yaml_idempotent(doc: Yaml) {
    assert_eq!(merge_keys(doc.clone()).unwrap(), doc);
}

fn merge_key() -> Yaml {
    Yaml::String("<<".to_string())
}

macro_rules! yaml_hash {
    [ $( $pair:expr, )* ] => {
        Yaml::Hash([$( $pair, )*].into_iter().cloned().collect())
    }
}

#[test]
fn test_ignore_non_containers() {
    let null = Yaml::Null;
    let bool_true = Yaml::Boolean(true);
    let bool_false = Yaml::Boolean(false);
    let string = Yaml::String("".to_string());
    let integer = Yaml::Integer(1234);
    let real = Yaml::Real("0.02".to_string());

    assert_yaml_idempotent(null);
    assert_yaml_idempotent(bool_true);
    assert_yaml_idempotent(bool_false);
    assert_yaml_idempotent(string);
    assert_yaml_idempotent(integer);
    assert_yaml_idempotent(real);
}

#[test]
fn test_ignore_container_no_merge_keys() {
    let arr = Yaml::Array(vec![
        Yaml::Integer(10),
        Yaml::Integer(100),
    ]);
    let hash = yaml_hash![
        (Yaml::Integer(10), Yaml::Null),
        (Yaml::Integer(100), Yaml::String("string".to_string())),
    ];

    assert_yaml_idempotent(arr);
    assert_yaml_idempotent(hash);
}

#[test]
fn test_remove_merge_keys() {
    let hash = yaml_hash![
        (merge_key(), yaml_hash![]),
        (Yaml::Integer(10), Yaml::Null),
        (Yaml::Integer(100), Yaml::String("string".to_string())),
    ];
    let expected = yaml_hash![
        (Yaml::Integer(10), Yaml::Null),
        (Yaml::Integer(100), Yaml::String("string".to_string())),
    ];

    assert_eq!(merge_keys(hash).unwrap(), expected);
}

#[test]
fn test_handle_merge_keys() {
    let hash = yaml_hash![
        (merge_key(), yaml_hash![
            (Yaml::Integer(15), Yaml::Null),
        ]),
        (Yaml::Integer(10), Yaml::Null),
        (Yaml::Integer(100), Yaml::String("string".to_string())),
    ];
    let expected = yaml_hash![
        (Yaml::Integer(10), Yaml::Null),
        (Yaml::Integer(15), Yaml::Null),
        (Yaml::Integer(100), Yaml::String("string".to_string())),
    ];

    assert_eq!(merge_keys(hash).unwrap(), expected);
}

#[test]
fn test_merge_key_precedence() {
    let hash = yaml_hash![
        (merge_key(), yaml_hash![
            (Yaml::Integer(10), Yaml::Integer(10)),
        ]),
        (Yaml::Integer(10), Yaml::Null),
        (Yaml::Integer(100), Yaml::String("string".to_string())),
    ];
    let expected = yaml_hash![
        (Yaml::Integer(10), Yaml::Null),
        (Yaml::Integer(100), Yaml::String("string".to_string())),
    ];

    assert_eq!(merge_keys(hash).unwrap(), expected);
}

#[test]
fn test_merge_key_array() {
    let hash = yaml_hash![
        (merge_key(), Yaml::Array(vec![
            yaml_hash![
                (Yaml::Integer(15), Yaml::Integer(10)),
            ],
            yaml_hash![
                (Yaml::Integer(20), Yaml::Integer(10)),
            ],
        ])),
        (Yaml::Integer(10), Yaml::Null),
        (Yaml::Integer(100), Yaml::String("string".to_string())),
    ];
    let expected = yaml_hash![
        (Yaml::Integer(10), Yaml::Null),
        (Yaml::Integer(15), Yaml::Integer(10)),
        (Yaml::Integer(20), Yaml::Integer(10)),
        (Yaml::Integer(100), Yaml::String("string".to_string())),
    ];

    assert_eq!(merge_keys(hash).unwrap(), expected);
}

#[test]
fn test_merge_key_array_precedence() {
    let hash = yaml_hash![
        (merge_key(), Yaml::Array(vec![
            yaml_hash![
                (Yaml::Integer(15), Yaml::Integer(10)),
            ],
            yaml_hash![
                (Yaml::Integer(15), Yaml::Integer(20)),
            ],
        ])),
        (Yaml::Integer(10), Yaml::Null),
        (Yaml::Integer(100), Yaml::String("string".to_string())),
    ];
    let expected = yaml_hash![
        (Yaml::Integer(10), Yaml::Null),
        (Yaml::Integer(15), Yaml::Integer(10)),
        (Yaml::Integer(100), Yaml::String("string".to_string())),
    ];

    assert_eq!(merge_keys(hash).unwrap(), expected);
}

#[test]
fn test_merge_key_nested_array() {
    let hash = Yaml::Array(vec![
        yaml_hash![
            (merge_key(), Yaml::Array(vec![
                yaml_hash![
                    (Yaml::Integer(15), Yaml::Integer(10)),
                ],
                yaml_hash![
                    (Yaml::Integer(15), Yaml::Integer(20)),
                ],
            ])),
            (Yaml::Integer(10), Yaml::Null),
            (Yaml::Integer(100), Yaml::String("string".to_string())),
        ],
    ]);
    let expected = Yaml::Array(vec![
        yaml_hash![
            (Yaml::Integer(10), Yaml::Null),
            (Yaml::Integer(15), Yaml::Integer(10)),
            (Yaml::Integer(100), Yaml::String("string".to_string())),
        ],
    ]);

    assert_eq!(merge_keys(hash).unwrap(), expected);
}

#[test]
fn test_merge_key_nested_hash_value() {
    let hash = yaml_hash![
        (Yaml::Null, yaml_hash![
            (merge_key(), Yaml::Array(vec![
                yaml_hash![
                    (Yaml::Integer(15), Yaml::Integer(10)),
                ],
                yaml_hash![
                    (Yaml::Integer(15), Yaml::Integer(20)),
                ],
            ])),
            (Yaml::Integer(10), Yaml::Null),
            (Yaml::Integer(100), Yaml::String("string".to_string())),
        ]),
    ];
    let expected = yaml_hash![
        (Yaml::Null, yaml_hash![
            (Yaml::Integer(10), Yaml::Null),
            (Yaml::Integer(15), Yaml::Integer(10)),
            (Yaml::Integer(100), Yaml::String("string".to_string())),
        ]),
    ];

    assert_eq!(merge_keys(hash).unwrap(), expected);
}

#[test]
fn test_merge_key_nested_hash_key() {
    let hash = yaml_hash![
        (yaml_hash![
            (merge_key(), Yaml::Array(vec![
                yaml_hash![
                    (Yaml::Integer(15), Yaml::Integer(10)),
                ],
                yaml_hash![
                    (Yaml::Integer(15), Yaml::Integer(20)),
                ],
            ])),
            (Yaml::Integer(10), Yaml::Null),
            (Yaml::Integer(100), Yaml::String("string".to_string())),
        ], Yaml::Null),
    ];
    let expected = yaml_hash![
        (yaml_hash![
            (Yaml::Integer(10), Yaml::Null),
            (Yaml::Integer(15), Yaml::Integer(10)),
            (Yaml::Integer(100), Yaml::String("string".to_string())),
        ], Yaml::Null),
    ];

    assert_eq!(merge_keys(hash).unwrap(), expected);
}

#[test]
fn test_yaml_spec_examples() {
    let center = yaml_hash![
        (Yaml::String("x".to_string()), Yaml::Integer(1)),
        (Yaml::String("y".to_string()), Yaml::Integer(2)),
    ];
    let left = yaml_hash![
        (Yaml::String("x".to_string()), Yaml::Integer(0)),
        (Yaml::String("y".to_string()), Yaml::Integer(2)),
    ];
    let big = yaml_hash![
        (Yaml::String("r".to_string()), Yaml::Integer(10)),
    ];
    let small = yaml_hash![
        (Yaml::String("r".to_string()), Yaml::Integer(1)),
    ];

    let explicit = yaml_hash![
        (Yaml::String("x".to_string()), Yaml::Integer(1)),
        (Yaml::String("y".to_string()), Yaml::Integer(2)),
        (Yaml::String("r".to_string()), Yaml::Integer(10)),
        (Yaml::String("label".to_string()), Yaml::String("center/big".to_string())),
    ];
    let merge_one_map = yaml_hash![
        (merge_key(), center.clone()),
        (Yaml::String("r".to_string()), Yaml::Integer(10)),
        (Yaml::String("label".to_string()), Yaml::String("center/big".to_string())),
    ];
    let merge_multiple_maps = yaml_hash![
        (merge_key(), Yaml::Array(vec![center.clone(), big.clone()])),
        (Yaml::String("r".to_string()), Yaml::Integer(10)),
        (Yaml::String("label".to_string()), Yaml::String("center/big".to_string())),
    ];
    let overrides = yaml_hash![
        (merge_key(), Yaml::Array(vec![big.clone(), left.clone(), small.clone()])),
        (Yaml::String("x".to_string()), Yaml::Integer(1)),
        (Yaml::String("label".to_string()), Yaml::String("center/big".to_string())),
    ];

    assert_eq!(merge_keys(explicit.clone()).unwrap(), explicit);
    assert_eq!(merge_keys(merge_one_map).unwrap(), explicit);
    assert_eq!(merge_keys(merge_multiple_maps).unwrap(), explicit);
    assert_eq!(merge_keys(overrides).unwrap(), explicit);
}
