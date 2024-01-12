use std::fmt::Debug;
use std::marker::PhantomData;

use crate::matchers::{Matcher, MatcherResult};

pub enum IncreasingDecreasingMatcher<T: PartialOrd + Debug> {
    MonotonicallyIncreasing(PhantomData<T>),
    MonotonicallyDecreasing(PhantomData<T>),
    StrictlyIncreasing(PhantomData<T>),
    StrictlyDecreasing(PhantomData<T>),
}

impl<T: PartialOrd + Debug> IncreasingDecreasingMatcher<T> {
    fn test(&self, collection: &[T]) -> MatcherResult {
        match self {
            IncreasingDecreasingMatcher::MonotonicallyIncreasing(_) => MatcherResult::formatted(
                collection.windows(2).all(|window| window[0] <= window[1]),
                format!("{:?} should be monotonically increasing", collection),
                format!("{:?} should not be monotonically increasing", collection),
            ),
            IncreasingDecreasingMatcher::MonotonicallyDecreasing(_) => MatcherResult::formatted(
                collection.windows(2).all(|window| window[0] >= window[1]),
                format!("{:?} should be monotonically decreasing", collection),
                format!("{:?} should not be monotonically decreasing", collection),
            ),
            IncreasingDecreasingMatcher::StrictlyIncreasing(_) => MatcherResult::formatted(
                collection.windows(2).all(|window| window[0] < window[1]),
                format!("{:?} should be strictly increasing", collection),
                format!("{:?} should not be strictly increasing", collection),
            ),
            IncreasingDecreasingMatcher::StrictlyDecreasing(_) => MatcherResult::formatted(
                collection.windows(2).all(|window| window[0] > window[1]),
                format!("{:?} should be strictly decreasing", collection),
                format!("{:?} should not be strictly decreasing", collection),
            ),
        }
    }
}

impl<T: PartialOrd + Debug> Matcher<Vec<T>> for IncreasingDecreasingMatcher<T> {
    fn test(&self, collection: &Vec<T>) -> MatcherResult {
        self.test(collection)
    }
}

impl<T: PartialOrd + Debug, const N: usize> Matcher<[T; N]> for IncreasingDecreasingMatcher<T> {
    fn test(&self, collection: &[T; N]) -> MatcherResult {
        self.test(collection as &[T])
    }
}

impl<T: PartialOrd + Debug> Matcher<&[T]> for IncreasingDecreasingMatcher<T> {
    fn test(&self, collection: &&[T]) -> MatcherResult {
        self.test(collection)
    }
}

pub fn be_monotonically_increasing<T: PartialOrd + Debug>() -> IncreasingDecreasingMatcher<T> {
    IncreasingDecreasingMatcher::MonotonicallyIncreasing(PhantomData)
}

pub fn be_monotonically_decreasing<T: PartialOrd + Debug>() -> IncreasingDecreasingMatcher<T> {
    IncreasingDecreasingMatcher::MonotonicallyDecreasing(PhantomData)
}

pub fn be_strictly_increasing<T: PartialOrd + Debug>() -> IncreasingDecreasingMatcher<T> {
    IncreasingDecreasingMatcher::StrictlyIncreasing(PhantomData)
}

pub fn be_strictly_decreasing<T: PartialOrd + Debug>() -> IncreasingDecreasingMatcher<T> {
    IncreasingDecreasingMatcher::StrictlyDecreasing(PhantomData)
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
