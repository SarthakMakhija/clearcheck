use std::ops::{Range, RangeInclusive};

use crate::matchers::equal::be_equal_ignoring_case;
use crate::matchers::range::{be_in_exclusive_range, be_in_inclusive_range};
use crate::matchers::{Should, ShouldNot};

pub trait RangeAssertion {
    fn should_be_in_inclusive_range(&self, range: RangeInclusive<char>) -> &Self;
    fn should_not_be_in_inclusive_range(&self, range: RangeInclusive<char>) -> &Self;
    fn should_be_in_exclusive_range(&self, range: Range<char>) -> &Self;
    fn should_not_be_in_exclusive_range(&self, range: Range<char>) -> &Self;
}

pub trait EqualityAssertion {
    fn should_be_equal_ignoring_case(&self, other: &char) -> &Self;
    fn should_not_be_equal_ignoring_case(&self, other: &char) -> &Self;
}

impl RangeAssertion for char {
    fn should_be_in_inclusive_range(&self, range: RangeInclusive<char>) -> &Self {
        self.should(&be_in_inclusive_range(&range));
        self
    }

    fn should_not_be_in_inclusive_range(&self, range: RangeInclusive<char>) -> &Self {
        self.should_not(&be_in_inclusive_range(&range));
        self
    }

    fn should_be_in_exclusive_range(&self, range: Range<char>) -> &Self {
        self.should(&be_in_exclusive_range(&range));
        self
    }

    fn should_not_be_in_exclusive_range(&self, range: Range<char>) -> &Self {
        self.should_not(&be_in_exclusive_range(&range));
        self
    }
}

impl EqualityAssertion for char {
    fn should_be_equal_ignoring_case(&self, other: &char) -> &Self {
        self.should(&be_equal_ignoring_case(other));
        self
    }

    fn should_not_be_equal_ignoring_case(&self, other: &char) -> &Self {
        self.should_not(&be_equal_ignoring_case(other));
        self
    }
}

#[cfg(test)]
mod range_tests {
    use crate::assertions::char::RangeAssertion;

    #[test]
    fn should_be_in_the_inclusive_range() {
        let letter = 'd';
        letter.should_be_in_inclusive_range('a'..='d');
    }

    #[test]
    #[should_panic]
    fn should_be_in_the_inclusive_range_but_was_not() {
        let letter = 'd';
        letter.should_be_in_inclusive_range('a'..='c');
    }

    #[test]
    fn should_not_be_in_the_inclusive_range() {
        let letter = 'd';
        letter.should_not_be_in_inclusive_range('a'..='c');
    }

    #[test]
    #[should_panic]
    fn should_not_be_in_the_inclusive_range_but_was() {
        let letter = 'd';
        letter.should_not_be_in_inclusive_range('a'..='d');
    }

    #[test]
    fn should_be_in_the_exclusive_range() {
        let letter = 'd';
        letter.should_be_in_exclusive_range('a'..'e');
    }

    #[test]
    #[should_panic]
    fn should_be_in_the_exclusive_range_but_was_not() {
        let letter = 'd';
        letter.should_be_in_exclusive_range('a'..'d');
    }

    #[test]
    fn should_not_be_in_the_exclusive_range() {
        let letter = 'd';
        letter.should_not_be_in_exclusive_range('a'..'d');
    }

    #[test]
    #[should_panic]
    fn should_not_be_in_the_exclusive_range_but_was() {
        let letter = 'd';
        letter.should_not_be_in_exclusive_range('a'..'e');
    }
}

#[cfg(test)]
mod equal_tests {
    use crate::assertions::char::EqualityAssertion;

    #[test]
    fn should_be_equal_ignoring_case() {
        let letter = 'd';
        letter.should_be_equal_ignoring_case(&'D');
    }

    #[test]
    #[should_panic]
    fn should_be_equal_ignoring_case_but_was_not() {
        let letter = 'd';
        letter.should_be_equal_ignoring_case(&'E');
    }

    #[test]
    fn should_not_be_equal_ignoring_case() {
        let letter = 'd';
        letter.should_not_be_equal_ignoring_case(&'E');
    }

    #[test]
    #[should_panic]
    fn should_not_be_equal_ignoring_case_but_was() {
        let letter = 'd';
        letter.should_not_be_equal_ignoring_case(&'D');
    }
}
