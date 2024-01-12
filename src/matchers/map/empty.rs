use std::collections::HashMap;
use std::hash::Hash;

use crate::matchers::{Matcher, MatcherResult};

pub enum MapEmptyMatcher {
    Empty,
    NotEmpty,
}

impl<K: Hash + Eq, V> Matcher<HashMap<K, V>> for MapEmptyMatcher {
    fn test(&self, collection: &HashMap<K, V>) -> MatcherResult {
        match self {
            MapEmptyMatcher::Empty => MatcherResult::new(
                collection.is_empty(),
                "Map should be empty",
                "Map should not be empty",
            ),
            MapEmptyMatcher::NotEmpty => MatcherResult::new(
                !collection.is_empty(),
                "Map should not be empty",
                "Map should be empty",
            ),
        }
    }
}


pub fn be_empty() -> MapEmptyMatcher {
    MapEmptyMatcher::Empty
}

pub fn not_be_empty() -> MapEmptyMatcher {
    MapEmptyMatcher::NotEmpty
}

#[cfg(test)]
mod map_tests {
    use std::collections::HashMap;

    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::map::empty::{be_empty, not_be_empty};
    use crate::matchers::Matcher;

    #[test]
    fn should_be_empty() {
        let key_value: HashMap<i32, i32> = HashMap::new();
        let matcher = be_empty();
        matcher.test(&key_value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_empty_but_was_not() {
        let mut key_value: HashMap<&str, &str> = HashMap::new();
        key_value.insert("java", "junit");

        let matcher = be_empty();
        matcher.test(&key_value).passed.should_be_true();
    }

    #[test]
    fn should_not_be_empty() {
        let mut key_value: HashMap<&str, &str> = HashMap::new();
        key_value.insert("java", "junit");

        let matcher = not_be_empty();
        matcher.test(&key_value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_not_be_empty_but_was() {
        let key_value: HashMap<&str, &str> = HashMap::new();

        let matcher = not_be_empty();
        matcher.test(&key_value).passed.should_be_true();
    }
}