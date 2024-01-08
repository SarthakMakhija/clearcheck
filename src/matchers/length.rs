use std::collections::HashMap;
use std::hash::Hash;

use crate::matchers::MatcherResult;

pub enum LengthBased {
    Same(usize),
    Atleast(usize),
    Atmost(usize),
}

impl LengthBased {
    pub fn test_slice<T>(&self, collection: &[T]) -> MatcherResult {
        self.test(collection.len(), "Collection")
    }

    pub fn test_map<K: Hash + Eq, V>(&self, collection: &HashMap<K, V>) -> MatcherResult {
        self.test(collection.len(), "Collection")
    }

    pub fn test_string(&self, collection: &str) -> MatcherResult {
        self.test(collection.len(), "String")
    }

    fn test(&self, input_length: usize, message_prefix: &'static str) -> MatcherResult {
        match self {
            LengthBased::Same(length) => MatcherResult::formatted(
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
            LengthBased::Atleast(length) => MatcherResult::formatted(
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
            LengthBased::Atmost(length) => MatcherResult::formatted(
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

pub fn have_same_length(length: usize) -> LengthBased {
    LengthBased::Same(length)
}

pub fn have_atleast_same_length(length: usize) -> LengthBased {
    LengthBased::Atleast(length)
}

pub fn have_atmost_same_length(length: usize) -> LengthBased {
    LengthBased::Atmost(length)
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertions;
    use crate::matchers::length::{
        have_atleast_same_length, have_atmost_same_length, have_same_length,
    };

    #[test]
    fn should_have_same_length() {
        let matcher = have_same_length(4);
        matcher.test(4, "Collection").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_same_length_but_was_not() {
        let matcher = have_same_length(4);
        matcher.test(2, "Collection").passed.should_be_true();
    }

    #[test]
    fn should_have_atleast_same_length() {
        let matcher = have_atleast_same_length(4);
        matcher.test(5, "Collection").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_atleast_same_length_but_was_not() {
        let matcher = have_atleast_same_length(4);
        matcher.test(2, "Collection").passed.should_be_true();
    }

    #[test]
    fn should_have_atmost_length() {
        let matcher = have_atmost_same_length(4);
        matcher.test(3, "Collection").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_atmost_length_but_was_not() {
        let matcher = have_atmost_same_length(4);
        matcher.test(5, "Collection").passed.should_be_true();
    }
}
