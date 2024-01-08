use std::ops::{Range, RangeInclusive};

use crate::matchers::length::{
    have_atleast_same_length, have_atmost_same_length, have_same_length,
};
use crate::matchers::range::{have_length_in_exclusive_range, have_length_in_inclusive_range};
use crate::matchers::{Should, ShouldNot};

pub trait LengthAssertion {
    fn should_have_length(&self, length: usize) -> &Self;
    fn should_not_have_length(&self, length: usize) -> &Self;
    fn should_have_at_least_length(&self, length: usize) -> &Self;
    fn should_have_at_most_length(&self, length: usize) -> &Self;
    fn should_have_length_in_inclusive_range(&self, range: RangeInclusive<usize>) -> &Self;
    fn should_not_have_length_in_inclusive_range(&self, range: RangeInclusive<usize>) -> &Self;
    fn should_have_length_in_exclusive_range(&self, range: Range<usize>) -> &Self;
    fn should_not_have_length_in_exclusive_range(&self, range: Range<usize>) -> &Self;
}

impl LengthAssertion for String {
    fn should_have_length(&self, length: usize) -> &Self {
        (self as &str).should_have_length(length);
        self
    }

    fn should_not_have_length(&self, length: usize) -> &Self {
        (self as &str).should_not_have_length(length);
        self
    }

    fn should_have_at_least_length(&self, length: usize) -> &Self {
        (self as &str).should_have_at_least_length(length);
        self
    }

    fn should_have_at_most_length(&self, length: usize) -> &Self {
        (self as &str).should_have_at_most_length(length);
        self
    }

    fn should_have_length_in_inclusive_range(&self, range: RangeInclusive<usize>) -> &Self {
        (self as &str).should_have_length_in_inclusive_range(range);
        self
    }

    fn should_not_have_length_in_inclusive_range(&self, range: RangeInclusive<usize>) -> &Self {
        (self as &str).should_not_have_length_in_inclusive_range(range);
        self
    }

    fn should_have_length_in_exclusive_range(&self, range: Range<usize>) -> &Self {
        (self as &str).should_have_length_in_exclusive_range(range);
        self
    }

    fn should_not_have_length_in_exclusive_range(&self, range: Range<usize>) -> &Self {
        (self as &str).should_not_have_length_in_exclusive_range(range);
        self
    }
}

impl LengthAssertion for &str {
    fn should_have_length(&self, length: usize) -> &Self {
        self.should(&have_same_length(length));
        self
    }

    fn should_not_have_length(&self, length: usize) -> &Self {
        self.should_not(&have_same_length(length));
        self
    }

    fn should_have_at_least_length(&self, length: usize) -> &Self {
        self.should(&have_atleast_same_length(length));
        self
    }

    fn should_have_at_most_length(&self, length: usize) -> &Self {
        self.should(&have_atmost_same_length(length));
        self
    }

    fn should_have_length_in_inclusive_range(&self, range: RangeInclusive<usize>) -> &Self {
        self.len().should(&have_length_in_inclusive_range(&range));
        self
    }

    fn should_not_have_length_in_inclusive_range(&self, range: RangeInclusive<usize>) -> &Self {
        self.len()
            .should_not(&have_length_in_inclusive_range(&range));
        self
    }

    fn should_have_length_in_exclusive_range(&self, range: Range<usize>) -> &Self {
        self.len().should(&have_length_in_exclusive_range(&range));
        self
    }

    fn should_not_have_length_in_exclusive_range(&self, range: Range<usize>) -> &Self {
        self.len()
            .should_not(&have_length_in_exclusive_range(&range));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::assertions::string::length::LengthAssertion;

    #[test]
    fn should_have_length_as_8() {
        let name = "assert4j";
        name.should_have_length(8);
    }

    #[test]
    #[should_panic]
    fn should_have_length_as_8_but_was_not() {
        let name = "assert";
        name.should_have_length(8);
    }

    #[test]
    fn should_not_have_length_as_8() {
        let name = "assert";
        name.should_not_have_length(8);
    }

    #[test]
    #[should_panic]
    fn should_not_have_length_as_8_but_was() {
        let name = "assert4j";
        name.should_not_have_length(8);
    }

    #[test]
    fn should_have_at_least_length_as_8() {
        let name = "assert4j";
        name.should_have_at_least_length(8);
    }

    #[test]
    #[should_panic]
    fn should_have_at_least_length_as_8_but_was_not() {
        let name = "assert";
        name.should_have_at_least_length(8);
    }

    #[test]
    fn should_have_at_most_length_as_8() {
        let name = "assert4j";
        name.should_have_at_most_length(8);
    }

    #[test]
    #[should_panic]
    fn should_have_at_most_length_as_6_but_was_not() {
        let name = "assert4j";
        name.should_have_at_most_length(6);
    }

    #[test]
    fn should_have_length_in_the_inclusive_range() {
        let name = "assert4j";
        name.should_have_length_in_inclusive_range(3..=8);
    }

    #[test]
    #[should_panic]
    fn should_have_length_in_the_inclusive_range_but_was_not() {
        let name = "assert4j";
        name.should_have_length_in_inclusive_range(1..=4);
    }

    #[test]
    fn should_not_have_length_in_the_inclusive_range() {
        let name = "assert4j";
        name.should_not_have_length_in_inclusive_range(1..=4);
    }

    #[test]
    #[should_panic]
    fn should_not_have_length_in_the_inclusive_range_but_was() {
        let name = "assert4j";
        name.should_not_have_length_in_inclusive_range(3..=8);
    }

    #[test]
    fn should_have_length_in_the_exclusive_range() {
        let name = "assert4j";
        name.should_have_length_in_exclusive_range(3..9);
    }

    #[test]
    #[should_panic]
    fn should_have_length_in_the_range_but_was_not() {
        let name = "assert4j";
        name.should_have_length_in_exclusive_range(1..8);
    }

    #[test]
    fn should_not_have_length_in_the_exclusive_range() {
        let name = "assert4j";
        name.should_not_have_length_in_exclusive_range(1..4);
    }

    #[test]
    #[should_panic]
    fn should_not_have_length_in_the_exclusive_range_but_was() {
        let name = "assert4j";
        name.should_not_have_length_in_exclusive_range(3..9);
    }
}
