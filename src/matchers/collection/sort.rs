use std::fmt::Debug;

use crate::matchers::{Matcher, MatcherResult};

/// SortMatcher offers a flexible way to assert whether a collection is sorted in ascending or descending order.
///
/// clearcheck implements SortMatcher for collection types including vector, arrays and slices.
///
/// # Example
///```
/// use clearcheck::matchers::collection::sort::{be_sorted_ascending, SortMatcher};
/// use clearcheck::matchers::Matcher;
///
/// let matcher = be_sorted_ascending();
/// let collection = vec!["clearcheck", "junit"];
///
/// assert!(matcher.test(&collection).passed());
/// ```
pub enum SortMatcher {
    Ascending,
    Descending,
}

impl SortMatcher {
    fn test<T: PartialOrd + Debug>(&self, collection: &[T]) -> MatcherResult {
        match self {
            SortMatcher::Ascending => MatcherResult::formatted(
                (0..collection.len() - 1).all(|index| collection[index] <= collection[index + 1]),
                format!("{:?} should be sorted ascending", collection),
                format!("{:?} should not be sorted ascending", collection),
            ),
            SortMatcher::Descending => MatcherResult::formatted(
                (0..collection.len() - 1).all(|index| collection[index] >= collection[index + 1]),
                format!("{:?} should be sorted descending", collection),
                format!("{:?} should not be sorted descending", collection),
            ),
        }
    }
}

impl<T: PartialOrd + Debug> Matcher<Vec<T>> for SortMatcher {
    fn test(&self, collection: &Vec<T>) -> MatcherResult {
        self.test(collection)
    }
}

impl<T: PartialOrd + Debug, const N: usize> Matcher<[T; N]> for SortMatcher {
    fn test(&self, collection: &[T; N]) -> MatcherResult {
        self.test(collection as &[T])
    }
}

impl<T: PartialOrd + Debug> Matcher<&[T]> for SortMatcher {
    fn test(&self, collection: &&[T]) -> MatcherResult {
        self.test(collection)
    }
}

/// Creates an SortMatcher that asserts whether the elements in a collection are sorted in ascending order.
pub fn be_sorted_ascending() -> SortMatcher {
    SortMatcher::Ascending
}

/// Creates an SortMatcher that asserts whether the elements in a collection are sorted in descending order.
pub fn be_sorted_descending() -> SortMatcher {
    SortMatcher::Descending
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::collection::sort::{be_sorted_ascending, be_sorted_descending};

    #[test]
    fn should_be_sorted_ascending() {
        let matcher = be_sorted_ascending();
        let collection = vec!["assert4j", "junit"];
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_sorted_ascending_but_was_not() {
        let matcher = be_sorted_ascending();
        let collection = vec!["junit", "assert4j"];
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_be_sorted_descending() {
        let matcher = be_sorted_descending();
        let collection = vec!["junit", "assert4j"];
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_sorted_descending_but_was_not() {
        let matcher = be_sorted_descending();
        let collection = vec!["assert4j", "junit"];
        matcher.test(&collection).passed.should_be_true();
    }
}
