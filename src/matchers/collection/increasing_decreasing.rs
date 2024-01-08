use std::fmt::Debug;

use crate::matchers::{Matcher, MatcherResult};

pub enum IncreasingDecreasingBased {
    MonotonicallyIncreasing,
    MonotonicallyDecreasing,
    StrictlyIncreasing,
    StrictlyDecreasing,
}

impl IncreasingDecreasingBased {
    fn test<T: PartialOrd + Debug>(&self, collection: &[T]) -> MatcherResult {
        match self {
            IncreasingDecreasingBased::MonotonicallyIncreasing => MatcherResult::formatted(
                collection.windows(2).all(|window| window[0] <= window[1]),
                format!("{:?} should be monotonically increasing", collection),
                format!("{:?} should not be monotonically increasing", collection),
            ),
            IncreasingDecreasingBased::MonotonicallyDecreasing => MatcherResult::formatted(
                collection.windows(2).all(|window| window[0] >= window[1]),
                format!("{:?} should be monotonically decreasing", collection),
                format!("{:?} should not be monotonically decreasing", collection),
            ),
            IncreasingDecreasingBased::StrictlyIncreasing => MatcherResult::formatted(
                collection.windows(2).all(|window| window[0] < window[1]),
                format!("{:?} should be strictly increasing", collection),
                format!("{:?} should not be strictly increasing", collection),
            ),
            IncreasingDecreasingBased::StrictlyDecreasing => MatcherResult::formatted(
                collection.windows(2).all(|window| window[0] > window[1]),
                format!("{:?} should be strictly decreasing", collection),
                format!("{:?} should not be strictly decreasing", collection),
            ),
        }
    }
}

impl<T: PartialOrd + Debug> Matcher<Vec<T>> for IncreasingDecreasingBased {
    fn test(&self, collection: &Vec<T>) -> MatcherResult {
        self.test(&collection)
    }
}

impl<T: PartialOrd + Debug, const N: usize> Matcher<[T; N]> for IncreasingDecreasingBased {
    fn test(&self, collection: &[T; N]) -> MatcherResult {
        self.test(collection as &[T])
    }
}

impl<T: PartialOrd + Debug> Matcher<&[T]> for IncreasingDecreasingBased {
    fn test(&self, collection: &&[T]) -> MatcherResult {
        self.test(&collection)
    }
}

pub fn be_monotonically_increasing() -> IncreasingDecreasingBased {
    IncreasingDecreasingBased::MonotonicallyIncreasing
}

pub fn be_monotonically_decreasing() -> IncreasingDecreasingBased {
    IncreasingDecreasingBased::MonotonicallyDecreasing
}

pub fn be_strictly_increasing() -> IncreasingDecreasingBased {
    IncreasingDecreasingBased::StrictlyIncreasing
}

pub fn be_strictly_decreasing() -> IncreasingDecreasingBased {
    IncreasingDecreasingBased::StrictlyDecreasing
}

#[cfg(test)]
mod test {
    use crate::assertions::bool::TrueFalseAssertions;
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
