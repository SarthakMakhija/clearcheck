use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::matchers::{Matcher, MatcherResult};

pub enum KeyMembershipBased<'a, T: Debug> {
    Key(&'a T),
    AllKeys(&'a [T]),
    AnyOfKeys(&'a [T]),
}

pub enum ValueMembershipBased<'a, T: Debug> {
    Value(&'a T),
    AllValues(&'a [T]),
    AnyOfValues(&'a [T]),
}

pub enum KeyValueMembershipBased<'a, K: Debug, V: Debug> {
    KeyValue(&'a K, &'a V),
}

impl<K, V> Matcher<HashMap<K, V>> for KeyMembershipBased<'_, K>
where
    K: Hash + Eq + Debug,
{
    fn test(&self, collection: &HashMap<K, V>) -> MatcherResult {
        match self {
            KeyMembershipBased::Key(key) => MatcherResult::formatted(
                collection.contains_key(key),
                format!(
                    "Keys {:?} in the map should contain {:?}",
                    collection.keys(),
                    key
                ),
                format!(
                    "Keys {:?} in the map should not contain {:?}",
                    collection.keys(),
                    key
                ),
            ),
            KeyMembershipBased::AllKeys(keys) => {
                let missing = keys
                    .iter()
                    .filter(|key| !collection.contains_key(key))
                    .collect::<Vec<_>>();

                MatcherResult::formatted(
                    missing.len() == 0,
                    format!(
                        "Keys {:?} in the map should contain all {:?} but it was missing {:?}",
                        collection.keys(),
                        keys,
                        missing
                    ),
                    format!(
                        "Keys {:?} in the map should not contain {:?}",
                        collection.keys(),
                        keys
                    ),
                )
            }
            KeyMembershipBased::AnyOfKeys(keys) => MatcherResult::formatted(
                keys.iter().any(|key| collection.contains_key(key)),
                format!(
                    "Keys {:?} in the map should contain any of the keys {:?}",
                    collection.keys(),
                    keys
                ),
                format!(
                    "Keys {:?} in the map should not contain any of the keys {:?}",
                    collection.keys(),
                    keys
                ),
            ),
        }
    }
}

impl<V> ValueMembershipBased<'_, V>
where
    V: Eq + Debug,
{
    fn contains_value<K: Hash + Eq>(collection: &HashMap<K, V>, value: &&V) -> bool {
        collection.values().any(|source| &source == value)
    }
}

impl<K, V> Matcher<HashMap<K, V>> for ValueMembershipBased<'_, V>
where
    K: Hash + Eq,
    V: Eq + Debug,
{
    fn test(&self, collection: &HashMap<K, V>) -> MatcherResult {
        match self {
            ValueMembershipBased::Value(value) => MatcherResult::formatted(
                Self::contains_value(collection, value),
                format!(
                    "Values {:?} in the map should contain {:?}",
                    collection.values(),
                    value
                ),
                format!(
                    "Values {:?} in the map should not contain {:?}",
                    collection.values(),
                    value
                ),
            ),
            ValueMembershipBased::AllValues(values) => {
                let missing = values
                    .iter()
                    .filter(|value| !Self::contains_value(collection, value))
                    .collect::<Vec<_>>();

                MatcherResult::formatted(
                    missing.len() == 0,
                    format!(
                        "Values {:?} in the map should contain all {:?} but it was missing {:?}",
                        collection.values(),
                        values,
                        missing
                    ),
                    format!(
                        "Values {:?} in the map should not contain {:?}",
                        collection.values(),
                        values
                    ),
                )
            }
            ValueMembershipBased::AnyOfValues(values) => MatcherResult::formatted(
                values
                    .iter()
                    .any(|value| Self::contains_value(collection, &value)),
                format!(
                    "Values {:?} in the map should contain any of the values {:?}",
                    collection.values(),
                    values
                ),
                format!(
                    "Values {:?} in the map should not contain any of the values {:?}",
                    collection.values(),
                    values
                ),
            ),
        }
    }
}

impl<K, V> Matcher<HashMap<K, V>> for KeyValueMembershipBased<'_, K, V>
where
    K: Hash + Eq + Debug,
    V: Eq + Debug,
{
    fn test(&self, collection: &HashMap<K, V>) -> MatcherResult {
        return match self {
            KeyValueMembershipBased::KeyValue(key, value) => MatcherResult::formatted(
                collection
                    .get(key)
                    .filter(|source| source == value)
                    .is_some(),
                format!(
                    "Map {:?} should contain key {:?} and value {:?}",
                    collection, key, value
                ),
                format!(
                    "Map {:?} should contain key {:?} and value {:?}",
                    collection, key, value
                ),
            ),
        };
    }
}

pub fn contain_key<Q>(key: &Q) -> KeyMembershipBased<'_, Q>
where
    Q: Hash + Eq + Debug,
{
    KeyMembershipBased::Key(key)
}

pub fn contain_all_keys<Q>(keys: &[Q]) -> KeyMembershipBased<'_, Q>
where
    Q: Hash + Eq + Debug,
{
    KeyMembershipBased::AllKeys(keys)
}

pub fn contain_any_of_keys<Q>(keys: &[Q]) -> KeyMembershipBased<'_, Q>
where
    Q: Hash + Eq + Debug,
{
    KeyMembershipBased::AnyOfKeys(keys)
}

pub fn contain_value<Q>(value: &Q) -> ValueMembershipBased<'_, Q>
where
    Q: Eq + Debug,
{
    ValueMembershipBased::Value(value)
}

pub fn contain_all_values<Q>(values: &[Q]) -> ValueMembershipBased<'_, Q>
where
    Q: Eq + Debug,
{
    ValueMembershipBased::AllValues(values)
}

pub fn contain_any_of_values<Q>(values: &[Q]) -> ValueMembershipBased<'_, Q>
where
    Q: Eq + Debug,
{
    ValueMembershipBased::AnyOfValues(values)
}

pub fn contain_key_value<'a, K, V>(key: &'a K, value: &'a V) -> KeyValueMembershipBased<'a, K, V>
where
    K: Eq + Debug,
    V: Eq + Debug,
{
    KeyValueMembershipBased::KeyValue(key, value)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::assertions::bool::TrueFalse;
    use crate::matchers::map::membership::{
        contain_all_keys, contain_all_values, contain_any_of_keys, contain_any_of_values,
        contain_key, contain_key_value, contain_value,
    };
    use crate::matchers::Matcher;

    #[test]
    fn should_contain_a_key() {
        let mut collection = HashMap::new();
        collection.insert("rust", "assert");

        let matcher = contain_key(&"rust");
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contain_a_key_but_it_did_not() {
        let mut collection = HashMap::new();
        collection.insert("rust", "assert");

        let matcher = contain_key(&"java");
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_contain_all_keys() {
        let mut collection = HashMap::new();
        collection.insert("rust", "assert");
        collection.insert("java", "junit");

        let to_contain = vec!["rust", "java"];
        let matcher = contain_all_keys(&to_contain);
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contain_all_keys_but_it_did_not() {
        let mut collection = HashMap::new();
        collection.insert("rust", "assert");
        collection.insert("java", "junit");

        let to_contain = vec!["rust", "scala"];
        let matcher = contain_all_keys(&to_contain);
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_contain_any_of_keys() {
        let mut collection = HashMap::new();
        collection.insert("rust", "assert");
        collection.insert("java", "junit");

        let to_contain = vec!["scala", "java"];
        let matcher = contain_any_of_keys(&to_contain);
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contain_any_of_keys_but_it_did_not() {
        let mut collection = HashMap::new();
        collection.insert("rust", "assert");
        collection.insert("java", "junit");

        let to_contain = vec!["scala", "golang"];
        let matcher = contain_any_of_keys(&to_contain);
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_contain_a_value() {
        let mut collection = HashMap::new();
        collection.insert("rust", "assert");

        let matcher = contain_value(&"assert");
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contain_a_value_but_it_did_not() {
        let mut collection = HashMap::new();
        collection.insert("rust", "assert");

        let matcher = contain_key(&"java");
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_contain_all_values() {
        let mut collection = HashMap::new();
        collection.insert("rust", "assert");
        collection.insert("java", "junit");

        let matcher = contain_all_values(&["assert", "junit"]);
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contain_all_values_but_it_did_not() {
        let mut collection = HashMap::new();
        collection.insert("rust", "assert");
        collection.insert("java", "junit");

        let matcher = contain_all_values(&["assert", "xunit"]);
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_contain_any_of_values() {
        let mut collection = HashMap::new();
        collection.insert("rust", "assert");
        collection.insert("java", "junit");

        let matcher = contain_any_of_values(&["assert", "junit"]);
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contain_any_of_values_but_it_did_not() {
        let mut collection = HashMap::new();
        collection.insert("rust", "assert");
        collection.insert("java", "junit");

        let matcher = contain_any_of_values(&["catch", "xunit"]);
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_contain_a_key_value() {
        let mut collection = HashMap::new();
        collection.insert("rust", "assert");

        let matcher = contain_key_value(&"rust", &"assert");
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_contain_a_key_value_but_it_did_not() {
        let mut collection = HashMap::new();
        collection.insert("rust", "assert");

        let matcher = contain_key_value(&"rust", &"testify");
        matcher.test(&collection).passed.should_be_true();
    }
}
