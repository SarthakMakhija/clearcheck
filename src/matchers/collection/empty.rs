use crate::matchers::{Matcher, MatcherResult};

/// CollectionEmptyMatcher offers a flexible way to assert whether a collection is empty.
///
/// clearcheck implements CollectionEmptyMatcher for collection types including vector, arrays and slices.
///
/// # Example
///```
/// use clearcheck::matchers::collection::empty::be_empty;
/// use clearcheck::matchers::Matcher;
///
/// let collection: Vec<i32> = vec![];
/// let matcher = be_empty();
///
/// assert!(matcher.test(&collection).passed());
/// ```
pub enum CollectionEmptyMatcher {
    Empty,
    NotEmpty,
}

impl<T> Matcher<Vec<T>> for CollectionEmptyMatcher {
    fn test(&self, collection: &Vec<T>) -> MatcherResult {
        self.test_length(collection)
    }
}

impl<T, const N: usize> Matcher<[T; N]> for CollectionEmptyMatcher {
    fn test(&self, collection: &[T; N]) -> MatcherResult {
        self.test_length(collection as &[T])
    }
}

impl<T> Matcher<&[T]> for CollectionEmptyMatcher {
    fn test(&self, collection: &&[T]) -> MatcherResult {
        self.test_length(collection)
    }
}

impl CollectionEmptyMatcher {
    pub fn test_length<T>(&self, collection: &[T]) -> MatcherResult {
        match self {
            CollectionEmptyMatcher::Empty => MatcherResult::new(
                collection.is_empty(),
                "Collection should be empty",
                "Collection should not be empty",
            ),
            CollectionEmptyMatcher::NotEmpty => MatcherResult::new(
                !collection.is_empty(),
                "Collection should not be empty",
                "Collection should be empty",
            ),
        }
    }
}

/// Creates a CollectionEmptyMatcher that asserts whether the underlying collection is empty.
pub fn be_empty() -> CollectionEmptyMatcher {
    CollectionEmptyMatcher::Empty
}

/// Creates a CollectionEmptyMatcher that asserts whether the underlying collection is not empty.
pub fn not_be_empty() -> CollectionEmptyMatcher {
    CollectionEmptyMatcher::NotEmpty
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