use std::collections::HashMap;
use std::hash::Hash;

use crate::matchers::{Matcher, MatcherResult};

pub enum MapLengthMatcher {
    Same(usize),
    Atleast(usize),
    Atmost(usize),
}

impl<K: Hash + Eq, V> Matcher<HashMap<K, V>> for MapLengthMatcher {
    fn test(&self, collection: &HashMap<K, V>) -> MatcherResult {
        self.test_length(collection.len())
    }
}

impl MapLengthMatcher {
    fn test_length(&self, input_length: usize) -> MatcherResult {
        let message_prefix = "Map";
        match self {
            MapLengthMatcher::Same(length) => MatcherResult::formatted(
                input_length == *length,
                format!(
                    "{:?} length {:?} should be {:?}",
                    message_prefix, input_length, length
                ),
                format!(
                    "{:?} length {:?} should not be {:?}",
                    message_prefix, input_length, length
                ),
            ),
            MapLengthMatcher::Atleast(length) => MatcherResult::formatted(
                input_length >= *length,
                format!(
                    "{:?} length {:?} should be atleast {:?}",
                    message_prefix, input_length, length
                ),
                format!(
                    "{:?} length {:?} should not be atleast {:?}",
                    message_prefix, input_length, length
                ),
            ),
            MapLengthMatcher::Atmost(length) => MatcherResult::formatted(
                input_length <= *length,
                format!(
                    "{:?} length {:?} should be atmost {:?}",
                    message_prefix, input_length, length
                ),
                format!(
                    "{:?} length {:?} should not be atmost {:?}",
                    message_prefix, input_length, length
                ),
            ),
        }
    }
}

pub fn have_same_length(length: usize) -> MapLengthMatcher {
    MapLengthMatcher::Same(length)
}

pub fn have_atleast_same_length(length: usize) -> MapLengthMatcher {
    MapLengthMatcher::Atleast(length)
}

pub fn have_atmost_same_length(length: usize) -> MapLengthMatcher {
    MapLengthMatcher::Atmost(length)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::map::length::{have_atleast_same_length, have_atmost_same_length, have_same_length};
    use crate::matchers::Matcher;

    #[test]
    fn should_have_same_length() {
        let mut key_value = HashMap::new();
        key_value.insert(1, 10);
        key_value.insert(2, 20);

        let matcher = have_same_length(2);
        matcher.test(&key_value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_same_length_but_was_not() {
        let mut key_value = HashMap::new();
        key_value.insert(1, 10);
        key_value.insert(2, 20);

        let matcher = have_same_length(5);
        matcher.test(&key_value).passed.should_be_true();
    }

    #[test]
    fn should_have_atleast_same_length() {
        let mut key_value = HashMap::new();
        key_value.insert(1, 10);
        key_value.insert(2, 20);

        let matcher = have_atleast_same_length(2);
        matcher.test(&key_value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_atleast_same_length_but_was_not() {
        let mut key_value = HashMap::new();
        key_value.insert(1, 10);
        key_value.insert(2, 20);

        let matcher = have_atleast_same_length(6);
        matcher.test(&key_value).passed.should_be_true();
    }

    #[test]
    fn should_have_atmost_length() {
        let mut key_value = HashMap::new();
        key_value.insert(1, 10);
        key_value.insert(2, 20);

        let matcher = have_atmost_same_length(2);
        matcher.test(&key_value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_atmost_length_but_was_not() {
        let mut key_value = HashMap::new();
        key_value.insert(1, 10);
        key_value.insert(2, 20);

        let matcher = have_atmost_same_length(1);
        matcher.test(&key_value).passed.should_be_true();
    }
}
