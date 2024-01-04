use std::collections::HashMap;
use std::hash::Hash;

use crate::matchers::Matcher;

pub enum KeyMembershipBased<'a, T> {
    Key(&'a T),
}

pub enum ValueMembershipBased<'a, T> {
    Value(&'a T),
}

pub enum KeyValueMembershipBased<'a, K, V> {
    KeyValue(&'a K, &'a V),
}

impl<K, V> Matcher<HashMap<K, V>> for KeyMembershipBased<'_, K>
where
    K: Hash + Eq,
{
    fn test(&self, collection: &HashMap<K, V>) -> bool {
        match self {
            KeyMembershipBased::Key(key) => collection.contains_key(key),
        }
    }
}

impl<K, V> Matcher<HashMap<K, V>> for ValueMembershipBased<'_, V>
where
    K: Hash + Eq,
    V: Eq,
{
    fn test(&self, collection: &HashMap<K, V>) -> bool {
        match self {
            ValueMembershipBased::Value(value) => {
                collection.values().any(|source| &source == value)
            }
        }
    }
}

impl<K, V> Matcher<HashMap<K, V>> for KeyValueMembershipBased<'_, K, V>
where
    K: Hash + Eq,
    V: Eq,
{
    fn test(&self, collection: &HashMap<K, V>) -> bool {
        return match self {
            KeyValueMembershipBased::KeyValue(key, value) => collection
                .get(key)
                .filter(|source| source == value)
                .is_some(),
        };
    }
}

pub fn contain_key<Q>(key: &Q) -> KeyMembershipBased<'_, Q>
where
    Q: Hash + Eq,
{
    KeyMembershipBased::Key(&key)
}

pub fn contain_value<Q>(value: &Q) -> ValueMembershipBased<'_, Q>
where
    Q: Eq,
{
    ValueMembershipBased::Value(value)
}

pub fn contain_key_value<'a, K, V>(key: &'a K, value: &'a V) -> KeyValueMembershipBased<'a, K, V>
where
    K: Eq,
    V: Eq,
{
    KeyValueMembershipBased::KeyValue(key, value)
}
