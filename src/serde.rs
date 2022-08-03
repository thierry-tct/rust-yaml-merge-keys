// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde_yaml::value::{Tag, TaggedValue};
use serde_yaml::Value;
use yaml_rust::yaml::Hash;
use yaml_rust::Yaml;

use crate::merge_keys;
use crate::MergeKeyError;

struct YamlWrap(Yaml);

impl YamlWrap {
    fn into_yaml(self) -> Yaml {
        self.into()
    }
}

impl From<YamlWrap> for Yaml {
    fn from(yaml: YamlWrap) -> Self {
        yaml.0
    }
}

impl From<Yaml> for YamlWrap {
    fn from(yaml: Yaml) -> Self {
        YamlWrap(yaml)
    }
}

pub(crate) const TAGGED_YAML_SMUGGLE_TAG_KEY: &str = "70235535-46bb-46f9-b535-31596b77733f";
pub(crate) const TAGGED_YAML_SMUGGLE_VALUE_KEY: &str = "b5282e53-aaaf-4f67-a0b5-d5bb3bd0d4d2";

impl From<Value> for YamlWrap {
    fn from(yaml: Value) -> Self {
        YamlWrap(match yaml {
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Yaml::Integer(i)
                } else {
                    Yaml::Real(format!("{}", n))
                }
            },
            Value::String(s) => Yaml::String(s),
            Value::Bool(s) => Yaml::Boolean(s),
            Value::Sequence(seq) => {
                Yaml::Array(
                    seq.into_iter()
                        .map(Into::into)
                        .map(Self::into_yaml)
                        .collect(),
                )
            },
            Value::Mapping(map) => {
                Yaml::Hash(
                    map.into_iter()
                        .map(|(k, v)| (Self::into_yaml(k.into()), Self::into_yaml(v.into())))
                        .collect(),
                )
            },
            Value::Tagged(tagged) => {
                Yaml::Hash(
                    // XXX(rust-2021): use `.into_iter()` instead.
                    IntoIterator::into_iter([
                        (
                            Yaml::String(TAGGED_YAML_SMUGGLE_TAG_KEY.into()),
                            Yaml::String(format!("{}", tagged.tag)),
                        ),
                        (
                            Yaml::String(TAGGED_YAML_SMUGGLE_VALUE_KEY.into()),
                            YamlWrap::from(tagged.value).into(),
                        ),
                    ])
                    .collect(),
                )
            },
            Value::Null => Yaml::Null,
        })
    }
}

fn as_smuggled_tagged_value(mut hash: Hash) -> Result<(Tag, Value), Hash> {
    let value_key = Yaml::String(TAGGED_YAML_SMUGGLE_VALUE_KEY.into());
    let tag_key = Yaml::String(TAGGED_YAML_SMUGGLE_TAG_KEY.into());

    if hash.len() == 2
        && hash.contains_key(&value_key)
        && matches!(hash.get(&tag_key), Some(Yaml::String(_)))
    {
        let tag = match hash
            .remove(&tag_key)
            .expect("tag was checked in the condition")
        {
            Yaml::String(tag) => tag,
            _ => unreachable!("tag value type was checked in the condition"),
        };
        let value = hash
            .remove(&value_key)
            .expect("value was checked in the condition");

        Ok((Tag::new(tag), YamlWrap(value).into()))
    } else {
        Err(hash)
    }
}

impl From<YamlWrap> for Value {
    fn from(yaml: YamlWrap) -> Self {
        match yaml.0 {
            Yaml::Real(f) => {
                match serde_yaml::from_str(&f) {
                    Ok(f) => Value::Number(f),
                    Err(_) => Value::String(f),
                }
            },
            Yaml::Integer(i) => Value::Number(i.into()),
            Yaml::String(s) => Value::String(s),
            Yaml::Boolean(b) => Value::Bool(b),
            Yaml::Array(array) => {
                Value::Sequence(
                    array
                        .into_iter()
                        .map(|item| {
                            let wrap: YamlWrap = item.into();
                            wrap.into()
                        })
                        .collect(),
                )
            },
            Yaml::Hash(hash) => {
                match as_smuggled_tagged_value(hash) {
                    Ok((tag, value)) => {
                        Value::Tagged(Box::new(TaggedValue {
                            tag,
                            value,
                        }))
                    },
                    Err(hash) => {
                        Value::Mapping(
                            hash.into_iter()
                                .map(|(k, v)| {
                                    let key: YamlWrap = k.into();
                                    let value: YamlWrap = v.into();
                                    (key.into(), value.into())
                                })
                                .collect(),
                        )
                    },
                }
            },
            Yaml::Alias(_) => unreachable!("alias unsupported"),
            Yaml::Null => Value::Null,
            Yaml::BadValue => unreachable!("bad value"),
        }
    }
}

/// Handle merge keys in a serde YAML document.
pub fn merge_keys_serde(doc: Value) -> Result<Value, MergeKeyError> {
    merge_keys(YamlWrap::into_yaml(doc.into()))
        .map(YamlWrap)
        .map(Into::into)
}
