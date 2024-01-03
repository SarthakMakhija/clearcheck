use std::ops::{Range, RangeInclusive};

use crate::matchers::Matcher;

pub enum RangeBasedMatcher<'a, T> {
    Closed(&'a RangeInclusive<T>),
    HalfOpen(&'a Range<T>),
}

impl<'a, T> Matcher<T> for RangeBasedMatcher<'a, T>
where
    T: PartialOrd<T>,
{
    fn test(&self, value: &T) -> bool {
        match self {
            RangeBasedMatcher::Closed(range) => range.contains(value),
            RangeBasedMatcher::HalfOpen(range) => range.contains(value),
        }
    }
}

pub fn be_in_inclusive_range<T: PartialOrd>(range: &RangeInclusive<T>) -> RangeBasedMatcher<T> {
    RangeBasedMatcher::Closed(range)
}

pub fn be_in_exclusive_range<T: PartialOrd>(range: &Range<T>) -> RangeBasedMatcher<T> {
    RangeBasedMatcher::HalfOpen(range)
}
