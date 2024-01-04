use std::fmt::Debug;
use std::ops::{Range, RangeInclusive};

use crate::matchers::{Matcher, MatcherResult};

pub enum RangeBased<'a, T: Debug> {
    Closed(&'a RangeInclusive<T>),
    HalfOpen(&'a Range<T>),
}

impl<'a, T> Matcher<T> for RangeBased<'a, T>
where
    T: PartialOrd<T> + Debug,
{
    fn test(&self, value: &T) -> MatcherResult {
        match self {
            RangeBased::Closed(range) => MatcherResult::formatted(
                range.contains(value),
                format!("{:?} should contain the value {:?}", range, value),
                format!("{:?} should not contain the value {:?}", range, value),
            ),
            RangeBased::HalfOpen(range) => MatcherResult::formatted(
                range.contains(value),
                format!("{:?} should contain the value {:?}", range, value),
                format!("{:?} should not contain the value {:?}", range, value),
            ),
        }
    }
}

pub fn be_in_inclusive_range<T: PartialOrd + Debug>(range: &RangeInclusive<T>) -> RangeBased<T> {
    RangeBased::Closed(range)
}

pub fn be_in_exclusive_range<T: PartialOrd + Debug>(range: &Range<T>) -> RangeBased<T> {
    RangeBased::HalfOpen(range)
}

#[cfg(test)]
mod tests {
    use crate::assertions::bool::TrueFalse;
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
