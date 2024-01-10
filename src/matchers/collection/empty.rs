use crate::matchers::empty::EmptyMatcher;
use crate::matchers::{Matcher, MatcherResult};

impl<T> Matcher<Vec<T>> for EmptyMatcher {
    fn test(&self, collection: &Vec<T>) -> MatcherResult {
        self.test_slice(collection)
    }
}

impl<T, const N: usize> Matcher<[T; N]> for EmptyMatcher {
    fn test(&self, collection: &[T; N]) -> MatcherResult {
        self.test_slice(collection as &[T])
    }
}

impl<T> Matcher<&[T]> for EmptyMatcher {
    fn test(&self, collection: &&[T]) -> MatcherResult {
        self.test_slice(collection)
    }
}
