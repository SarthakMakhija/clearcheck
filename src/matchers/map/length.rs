use std::collections::HashMap;
use std::hash::Hash;

use crate::matchers::length::LengthMatcher;
use crate::matchers::{Matcher, MatcherResult};

impl<K: Eq + Hash, V> Matcher<HashMap<K, V>> for LengthMatcher {
    fn test(&self, collection: &HashMap<K, V>) -> MatcherResult {
        self.test_map(collection)
    }
}
