use std::fmt::Debug;
use std::ops::{Range, RangeInclusive};

use crate::matchers::{Matcher, MatcherResult};

pub enum MinMaxMatcher<T: Ord> {
    Min(T),
    Max(T),
    MinInInclusiveRange(RangeInclusive<T>),
    MinInExclusiveRange(Range<T>),
    MaxInInclusiveRange(RangeInclusive<T>),
    MaxInExclusiveRange(Range<T>),
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
            MinMaxMatcher::MinInInclusiveRange(range) => MatcherResult::formatted(
                collection.iter().min().is_some_and(|min| range.contains(min)),
                format!("{:?} should have minimum in the range {:?}", collection, range),
                format!("{:?} should not have minimum in the range {:?}", collection, range),
            ),
            MinMaxMatcher::MinInExclusiveRange(range) => MatcherResult::formatted(
                collection.iter().min().is_some_and(|min| range.contains(min)),
                format!("{:?} should have minimum in the range {:?}", collection, range),
                format!("{:?} should not have minimum in the range {:?}", collection, range),
            ),
            MinMaxMatcher::MaxInInclusiveRange(range) => MatcherResult::formatted(
                collection.iter().max().is_some_and(|max| range.contains(max)),
                format!("{:?} should have maximum in the range {:?}", collection, range),
                format!("{:?} should not have maximum in the range {:?}", collection, range),
            ),
            MinMaxMatcher::MaxInExclusiveRange(range) => MatcherResult::formatted(
                collection.iter().max().is_some_and(|max| range.contains(max)),
                format!("{:?} should have maximum in the range {:?}", collection, range),
                format!("{:?} should not have maximum in the range {:?}", collection, range),
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

pub fn have_min_in_inclusive_range<T: Ord>(range: RangeInclusive<T>) -> MinMaxMatcher<T> {
    MinMaxMatcher::MinInInclusiveRange(range)
}

pub fn have_min_in_exclusive_range<T: Ord>(range: Range<T>) -> MinMaxMatcher<T> {
    MinMaxMatcher::MinInExclusiveRange(range)
}

pub fn have_max_in_inclusive_range<T: Ord>(range: RangeInclusive<T>) -> MinMaxMatcher<T> {
    MinMaxMatcher::MaxInInclusiveRange(range)
}

pub fn have_max_in_exclusive_range<T: Ord>(range: Range<T>) -> MinMaxMatcher<T> {
    MinMaxMatcher::MaxInExclusiveRange(range)
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

#[cfg(test)]
mod range_tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::collection::min_max::{have_max_in_exclusive_range, have_max_in_inclusive_range, have_min_in_exclusive_range, have_min_in_inclusive_range};

    #[test]
    fn should_a_min_in_inclusive_range() {
        let collection = vec!["assert", "clearcheck", "junit"];
        let matcher = have_min_in_inclusive_range("assert"..="junit");

        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_a_min_in_exclusive_range() {
        let collection = vec!["assert", "clearcheck", "junit"];
        let matcher = have_min_in_exclusive_range("assert".."junit");

        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_a_max_in_inclusive_range() {
        let collection = vec!["assert", "clearcheck", "junit"];
        let matcher = have_max_in_inclusive_range("assert"..="junit");

        matcher.test(&collection).passed.should_be_true();
    }

    #[test]
    fn should_a_max_in_exclusive_range() {
        let collection = vec!["assert", "clearcheck", "junit"];
        let matcher = have_max_in_exclusive_range("assert".."testify");

        matcher.test(&collection).passed.should_be_true();
    }
}