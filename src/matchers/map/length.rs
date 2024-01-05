use std::collections::HashMap;
use std::hash::Hash;

use crate::matchers::length::LengthBased;
use crate::matchers::{Matcher, MatcherResult};

impl<K: Eq + Hash, V> Matcher<HashMap<K, V>> for LengthBased {
    fn test(&self, collection: &HashMap<K, V>) -> MatcherResult {
        self.test_map(collection)
    }
}
