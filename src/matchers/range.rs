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
