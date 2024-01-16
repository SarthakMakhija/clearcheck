use std::fmt::Debug;

use crate::matchers::{Matcher, MatcherResult};

pub enum MinMaxMatcher<T: Ord> {
    Min(T),
    Max(T),
}

impl<T: Ord + Debug> MinMaxMatcher<T> {
    fn test(&self, collection: &[T]) -> MatcherResult {
        match self {
            MinMaxMatcher::Min(min) => MatcherResult::formatted(
                collection.iter().min() == Some(min),
                format!("{:?} should have {:?} as the minimum element", collection, min),
                format!("{:?} should not have {:?} as the minimum element", collection, min),
            ),
            MinMaxMatcher::Max(max) => MatcherResult::formatted(
                collection.iter().max() == Some(max),
                format!("{:?} should have {:?} as the maximum element", collection, max),
                format!("{:?} should not have {:?} as the maximum element", collection, max),
            ),
        }
    }
}

impl<T: Ord + Debug> Matcher<Vec<T>> for MinMaxMatcher<T> {
    fn test(&self, collection: &Vec<T>) -> MatcherResult {
        self.test(collection)
    }
}

impl<T: Ord + Debug, const N: usize> Matcher<[T; N]> for MinMaxMatcher<T> {
    fn test(&self, collection: &[T; N]) -> MatcherResult {
        self.test(collection as &[T])
    }
}

impl<T: Ord + Debug> Matcher<&[T]> for MinMaxMatcher<T> {
    fn test(&self, collection: &&[T]) -> MatcherResult {
        self.test(collection)
    }
}

pub fn have_min<T: Ord>(min: T) -> MinMaxMatcher<T> {
    MinMaxMatcher::Min(min)
}

pub fn have_max<T: Ord>(max: T) -> MinMaxMatcher<T> {
    MinMaxMatcher::Max(max)
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::collection::min_max::{have_max, have_min};

    #[test]
    fn should_a_min_element() {
        let collection = vec!["assert", "clearcheck", "junit"];
        let matcher = have_min("assert");

        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_the_given_min_element_but_was_not() {
        let collection = vec!["assert", "clearcheck", "junit"];
        let matcher = have_min("junit");

        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_a_max_element() {
        let collection = vec!["assert", "clearcheck", "junit"];
        let matcher = have_max("junit");

        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_have_the_given_max_element_but_was_not() {
        let collection = vec!["assert", "clearcheck", "junit"];
        let matcher = have_max("clearcheck");

        matcher.test(&collection).passed.should_be_true();
    }
}