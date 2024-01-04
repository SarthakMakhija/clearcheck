use std::ops::{Range, RangeInclusive};

use crate::matchers::Matcher;

pub enum RangeBased<'a, T> {
    Closed(&'a RangeInclusive<T>),
    HalfOpen(&'a Range<T>),
}

impl<'a, T> Matcher<T> for RangeBased<'a, T>
where
    T: PartialOrd<T>,
{
    fn test(&self, value: &T) -> bool {
        match self {
            RangeBased::Closed(range) => range.contains(value),
            RangeBased::HalfOpen(range) => range.contains(value),
        }
    }
}

pub fn be_in_inclusive_range<T: PartialOrd>(range: &RangeInclusive<T>) -> RangeBased<T> {
    RangeBased::Closed(range)
}

pub fn be_in_exclusive_range<T: PartialOrd>(range: &Range<T>) -> RangeBased<T> {
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
        matcher.test(&2).should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_in_inclusive_range_but_was_not() {
        let matcher = be_in_inclusive_range(&(1..=4));
        matcher.test(&5).should_be_true();
    }

    #[test]
    fn should_be_in_exclusive_range() {
        let matcher = be_in_exclusive_range(&(1..4));
        matcher.test(&2).should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_in_exclusive_range_but_was_not() {
        let matcher = be_in_exclusive_range(&(1..4));
        matcher.test(&4).should_be_true();
    }
}
