use std::collections::HashMap;
use std::hash::Hash;

use crate::matchers::MatcherResult;

pub enum EmptyBased {
    Empty,
    NotEmpty,
}

impl EmptyBased {
    pub fn test_slice<T>(&self, collection: &[T]) -> MatcherResult {
        match self {
            EmptyBased::Empty => MatcherResult::new(
                collection.len() == 0,
                "Collection should be empty",
                "Collection should not be empty",
            ),
            EmptyBased::NotEmpty => MatcherResult::new(
                collection.len() != 0,
                "Collection should not be empty",
                "Collection should be empty",
            ),
        }
    }

    pub fn test_map<K: Hash + Eq, V>(&self, collection: &HashMap<K, V>) -> MatcherResult {
        match self {
            EmptyBased::Empty => MatcherResult::new(
                collection.len() == 0,
                "Collection should be empty",
                "Collection should not be empty",
            ),
            EmptyBased::NotEmpty => MatcherResult::new(
                collection.len() != 0,
                "Collection should not be empty",
                "Collection should be empty",
            ),
        }
    }

    pub fn test_string(&self, value: &str) -> MatcherResult {
        match self {
            EmptyBased::Empty => MatcherResult::new(
                value.len() == 0,
                "Value should be empty",
                "Value should not be empty",
            ),
            EmptyBased::NotEmpty => MatcherResult::new(
                value.len() != 0,
                "Value should not be empty",
                "Value should be empty",
            ),
        }
    }
}

pub fn be_empty() -> EmptyBased {
    EmptyBased::Empty
}

pub fn not_be_empty() -> EmptyBased {
    EmptyBased::NotEmpty
}

#[cfg(test)]
mod collection_tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::empty::{be_empty, not_be_empty};

    #[test]
    fn should_be_empty() {
        let collection: Vec<i32> = vec![];
        let matcher = be_empty();
        matcher.test_slice(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_empty_but_was_not() {
        let collection = vec![1, 2, 3];
        let matcher = be_empty();
        matcher.test_slice(&collection).passed.should_be_true();
    }

    #[test]
    fn should_not_be_empty() {
        let collection = vec![1, 2, 3];
        let matcher = not_be_empty();
        matcher.test_slice(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_not_be_empty_but_was() {
        let collection: Vec<i32> = vec![];
        let matcher = not_be_empty();
        matcher.test_slice(&collection).passed.should_be_true();
    }
}

#[cfg(test)]
mod map_tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::empty::{be_empty, not_be_empty};
    use std::collections::HashMap;

    #[test]
    fn should_be_empty() {
        let key_value: HashMap<i32, i32> = HashMap::new();
        let matcher = be_empty();
        matcher.test_map(&key_value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_empty_but_was_not() {
        let mut key_value: HashMap<&str, &str> = HashMap::new();
        key_value.insert("java", "junit");

        let matcher = be_empty();
        matcher.test_map(&key_value).passed.should_be_true();
    }

    #[test]
    fn should_not_be_empty() {
        let mut key_value: HashMap<&str, &str> = HashMap::new();
        key_value.insert("java", "junit");

        let matcher = not_be_empty();
        matcher.test_map(&key_value).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_not_be_empty_but_was() {
        let key_value: HashMap<&str, &str> = HashMap::new();

        let matcher = not_be_empty();
        matcher.test_map(&key_value).passed.should_be_true();
    }
}

#[cfg(test)]
mod string_tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::empty::{be_empty, not_be_empty};

    #[test]
    fn should_be_empty() {
        let matcher = be_empty();
        matcher.test_string(&"").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_empty_but_was_not() {
        let matcher = be_empty();
        matcher.test_string(&"goselect").passed.should_be_true();
    }

    #[test]
    fn should_not_be_empty() {
        let matcher = not_be_empty();
        matcher.test_string(&"goselect").passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_not_be_empty_but_was() {
        let matcher = not_be_empty();
        matcher.test_string(&"").passed.should_be_true();
    }
}
