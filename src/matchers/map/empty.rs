use std::collections::HashMap;
use std::hash::Hash;

use crate::matchers::empty::EmptyBased;
use crate::matchers::{Matcher, MatcherResult};

impl<K: Hash + Eq, V> Matcher<HashMap<K, V>> for EmptyBased {
    fn test(&self, collection: &HashMap<K, V>) -> MatcherResult {
        self.test_map(collection)
    }
}
