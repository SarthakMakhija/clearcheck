use crate::matchers::length::LengthBased;
use crate::matchers::{Matcher, MatcherResult};

impl<T> Matcher<Vec<T>> for LengthBased {
    fn test(&self, collection: &Vec<T>) -> MatcherResult {
        self.test(collection.len())
    }
}

impl<T, const N: usize> Matcher<[T; N]> for LengthBased {
    fn test(&self, collection: &[T; N]) -> MatcherResult {
        self.test(collection.len())
    }
}

impl<T> Matcher<&[T]> for LengthBased {
    fn test(&self, collection: &&[T]) -> MatcherResult {
        self.test(collection.len())
    }
}
