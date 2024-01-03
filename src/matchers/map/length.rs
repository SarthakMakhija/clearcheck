use crate::matchers::length::LengthBased;
use crate::matchers::Matcher;
use std::collections::HashMap;
use std::hash::Hash;

impl<K: Eq + Hash, V> Matcher<HashMap<K, V>> for LengthBased {
    fn test(&self, collection: &HashMap<K, V>) -> bool {
        match self {
            LengthBased::Same(length) => collection.len() == *length,
            LengthBased::Atleast(length) => collection.len() >= *length,
            LengthBased::Atmost(length) => collection.len() <= *length,
        }
    }
}
