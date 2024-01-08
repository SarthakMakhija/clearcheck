use std::fmt::Debug;
use std::ops::{Range, RangeInclusive};

use crate::matchers::{Matcher, MatcherResult};

pub enum RangeMatcher<'a, T: Debug> {
    Closed(&'static str, &'a RangeInclusive<T>),
    HalfOpen(&'static str, &'a Range<T>),
}

impl<'a, T> Matcher<T> for RangeMatcher<'a, T>
where
    T: PartialOrd<T> + Debug,
{
    fn test(&self, value: &T) -> MatcherResult {
        match self {
            RangeMatcher::Closed(message_prefix, range) => MatcherResult::formatted(
                range.contains(value),
                format!(
                    "{:?} {:?} should fall in the range {:?}",
                    message_prefix, value, range
                ),
                format!(
                    "{:?} {:?} should not fall in the range {:?}",
                    message_prefix, value, range
                ),
            ),
            RangeMatcher::HalfOpen(message_prefix, range) => MatcherResult::formatted(
                range.contains(value),
                format!(
                    "{:?} {:?} should fall in the range {:?}",
                    message_prefix, value, range
                ),
                format!(
                    "{:?} {:?} should not fall in the range {:?}",
                    message_prefix, value, range
                ),
            ),
        }
    }
}

pub fn be_in_inclusive_range<T: PartialOrd + Debug>(range: &RangeInclusive<T>) -> RangeMatcher<T> {
    RangeMatcher::Closed("", range)
}

pub fn be_in_exclusive_range<T: PartialOrd + Debug>(range: &Range<T>) -> RangeMatcher<T> {
    RangeMatcher::HalfOpen("", range)
}

pub fn have_length_in_inclusive_range(range: &RangeInclusive<usize>) -> RangeMatcher<usize> {
    RangeMatcher::Closed("Length", range)
}

pub fn have_length_in_exclusive_range(range: &Range<usize>) -> RangeMatcher<usize> {
    RangeMatcher::HalfOpen("Length", range)
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalseAssertion;
    use crate::matchers::range::{be_in_exclusive_range, be_in_inclusive_range};
    use crate::matchers::Matcher;

    #[test]
    fn should_be_in_inclusive_range() {
        let matcher = be_in_inclusive_range(&(1..=4));
        matcher.test(&2).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_in_inclusive_range_but_was_not() {
        let matcher = be_in_inclusive_range(&(1..=4));
        matcher.test(&5).passed.should_be_true();
    }

    #[test]
    fn should_be_in_exclusive_range() {
        let matcher = be_in_exclusive_range(&(1..4));
        matcher.test(&2).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_in_exclusive_range_but_was_not() {
        let matcher = be_in_exclusive_range(&(1..4));
        matcher.test(&4).passed.should_be_true();
    }
}
