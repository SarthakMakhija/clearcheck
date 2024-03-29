use std::fmt::Debug;

use crate::matchers::{Matcher, MatcherResult};

/// OrderedMatcher offers a flexible way to assert whether a sequence of values exhibits a specific pattern of increasing or decreasing behavior.
///
/// clearcheck implements OrderedMatcher for collection types including vector, arrays and reference to slices.
///
/// # Example
///```
/// use clearcheck::matchers::collection::increasing_decreasing::be_monotonically_increasing;
/// use clearcheck::matchers::Matcher;
///
/// let matcher = be_monotonically_increasing();
/// let collection = vec![1, 2, 3, 4, 4, 5, 5, 6];
///
/// assert!(matcher.test(&collection).passed());
/// ```
pub enum IncreasingDecreasingMatcher {
    MonotonicallyIncreasing,
    MonotonicallyDecreasing,
    StrictlyIncreasing,
    StrictlyDecreasing,
}

impl IncreasingDecreasingMatcher {
    fn test<T: PartialOrd + Debug>(&self, collection: &[T]) -> MatcherResult {
        match self {
            IncreasingDecreasingMatcher::MonotonicallyIncreasing => MatcherResult::formatted(
                collection.windows(2).all(|window| window[0] <= window[1]),
                format!("{:?} should be monotonically increasing", collection),
                format!("{:?} should not be monotonically increasing", collection),
            ),
            IncreasingDecreasingMatcher::MonotonicallyDecreasing => MatcherResult::formatted(
                collection.windows(2).all(|window| window[0] >= window[1]),
                format!("{:?} should be monotonically decreasing", collection),
                format!("{:?} should not be monotonically decreasing", collection),
            ),
            IncreasingDecreasingMatcher::StrictlyIncreasing => MatcherResult::formatted(
                collection.windows(2).all(|window| window[0] < window[1]),
                format!("{:?} should be strictly increasing", collection),
                format!("{:?} should not be strictly increasing", collection),
            ),
            IncreasingDecreasingMatcher::StrictlyDecreasing => MatcherResult::formatted(
                collection.windows(2).all(|window| window[0] > window[1]),
                format!("{:?} should be strictly decreasing", collection),
                format!("{:?} should not be strictly decreasing", collection),
            ),
        }
    }
}

impl<T: PartialOrd + Debug> Matcher<Vec<T>> for IncreasingDecreasingMatcher {
    fn test(&self, collection: &Vec<T>) -> MatcherResult {
        self.test(collection)
    }
}

impl<T: PartialOrd + Debug, const N: usize> Matcher<[T; N]> for IncreasingDecreasingMatcher {
    fn test(&self, collection: &[T; N]) -> MatcherResult {
        self.test(collection as &[T])
    }
}

impl<T: PartialOrd + Debug> Matcher<&[T]> for IncreasingDecreasingMatcher {
    fn test(&self, collection: &&[T]) -> MatcherResult {
        self.test(collection)
    }
}

/// Creates an IncreasingDecreasingMatcher that asserts whether the elements in a collection are in non-decreasing order (allowing consecutive equal elements).
pub fn be_monotonically_increasing() -> IncreasingDecreasingMatcher {
    IncreasingDecreasingMatcher::MonotonicallyIncreasing
}

/// Creates an IncreasingDecreasingMatcher that asserts that the elements in a collection are in non-increasing order (allowing consecutive equal elements).
pub fn be_monotonically_decreasing() -> IncreasingDecreasingMatcher {
    IncreasingDecreasingMatcher::MonotonicallyDecreasing
}

/// Creates an IncreasingDecreasingMatcher that asserts that the elements in a collection are in strictly increasing order (no consecutive elements can be equal).
pub fn be_strictly_increasing() -> IncreasingDecreasingMatcher {
    IncreasingDecreasingMatcher::StrictlyIncreasing
}

/// Creates an IncreasingDecreasingMatcher that asserts that the elements in a collection are in strictly decreasing order (no consecutive elements can be equal).
pub fn be_strictly_decreasing() -> IncreasingDecreasingMatcher {
    IncreasingDecreasingMatcher::StrictlyDecreasing
}

#[cfg(test)]
mod test {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::collection::increasing_decreasing::{
        be_monotonically_decreasing, be_monotonically_increasing, be_strictly_decreasing,
        be_strictly_increasing,
    };

    #[test]
    fn should_be_monotonically_increasing() {
        let matcher = be_monotonically_increasing();
        let collection = vec![1, 2, 3, 4, 4, 5, 5, 6];
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_monotonically_increasing_but_was_not() {
        let matcher = be_monotonically_increasing();
        let collection = vec![1, 2, 3, 4, 4, 2, 5, 6];
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_be_monotonically_decreasing() {
        let matcher = be_monotonically_decreasing();
        let collection = vec![6, 6, 5, 5, 4, 4, 2];
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_monotonically_decreasing_but_was_not() {
        let matcher = be_monotonically_increasing();
        let collection = vec![6, 6, 5, 5, 4, 4, 5];
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_be_strictly_increasing() {
        let matcher = be_strictly_increasing();
        let collection = vec![1, 3, 5, 7, 9];
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_strictly_increasing_but_was_not() {
        let matcher = be_strictly_increasing();
        let collection = vec![1, 5, 7, 9, 9];
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_be_strictly_decreasing() {
        let matcher = be_strictly_decreasing();
        let collection = vec![9, 7, 5, 3, 1];
        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_strictly_decreasing_but_was_not() {
        let matcher = be_strictly_decreasing();
        let collection = vec![9, 7, 5, 3, 1, 1];
        matcher.test(&collection).passed.should_be_true();
    }
}
