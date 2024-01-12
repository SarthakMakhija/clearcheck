use std::marker::PhantomData;
use crate::matchers::{Matcher, MatcherResult};

pub enum CollectionEmptyMatcher<T> {
    Empty(PhantomData<T>),
    NotEmpty(PhantomData<T>),
}

impl<T> Matcher<Vec<T>> for CollectionEmptyMatcher<T> {
    fn test(&self, collection: &Vec<T>) -> MatcherResult {
        self.test_length(collection)
    }
}

impl<T, const N: usize> Matcher<[T; N]> for CollectionEmptyMatcher<T> {
    fn test(&self, collection: &[T; N]) -> MatcherResult {
        self.test_length(collection as &[T])
    }
}

impl<T> Matcher<&[T]> for CollectionEmptyMatcher<T> {
    fn test(&self, collection: &&[T]) -> MatcherResult {
        self.test_length(collection)
    }
}

impl<T> CollectionEmptyMatcher<T> {
    pub fn test_length(&self, collection: &[T]) -> MatcherResult {
        match self {
            CollectionEmptyMatcher::Empty(_) => MatcherResult::new(
                collection.is_empty(),
                "Collection should be empty",
                "Collection should not be empty",
            ),
            CollectionEmptyMatcher::NotEmpty(_) => MatcherResult::new(
                !collection.is_empty(),
                "Collection should not be empty",
                "Collection should be empty",
            ),
        }
    }
}

pub fn be_empty<T>() -> CollectionEmptyMatcher<T> {
    CollectionEmptyMatcher::Empty(PhantomData)
}

pub fn not_be_empty<T>() -> CollectionEmptyMatcher<T> {
    CollectionEmptyMatcher::NotEmpty(PhantomData)
}

#[cfg(test)]
mod collection_tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::collection::empty::{be_empty, not_be_empty};
    use crate::matchers::Matcher;

    #[test]
    fn should_be_empty() {
        let collection: Vec<i32> = vec![];
        let matcher = be_empty();
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_empty_but_was_not() {
        let collection = vec![1, 2, 3];
        let matcher = be_empty();
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_not_be_empty() {
        let collection = vec![1, 2, 3];
        let matcher = not_be_empty();
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_not_be_empty_but_was() {
        let collection: Vec<i32> = vec![];
        let matcher = not_be_empty();
        matcher.test(&collection).passed.should_be_true();
    }
}